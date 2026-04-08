//! Base handler trait and core types for the handler system.
//!
//! Every concrete handler implements [`Handler`], which provides a two-phase
//! dispatch: [`Handler::check_update`] tests whether an update is relevant,
//! and [`Handler::handle_update`] processes it.

use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use telegram_bot_raw::types::update::Update;

use crate::context::CallbackContext;

// ---------------------------------------------------------------------------
// Match result
// ---------------------------------------------------------------------------

/// The result of [`Handler::check_update`] when the update *is* relevant.
///
/// Different handlers produce different kinds of match data -- for example
/// a command handler yields the argument list, while a regex handler yields
/// captured groups.
#[derive(Debug)]
pub enum MatchResult {
    /// The handler matched but produced no additional data.
    Empty,
    /// Positional arguments (e.g. text after a `/command`).
    Args(Vec<String>),
    /// Positional-only regex capture groups (no named groups in pattern).
    RegexMatch(Vec<String>),
    /// Regex capture groups for patterns that contain at least one named group.
    ///
    /// `positional` holds every capture (index 0 = full match, 1... = groups),
    /// exactly like `RegexMatch`. `named` maps each named group's name to its
    /// matched value; only groups that actually matched are included.
    RegexMatchWithNames {
        /// All captures in index order (mirrors Python's `match.groups()`).
        positional: Vec<String>,
        /// Named captures (mirrors Python's `match.groupdict()`).
        named: HashMap<String, String>,
    },
    /// Arbitrary handler-specific payload (type-erased).
    Custom(Box<dyn Any + Send>),
}

// ---------------------------------------------------------------------------
// Handler result
// ---------------------------------------------------------------------------

/// The outcome of handling an update.
#[derive(Debug)]
pub enum HandlerResult {
    /// Processing succeeded; continue to next handler group.
    Continue,
    /// Processing succeeded; stop processing further handler groups.
    Stop,
    /// Processing failed with an error.
    Error(Box<dyn std::error::Error + Send + Sync>),
}

// ---------------------------------------------------------------------------
// Callback type alias
// ---------------------------------------------------------------------------

/// A type-erased, `Arc`-wrapped async handler callback.
///
/// The callback receives the [`Update`] and the [`MatchResult`] produced by
/// `check_update`, and returns a future resolving to [`HandlerResult`].
pub type HandlerCallback = Arc<
    dyn Fn(Arc<Update>, MatchResult) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>>
        + Send
        + Sync,
>;

/// A type-erased, `Arc`-wrapped async callback that receives a [`CallbackContext`].
///
/// Used by ergonomic constructors (`CommandHandler::new`, `MessageHandler::new`)
/// where the user function has signature `async fn(Update, Context) -> HandlerResult`.
pub type ContextCallback = Arc<
    dyn Fn(
            Arc<Update>,
            CallbackContext,
        )
            -> Pin<Box<dyn Future<Output = Result<(), crate::application::HandlerError>> + Send>>
        + Send
        + Sync,
>;

// ---------------------------------------------------------------------------
// Handler trait
// ---------------------------------------------------------------------------

/// Core trait that every update handler must implement.
///
/// # Design notes
///
/// * `check_update` is synchronous because it should be a cheap predicate
///   (regex match, field presence check, etc.).
/// * `handle_update` returns a boxed future so that concrete handlers can
///   be stored as trait objects in a heterogeneous handler list.
/// * The default `block()` implementation returns `true`, meaning the
///   application will `await` the future before moving on. Handlers that
///   wish to run concurrently can override this to return `false`.
pub trait Handler: Send + Sync {
    /// Determine whether this handler is interested in `update`.
    ///
    /// Returns `Some(match_result)` if the update should be handled, or
    /// `None` to pass.
    fn check_update(&self, update: &Update) -> Option<MatchResult>;

