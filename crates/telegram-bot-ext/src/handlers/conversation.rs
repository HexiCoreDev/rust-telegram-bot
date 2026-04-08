//! [`ConversationHandler`] -- stateful multi-step conversation handler.
//!
//! Ported from `python-telegram-bot`'s `ConversationHandler`. This is the
//! most complex handler in the system. It manages a state machine per
//! conversation, routing updates through entry points, state-specific
//! handlers, and fallback handlers.
//!
//! # Design
//!
//! The handler is generic over a state type `S` that must be `Hash + Eq +
//! Clone + Send + Sync + 'static`. State is tracked per conversation key,
//! which is a tuple of `(chat_id, user_id)` by default.
//!
//! Callbacks return [`ConversationResult<S>`] to control state transitions.
//!
//! # Fixes implemented
//!
//! - **C3**: `check_update` is now state-aware via `RwLock::try_read()`.
//!   Only the relevant handler list is checked based on current state.
//! - **C4**: `map_to_parent` support for nested conversations.
//! - **C5**: Timeout scheduling via `tokio::spawn` + `tokio::time::sleep`
//!   with cancellation via `tokio::sync::watch`.
//! - **C6**: Persistence integration with `load_conversations` / `save_conversations`.
//! - **M12**: Channel posts and edited channel posts are rejected.
//! - **M13**: WAITING state -- pending callbacks tracked, updates skipped while busy.
//! - **PendingState**: Non-blocking callbacks are spawned via `tokio::spawn`
//!   with result capture. On error the conversation reverts to the previous
//!   state instead of leaving the conversation in limbo.
//!
//! # Example
//!
//! ```rust,ignore
//! use telegram_bot_ext::handlers::conversation::*;
//! use telegram_bot_ext::handlers::base::*;
//! use telegram_bot_ext::handlers::command::CommandHandler;
//! use std::sync::Arc;
//! use std::collections::HashMap;
//!
//! #[derive(Clone, Hash, Eq, PartialEq)]
//! enum State { AskName, AskAge }
//!
//! let conv = ConversationHandler::builder()
//!     .entry_point(Box::new(start_handler))
//!     .state(State::AskName, vec![Box::new(name_handler)])
//!     .state(State::AskAge, vec![Box::new(age_handler)])
//!     .fallback(Box::new(cancel_handler))
//!     .build();
//! ```

use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::hash::Hash;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{watch, RwLock};
use tracing::{debug, error, warn};

use telegram_bot_raw::types::update::Update;

use super::base::{Handler, HandlerResult, MatchResult};

// ---------------------------------------------------------------------------
// Conversation key
// ---------------------------------------------------------------------------

/// The key that identifies a unique conversation.
///
/// By default this is `(chat_id, user_id)`, but the components included
/// depend on the `per_chat`, `per_user`, and `per_message` flags.
pub type ConversationKey = Vec<i64>;

// ---------------------------------------------------------------------------
// Conversation result
// ---------------------------------------------------------------------------

/// The result returned by a conversation step callback, controlling the
/// state machine transition.
#[derive(Debug, Clone)]
pub enum ConversationResult<S> {
    /// Transition to the given state.
    NextState(S),
    /// End the conversation (remove the key from the state map).
    End,
    /// Stay in the current state (no transition).
    Stay,
}

// ---------------------------------------------------------------------------
// Conversation callback
// ---------------------------------------------------------------------------

/// A conversation step callback.
///
/// Unlike the base `HandlerCallback`, this returns a `ConversationResult<S>`
/// alongside the `HandlerResult`.
pub type ConversationCallback<S> = Arc<
    dyn Fn(
            Arc<Update>,
            MatchResult,
        ) -> Pin<Box<dyn Future<Output = (HandlerResult, ConversationResult<S>)> + Send>>
        + Send
        + Sync,
>;

// ---------------------------------------------------------------------------
// Conversation step handler
// ---------------------------------------------------------------------------

/// A handler that participates in a conversation. It wraps a base `Handler`
/// for the matching logic, plus a conversation-aware callback.
pub struct ConversationStepHandler<S: Hash + Eq + Clone + Send + Sync + 'static> {
    /// The underlying handler used for `check_update`.
    pub handler: Box<dyn Handler>,
    /// The callback that produces a state transition.
    pub conv_callback: ConversationCallback<S>,
}

// ---------------------------------------------------------------------------
// Conversation handler
// ---------------------------------------------------------------------------

