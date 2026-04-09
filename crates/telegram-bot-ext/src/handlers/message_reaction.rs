//! [`MessageReactionHandler`] -- handles message reaction updates.
//!
//! Ported from `python-telegram-bot`'s `MessageReactionHandler`. Supports
//! filtering by reaction type (`message_reaction`, `message_reaction_count`,
//! or both) and by chat/user ID and username.

use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use rust_tg_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// Which kind of message reaction update to handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageReactionType {
    /// Only `message_reaction` (individual user reaction).
    Updated,
    /// Only `message_reaction_count` (anonymous reaction count).
    CountUpdated,
    /// Both reaction types.
    Any,
}

/// Handler for `Update.message_reaction` and `Update.message_reaction_count`.
///
/// # Filtering
///
/// When no ID/username filters are provided, all matching reaction updates
/// are accepted. Chat and user filters are OR-combined: if any of the
/// provided chat IDs, chat usernames, user IDs, or user usernames matches,
/// the update is accepted.
///
/// Note: user filtering is only valid with `MessageReactionType::Updated`
/// because anonymous reactions have no user information.
pub struct MessageReactionHandler {
    callback: HandlerCallback,
    reaction_type: MessageReactionType,
    chat_ids: HashSet<i64>,
    chat_usernames: HashSet<String>,
    user_ids: HashSet<i64>,
    user_usernames: HashSet<String>,
    block: bool,
}

impl MessageReactionHandler {
    /// Create a new `MessageReactionHandler`.
    ///
    /// # Panics
    ///
    /// Panics if user filters are provided together with a `reaction_type`
    /// that includes anonymous reactions (`Any` or `CountUpdated`).
    pub fn new(
        callback: HandlerCallback,
        reaction_type: MessageReactionType,
        chat_ids: HashSet<i64>,
        chat_usernames: HashSet<String>,
        user_ids: HashSet<i64>,
        user_usernames: HashSet<String>,
        block: bool,
    ) -> Self {
        let has_user_filter = !user_ids.is_empty() || !user_usernames.is_empty();
        let includes_anonymous = matches!(
            reaction_type,
            MessageReactionType::Any | MessageReactionType::CountUpdated
        );
        assert!(
            !(has_user_filter && includes_anonymous),
            "Cannot filter by user and include anonymous reactions. \
             Set reaction_type to MessageReactionType::Updated."
        );

        let chat_usernames = chat_usernames
            .into_iter()
            .map(|u| u.trim_start_matches('@').to_lowercase())
            .collect();
        let user_usernames = user_usernames
            .into_iter()
            .map(|u| u.trim_start_matches('@').to_lowercase())
            .collect();

        Self {
            callback,
            reaction_type,
            chat_ids,
            chat_usernames,
            user_ids,
            user_usernames,
            block,
        }
    }
}

impl Handler for MessageReactionHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        let has_reaction = update.message_reaction().is_some();
        let has_count = update.message_reaction_count().is_some();

        if !has_reaction && !has_count {
            return None;
        }

        // Type filter
        match self.reaction_type {
            MessageReactionType::Updated => {
                if has_count && !has_reaction {
                    return None;
                }
            }
            MessageReactionType::CountUpdated => {
                if has_reaction && !has_count {
                    return None;
                }
            }
            MessageReactionType::Any => {}
        }

        // No filters -> accept.
        let no_filters = self.chat_ids.is_empty()
            && self.chat_usernames.is_empty()
            && self.user_ids.is_empty()
            && self.user_usernames.is_empty();
        if no_filters {
            return Some(MatchResult::Empty);
        }

        // Check chat.
        if let Some(chat) = update.effective_chat() {
            if self.chat_ids.contains(&chat.id) {
                return Some(MatchResult::Empty);
            }
            if let Some(ref uname) = chat.username {
                if self.chat_usernames.contains(&uname.to_lowercase()) {
                    return Some(MatchResult::Empty);
                }
            }
        }

        // Check user.
        if let Some(user) = update.effective_user() {
            if self.user_ids.contains(&user.id) {
                return Some(MatchResult::Empty);
            }
            if let Some(ref uname) = user.username {
                if self.user_usernames.contains(&uname.to_lowercase()) {
                    return Some(MatchResult::Empty);
                }
            }
        }

        None
    }

    fn handle_update(
        &self,
        update: Arc<Update>,
        match_result: MatchResult,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> {
        (self.callback)(update, match_result)
    }

    fn block(&self) -> bool {
        self.block
    }
}