    /// Process the update. Called only when [`check_update`](Handler::check_update)
    /// returned `Some`.
    fn handle_update(
        &self,
        update: Arc<Update>,
        match_result: MatchResult,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>>;

    /// Whether the application should block on this handler's future before
    /// dispatching to the next handler group.
    fn block(&self) -> bool {
        true
    }

    /// Populate additional context fields (e.g. `context.args`, `context.matches`)
    /// from the match result before the handler callback is invoked.
    ///
    /// The default implementation is a no-op. Handlers should override this
    /// to inject their match-specific data into the context.
    fn collect_additional_context(
        &self,
        _context: &mut CallbackContext,
        _match_result: &MatchResult,
    ) {
        // Default: no-op
    }

    /// Process the update with an Application-provided [`CallbackContext`].
    ///
    /// The default implementation ignores the context and delegates to
    /// [`handle_update`](Handler::handle_update). Handlers created with
    /// ergonomic constructors (e.g. `CommandHandler::new("start", my_fn)`)
    /// override this to pass the context to the user's callback function.
    fn handle_update_with_context(
        &self,
        update: Arc<Update>,
        match_result: MatchResult,
        _context: CallbackContext,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> {
        self.handle_update(update, match_result)
    }
}

// ---------------------------------------------------------------------------
// FnHandler -- generic function-based handler
// ---------------------------------------------------------------------------

/// A lightweight handler that pairs a predicate with an async callback.
///
/// `FnHandler` bridges the gap between raw `application::Handler` structs and
/// the typed handler trait system. It implements [`Handler`] so it can be
/// registered via [`Application::add_typed_handler`].
///
/// # Examples
///
/// ```rust,ignore
/// use telegram_bot::ext::prelude::*;
/// use telegram_bot::ext::handlers::base::FnHandler;
///
/// async fn button_handler(update: Update, context: Context) -> HandlerResult {
///     // handle callback query ...
///     Ok(())
/// }
///
/// // Register with a predicate:
/// app.add_typed_handler(
///     FnHandler::new(|u| u.callback_query().is_some(), button_handler),
///     0,
/// ).await;
///
/// // Or use a convenience constructor:
/// app.add_typed_handler(FnHandler::on_callback_query(button_handler), 0).await;
/// ```
pub struct FnHandler {
    check: Arc<dyn Fn(&Update) -> bool + Send + Sync>,
    context_callback: ContextCallback,
}

impl FnHandler {
    /// Create a new `FnHandler` with a custom predicate and an async callback.
    ///
    /// The callback receives `(Update, CallbackContext)` and returns
    /// `Result<(), HandlerError>`.
    pub fn new<C, Cb, Fut>(check: C, callback: Cb) -> Self
    where
        C: Fn(&Update) -> bool + Send + Sync + 'static,
        Cb: Fn(Arc<Update>, CallbackContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), crate::application::HandlerError>> + Send + 'static,
    {
        let cb = Arc::new(callback);
        let context_cb: ContextCallback = Arc::new(move |update, ctx| {
            let fut = cb(update, ctx);
            Box::pin(fut)
                as Pin<
                    Box<dyn Future<Output = Result<(), crate::application::HandlerError>> + Send>,
                >
        });
        Self {
            check: Arc::new(check),
            context_callback: context_cb,
        }
    }

    /// Match updates that have a `callback_query`.
    pub fn on_callback_query<Cb, Fut>(callback: Cb) -> Self
    where
        Cb: Fn(Arc<Update>, CallbackContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), crate::application::HandlerError>> + Send + 'static,
    {
        Self::new(|u| u.callback_query().is_some(), callback)
    }

    /// Match updates that have an `inline_query`.
    pub fn on_inline_query<Cb, Fut>(callback: Cb) -> Self
    where
        Cb: Fn(Arc<Update>, CallbackContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), crate::application::HandlerError>> + Send + 'static,
    {
        Self::new(|u| u.inline_query().is_some(), callback)
    }

    /// Match updates that have a `poll`.
    pub fn on_poll<Cb, Fut>(callback: Cb) -> Self
    where
        Cb: Fn(Arc<Update>, CallbackContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), crate::application::HandlerError>> + Send + 'static,
    {
        Self::new(|u| u.poll().is_some(), callback)
    }

    /// Match updates that have a `poll_answer`.
    pub fn on_poll_answer<Cb, Fut>(callback: Cb) -> Self
    where
        Cb: Fn(Arc<Update>, CallbackContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), crate::application::HandlerError>> + Send + 'static,
    {
        Self::new(|u| u.poll_answer().is_some(), callback)
    }

    /// Match updates that have a `shipping_query`.
    pub fn on_shipping_query<Cb, Fut>(callback: Cb) -> Self
    where
        Cb: Fn(Arc<Update>, CallbackContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), crate::application::HandlerError>> + Send + 'static,
    {
        Self::new(|u| u.shipping_query().is_some(), callback)
    }

    /// Match updates that have a `pre_checkout_query`.
    pub fn on_pre_checkout_query<Cb, Fut>(callback: Cb) -> Self
    where
        Cb: Fn(Arc<Update>, CallbackContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), crate::application::HandlerError>> + Send + 'static,
    {
        Self::new(|u| u.pre_checkout_query().is_some(), callback)
    }

    /// Match updates that have a `chat_member`.
    pub fn on_chat_member<Cb, Fut>(callback: Cb) -> Self
    where
        Cb: Fn(Arc<Update>, CallbackContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), crate::application::HandlerError>> + Send + 'static,
    {
        Self::new(|u| u.chat_member().is_some(), callback)
    }

    /// Match updates that have a `my_chat_member`.
    pub fn on_my_chat_member<Cb, Fut>(callback: Cb) -> Self
    where
        Cb: Fn(Arc<Update>, CallbackContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), crate::application::HandlerError>> + Send + 'static,
    {
        Self::new(|u| u.my_chat_member().is_some(), callback)
    }

    /// Match updates that have a `message`.
    pub fn on_message<Cb, Fut>(callback: Cb) -> Self
    where
        Cb: Fn(Arc<Update>, CallbackContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), crate::application::HandlerError>> + Send + 'static,
    {
        Self::new(|u| u.message().is_some(), callback)
    }

    /// Match every update (catch-all).
    pub fn on_any<Cb, Fut>(callback: Cb) -> Self
    where
        Cb: Fn(Arc<Update>, CallbackContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), crate::application::HandlerError>> + Send + 'static,
    {
        Self::new(|_| true, callback)
    }
}

impl Handler for FnHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        if (self.check)(update) {
            Some(MatchResult::Empty)
        } else {
            None
        }
    }

    fn handle_update(
        &self,
        _update: Arc<Update>,
        _match_result: MatchResult,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> {
        // FnHandler always uses handle_update_with_context; this is a no-op fallback.
        Box::pin(async { HandlerResult::Continue })
    }

    fn handle_update_with_context(
        &self,
        update: Arc<Update>,
        _match_result: MatchResult,
        context: CallbackContext,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> {
        let fut = (self.context_callback)(update, context);
        Box::pin(async move {
            match fut.await {
                Ok(()) => HandlerResult::Continue,
                Err(crate::application::HandlerError::HandlerStop { .. }) => HandlerResult::Stop,
                Err(crate::application::HandlerError::Other(e)) => HandlerResult::Error(e),
            }
        })
    }
}