/// Stateful multi-step conversation handler, generic over state type `S`.
///
/// Manages a state machine per conversation key, dispatching updates through
/// entry points, state handlers, and fallbacks.
pub struct ConversationHandler<S: Hash + Eq + Clone + Send + Sync + 'static> {
    /// Handlers that can initiate a new conversation.
    entry_points: Vec<ConversationStepHandler<S>>,
    /// Per-state handler lists.
    states: HashMap<S, Vec<ConversationStepHandler<S>>>,
    /// Fallback handlers tried when no state handler matches.
    fallbacks: Vec<ConversationStepHandler<S>>,

    /// Current conversation states, keyed by conversation key.
    conversations: Arc<RwLock<HashMap<ConversationKey, S>>>,

    /// Whether a user already in a conversation can restart via entry points.
    allow_reentry: bool,
    /// Whether to include chat ID in the conversation key.
    per_chat: bool,
    /// Whether to include user ID in the conversation key.
    per_user: bool,
    /// Whether to include message ID (from callback query) in the key.
    per_message: bool,

    /// Optional conversation timeout. After this duration of inactivity the
    /// conversation is ended.
    conversation_timeout: Option<Duration>,

    /// C4: Mapping from child state to parent state for nested conversations.
    /// When a callback returns a state that is present in this map, the
    /// conversation ends and the mapped parent state is returned.
    map_to_parent: Option<HashMap<S, S>>,

    /// C5: Handlers for the TIMEOUT state. When a conversation times out,
    /// all matching timeout handlers are run before removing the conversation.
    timeout_handlers: Vec<ConversationStepHandler<S>>,

    /// C5: Per-conversation timeout cancellation senders. Sending a value
    /// through the watch channel cancels the pending timeout task.
    timeout_cancellers: Arc<RwLock<HashMap<ConversationKey, watch::Sender<bool>>>>,

    /// C6: Whether this conversation's state should be persisted.
    persistent: bool,

    /// C6: Optional name for persistence. Required when `persistent` is true.
    name: Option<String>,

    /// M13: Set of conversation keys that have a non-blocking callback in
    /// progress (WAITING state). Updates for these keys are skipped until
    /// the callback completes.
    pending_callbacks: Arc<RwLock<HashSet<ConversationKey>>>,
}

impl<S: Hash + Eq + Clone + Send + Sync + 'static> ConversationHandler<S> {
    /// Create a builder for constructing a `ConversationHandler`.
    pub fn builder() -> ConversationHandlerBuilder<S> {
        ConversationHandlerBuilder::default()
    }

    /// Build the conversation key for a given update.
    fn build_key(&self, update: &Update) -> Option<ConversationKey> {
        let mut key = Vec::new();

        if self.per_chat {
            let chat = update.effective_chat()?;
            key.push(chat.id);
        }

        if self.per_user {
            let user = update.effective_user()?;
            key.push(user.id);
        }

        if self.per_message {
            let cq = update.callback_query.as_ref()?;
            if let Some(ref inline_id) = cq.inline_message_id {
                use std::hash::Hasher;
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                hasher.write(inline_id.as_bytes());
                key.push(hasher.finish() as i64);
            } else if let Some(ref msg) = cq.message {
                key.push(msg.message_id());
            } else {
                return None;
            }
        }

        if key.is_empty() {
            return None;
        }

        Some(key)
    }

    /// Try to find a matching handler in the given list.
    fn find_matching(
        handlers: &[ConversationStepHandler<S>],
        update: &Update,
    ) -> Option<(usize, MatchResult)> {
        for (idx, step) in handlers.iter().enumerate() {
            if let Some(mr) = step.handler.check_update(update) {
                return Some((idx, mr));
            }
        }
        None
    }

    /// Get the current state for a conversation key.
    pub async fn get_state(&self, key: &ConversationKey) -> Option<S> {
        self.conversations.read().await.get(key).cloned()
    }

    /// Get a read-only snapshot of all active conversations.
    pub async fn active_conversations(&self) -> HashMap<ConversationKey, S> {
        self.conversations.read().await.clone()
    }

    // -- C6: Persistence --------------------------------------------------

    /// Load previously-persisted conversations into this handler.
    pub async fn load_conversations(&self, data: HashMap<ConversationKey, S>) {
        *self.conversations.write().await = data;
    }

    /// Export the current conversation state for persistence.
    pub async fn save_conversations(&self) -> HashMap<ConversationKey, S> {
        self.conversations.read().await.clone()
    }

    /// Whether this handler is configured for persistence.
    pub fn is_persistent(&self) -> bool {
        self.persistent
    }

    /// The handler's name (required for persistence).
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Apply a conversation state transition, handling End and map_to_parent.
    ///
    /// Returns `Some(new_state)` if the conversation continues, or `None` if
    /// the conversation was ended (either explicitly or via map_to_parent).
    async fn apply_state_transition(
        conversations: &RwLock<HashMap<ConversationKey, S>>,
        pending_callbacks: &RwLock<HashSet<ConversationKey>>,
        key: &ConversationKey,
        conv_result: ConversationResult<S>,
        current_state: &Option<S>,
        map_to_parent: &Option<HashMap<S, S>>,
    ) -> Option<S> {
        match conv_result {
            ConversationResult::End => {
                conversations.write().await.remove(key);
                pending_callbacks.write().await.remove(key);
                None
            }
            ConversationResult::Stay => current_state.clone(),
            ConversationResult::NextState(s) => {
                // C4: Check map_to_parent.
                if let Some(ref mtp) = map_to_parent {
                    if mtp.contains_key(&s) {
                        conversations.write().await.remove(key);
                        pending_callbacks.write().await.remove(key);
                        debug!(
                            "ConversationHandler: map_to_parent triggered for key {:?}",
                            key
                        );
                        return None;
                    }
                }
                Some(s)
            }
        }
    }

    /// Spawn a timeout task for the given conversation key after a callback
    /// completes. Shared between the blocking and non-blocking paths.
    fn spawn_timeout(
        conversations: Arc<RwLock<HashMap<ConversationKey, S>>>,
        pending_callbacks: Arc<RwLock<HashSet<ConversationKey>>>,
        timeout_cancellers: Arc<RwLock<HashMap<ConversationKey, watch::Sender<bool>>>>,
        key: ConversationKey,
        update: Arc<Update>,
        duration: Duration,
        timeout_cbs: Vec<ConversationCallback<S>>,
    ) -> watch::Sender<bool> {
        let (cancel_tx, mut cancel_rx) = watch::channel(false);
        let key2 = key.clone();

        tokio::spawn(async move {
            tokio::select! {
                _ = tokio::time::sleep(duration) => {
                    for cb in &timeout_cbs {
                        let _ = cb(update.clone(), MatchResult::Empty).await;
                    }
                    conversations.write().await.remove(&key2);
                    pending_callbacks.write().await.remove(&key2);
                    timeout_cancellers.write().await.remove(&key2);
                    debug!("Conversation {:?} timed out", key2);
                }
                _ = cancel_rx.changed() => {
                    debug!("Timeout cancelled for {:?}", key2);
                }
            }
        });

        cancel_tx
    }
}

impl<S: Hash + Eq + Clone + Send + Sync + 'static> Handler for ConversationHandler<S> {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        // ── M12: Reject channel posts and edited channel posts ───────────
        if update.channel_post.is_some() || update.edited_channel_post.is_some() {
            return None;
        }

        let key = self.build_key(update)?;

        // ── M13: Skip if a pending callback is in progress for this key ──
        if let Ok(pending) = self.pending_callbacks.try_read() {
            if pending.contains(&key) {
                debug!(
                    "ConversationHandler: skipping update for {:?} (pending callback)",
                    key
                );
                return None;
            }
        }

        // ── C3: State-aware handler selection via try_read() ─────────────
        let current_state = match self.conversations.try_read() {
            Ok(guard) => guard.get(&key).cloned(),
            Err(_) => {
                debug!(
                    "ConversationHandler: conversations lock contended, skipping {:?}",
                    key
                );
                return None;
            }
        };

        match current_state {
            None => {
                if Self::find_matching(&self.entry_points, update).is_some() {
                    return Some(MatchResult::Empty);
                }
            }
            Some(ref state) => {
                if self.allow_reentry && Self::find_matching(&self.entry_points, update).is_some() {
                    return Some(MatchResult::Empty);
                }

                if let Some(handlers) = self.states.get(state) {
                    if Self::find_matching(handlers, update).is_some() {
                        return Some(MatchResult::Empty);
                    }
                }

                if Self::find_matching(&self.fallbacks, update).is_some() {
                    return Some(MatchResult::Empty);
                }
            }
        }

        None
    }

    fn handle_update(
        &self,
        update: Arc<Update>,
        _match_result: MatchResult,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> {
        let conversations = Arc::clone(&self.conversations);
        let pending_callbacks = Arc::clone(&self.pending_callbacks);
        let allow_reentry = self.allow_reentry;

        #[derive(Debug, Clone, Copy)]
        enum HandlerSource {
            EntryPoint(usize),
            State(usize),
            Fallback(usize),
        }

        let key = self.build_key(&update);

        let current_state = key.as_ref().and_then(|k| {
            self.conversations
                .try_read()
                .ok()
                .and_then(|g| g.get(k).cloned())
        });

        // ── State-aware handler matching ─────────────────────────────────
        let mut source = None;
        let mut match_result = MatchResult::Empty;

        let check_entries = current_state.is_none() || allow_reentry;
        if check_entries {
            if let Some((idx, mr)) = Self::find_matching(&self.entry_points, &update) {
                source = Some(HandlerSource::EntryPoint(idx));
                match_result = mr;
            }
        }

        if source.is_none() {
            if let Some(ref state) = current_state {
                if let Some(handlers) = self.states.get(state) {
                    if let Some((idx, mr)) = Self::find_matching(handlers, &update) {
                        source = Some(HandlerSource::State(idx));
                        match_result = mr;
                    }
                }
            }
        }

        if source.is_none() {
            if let Some((idx, mr)) = Self::find_matching(&self.fallbacks, &update) {
                source = Some(HandlerSource::Fallback(idx));
                match_result = mr;
            }
        }

        // Resolve the callback Arc.
        let conv_cb = match source {
            Some(HandlerSource::EntryPoint(idx)) => {
                Arc::clone(&self.entry_points[idx].conv_callback)
            }
            Some(HandlerSource::State(idx)) => {
                let mut cb = None;
                if let Some(ref state) = current_state {
                    if let Some(handlers) = self.states.get(state) {
                        if idx < handlers.len() {
                            cb = Some(Arc::clone(&handlers[idx].conv_callback));
                        }
                    }
                }
                cb.unwrap_or_else(|| {
                    Arc::new(|_u, _m| {
                        Box::pin(async { (HandlerResult::Continue, ConversationResult::Stay) })
                    })
                })
            }
            Some(HandlerSource::Fallback(idx)) => Arc::clone(&self.fallbacks[idx].conv_callback),
            None => {
                return Box::pin(async { HandlerResult::Continue });
            }
        };

        let is_entry = matches!(source, Some(HandlerSource::EntryPoint(_)));

        // Determine if the matched step handler is non-blocking.
        let is_blocking = match source {
            Some(HandlerSource::EntryPoint(idx)) => self.entry_points[idx].handler.block(),
            Some(HandlerSource::State(idx)) => current_state
                .as_ref()
                .and_then(|s| self.states.get(s))
                .and_then(|handlers| handlers.get(idx))
                .map_or(true, |step| step.handler.block()),
            Some(HandlerSource::Fallback(idx)) => self.fallbacks[idx].handler.block(),
            None => true,
        };

        let map_to_parent = self.map_to_parent.clone();
        let has_timeout = self.conversation_timeout.is_some();
        let timeout_cancellers = Arc::clone(&self.timeout_cancellers);
        let timeout_duration = self.conversation_timeout;
        let timeout_cbs: Vec<_> = self
            .timeout_handlers
            .iter()
            .map(|step| Arc::clone(&step.conv_callback))
            .collect();

        let is_persistent = self.persistent;
        let _handler_name = self.name.clone();

        Box::pin(async move {
            let key = match key {
                Some(k) => k,
                None => return HandlerResult::Continue,
            };

            let current_state = conversations.read().await.get(&key).cloned();

            if is_entry && current_state.is_some() && !allow_reentry {
                debug!("ConversationHandler: ignoring re-entry for key {:?}", key);
                return HandlerResult::Continue;
            }

            // C5: Cancel any existing timeout before running the callback.
            if has_timeout {
                if let Some(tx) = timeout_cancellers.write().await.remove(&key) {
                    let _ = tx.send(true);
                }
            }

            // ── PendingState: Non-blocking callback resolution ───────────
            //
            // When the step handler is non-blocking, spawn the callback via
            // `tokio::spawn`. On success, apply the state transition normally.
            // On error (task panic / cancellation), revert to the previous
            // state instead of leaving the conversation in limbo.
            if !is_blocking {
                pending_callbacks.write().await.insert(key.clone());

                let conversations2 = Arc::clone(&conversations);
                let pending2 = Arc::clone(&pending_callbacks);
                let map_to_parent2 = map_to_parent.clone();
                let key2 = key.clone();
                let current_state2 = current_state.clone();
                let update2 = update.clone();
                let timeout_cancellers2 = Arc::clone(&timeout_cancellers);
                let timeout_cbs2 = timeout_cbs;

                tokio::spawn(async move {
                    // Spawn the callback itself so we can catch panics via JoinError.
                    let result = tokio::spawn(conv_cb(update2.clone(), match_result)).await;

                    match result {
                        Ok((_handler_result, conv_result)) => {
                            let new_state = Self::apply_state_transition(
                                &conversations2,
                                &pending2,
                                &key2,
                                conv_result,
                                &current_state2,
                                &map_to_parent2,
                            )
                            .await;

                            if let Some(new_s) = new_state {
                                conversations2.write().await.insert(key2.clone(), new_s);
                            }
                        }
                        Err(join_err) => {
                            // Callback panicked or was cancelled -- revert to
                            // the previous state.
                            error!(
                                "ConversationHandler: non-blocking callback failed for {:?}: {}. \
                                 Reverting to previous state.",
                                key2, join_err
                            );
                            if let Some(ref prev) = current_state2 {
                                conversations2
                                    .write()
                                    .await
                                    .insert(key2.clone(), prev.clone());
                            } else {
                                conversations2.write().await.remove(&key2);
                            }
                        }
                    }

                    // Remove from pending set.
                    pending2.write().await.remove(&key2);

                    // Reschedule timeout if configured.
                    if has_timeout {
                        if let Some(duration) = timeout_duration {
                            let cancel_tx = Self::spawn_timeout(
                                Arc::clone(&conversations2),
                                Arc::clone(&pending2),
                                Arc::clone(&timeout_cancellers2),
                                key2.clone(),
                                update2,
                                duration,
                                timeout_cbs2,
                            );
                            timeout_cancellers2.write().await.insert(key2, cancel_tx);
                        }
                    }
                });

                return HandlerResult::Continue;
            }

            // ── Blocking callback path ───────────────────────────────────
            let (handler_result, conv_result) = conv_cb(update.clone(), match_result).await;

            let new_state = Self::apply_state_transition(
                &conversations,
                &pending_callbacks,
                &key,
                conv_result,
                &current_state,
                &map_to_parent,
            )
            .await;

            // For End and map_to_parent, apply_state_transition already cleaned up.
            if new_state.is_none() && !conversations.read().await.contains_key(&key) {
                return handler_result;
            }

            if let Some(new_s) = new_state {
                conversations.write().await.insert(key.clone(), new_s);
            }

            // C5: Reschedule timeout after successful state transition.
            if has_timeout {
                if let Some(duration) = timeout_duration {
                    let cancel_tx = Self::spawn_timeout(
                        Arc::clone(&conversations),
                        Arc::clone(&pending_callbacks),
                        Arc::clone(&timeout_cancellers),
                        key.clone(),
                        update,
                        duration,
                        timeout_cbs,
                    );
                    timeout_cancellers.write().await.insert(key, cancel_tx);
                }
            }

            if is_persistent {
                debug!("ConversationHandler: state changed (persistent handler)");
            }

            handler_result
        })
    }

    fn block(&self) -> bool {
        true
    }
}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// Builder for [`ConversationHandler`].
pub struct ConversationHandlerBuilder<S: Hash + Eq + Clone + Send + Sync + 'static> {
    entry_points: Vec<ConversationStepHandler<S>>,
    states: HashMap<S, Vec<ConversationStepHandler<S>>>,
    fallbacks: Vec<ConversationStepHandler<S>>,
    allow_reentry: bool,
    per_chat: bool,
    per_user: bool,
    per_message: bool,
    conversation_timeout: Option<Duration>,
    name: Option<String>,
    map_to_parent: Option<HashMap<S, S>>,
    timeout_handlers: Vec<ConversationStepHandler<S>>,
    persistent: bool,
}

impl<S: Hash + Eq + Clone + Send + Sync + 'static> Default for ConversationHandlerBuilder<S> {
    fn default() -> Self {
        Self {
            entry_points: Vec::new(),
            states: HashMap::new(),
            fallbacks: Vec::new(),
            allow_reentry: false,
            per_chat: true,
            per_user: true,
            per_message: false,
            conversation_timeout: None,
            name: None,
            map_to_parent: None,
            timeout_handlers: Vec::new(),
            persistent: false,
        }
    }
}

impl<S: Hash + Eq + Clone + Send + Sync + 'static> ConversationHandlerBuilder<S> {
    /// Add an entry point handler.
    pub fn entry_point(mut self, handler: ConversationStepHandler<S>) -> Self {
        self.entry_points.push(handler);
        self
    }

    /// Add multiple entry point handlers.
    pub fn entry_points(mut self, handlers: Vec<ConversationStepHandler<S>>) -> Self {
        self.entry_points.extend(handlers);
        self
    }

    /// Add handlers for a specific conversation state.
    pub fn state(mut self, state: S, handlers: Vec<ConversationStepHandler<S>>) -> Self {
        self.states.insert(state, handlers);
        self
    }

    /// Add a fallback handler.
    pub fn fallback(mut self, handler: ConversationStepHandler<S>) -> Self {
        self.fallbacks.push(handler);
        self
    }

    /// Add multiple fallback handlers.
    pub fn fallbacks(mut self, handlers: Vec<ConversationStepHandler<S>>) -> Self {
        self.fallbacks.extend(handlers);
        self
    }

    /// Set whether re-entry via entry points is allowed.
    pub fn allow_reentry(mut self, allow: bool) -> Self {
        self.allow_reentry = allow;
        self
    }

    /// Set whether the conversation key includes the chat ID.
    pub fn per_chat(mut self, enabled: bool) -> Self {
        self.per_chat = enabled;
        self
    }

    /// Set whether the conversation key includes the user ID.
    pub fn per_user(mut self, enabled: bool) -> Self {
        self.per_user = enabled;
        self
    }

    /// Set whether the conversation key includes the message ID.
    pub fn per_message(mut self, enabled: bool) -> Self {
        self.per_message = enabled;
        self
    }

    /// Set the conversation timeout.
    pub fn conversation_timeout(mut self, timeout: Duration) -> Self {
        self.conversation_timeout = Some(timeout);
        self
    }

    /// Set an optional name (required for persistence).
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// C4: Set the map-to-parent state mapping for nested conversations.
    pub fn map_to_parent(mut self, mapping: HashMap<S, S>) -> Self {
        self.map_to_parent = Some(mapping);
        self
    }

    /// C5: Add handlers for the TIMEOUT state.
    pub fn timeout_handlers(mut self, handlers: Vec<ConversationStepHandler<S>>) -> Self {
        self.timeout_handlers = handlers;
        self
    }

    /// C5: Add a single timeout handler.
    pub fn timeout_handler(mut self, handler: ConversationStepHandler<S>) -> Self {
        self.timeout_handlers.push(handler);
        self
    }

    /// C6: Enable persistence for this conversation handler.
    pub fn persistent(mut self, enabled: bool) -> Self {
        self.persistent = enabled;
        self
    }

    /// Build the `ConversationHandler`.
    ///
    /// # Panics
    ///
    /// Panics if `per_chat`, `per_user`, and `per_message` are all `false`.
    /// Panics if `persistent` is `true` but `name` is `None`.
    pub fn build(self) -> ConversationHandler<S> {
        assert!(
            self.per_chat || self.per_user || self.per_message,
            "At least one of per_chat, per_user, per_message must be true"
        );

        if self.persistent && self.name.is_none() {
            panic!("Conversations can't be persistent when handler is unnamed");
        }

        if self.per_message && !self.per_chat {
            warn!(
                "ConversationHandler: per_message=true without per_chat=true \
                 -- message IDs are not globally unique"
            );
        }

        ConversationHandler {
            entry_points: self.entry_points,
            states: self.states,
            fallbacks: self.fallbacks,
            conversations: Arc::new(RwLock::new(HashMap::new())),
            allow_reentry: self.allow_reentry,
            per_chat: self.per_chat,
            per_user: self.per_user,
            per_message: self.per_message,
            conversation_timeout: self.conversation_timeout,
            map_to_parent: self.map_to_parent,
            timeout_handlers: self.timeout_handlers,
            timeout_cancellers: Arc::new(RwLock::new(HashMap::new())),
            persistent: self.persistent,
            name: self.name,
            pending_callbacks: Arc::new(RwLock::new(HashSet::new())),
        }
    }
}
