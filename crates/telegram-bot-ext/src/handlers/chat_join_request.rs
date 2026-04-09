//! [`ChatJoinRequestHandler`] -- handles chat join request updates.
//!
//! Ported from `python-telegram-bot`'s `ChatJoinRequestHandler`. Supports
//! optional filtering by chat ID and/or username.

use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use rust_tg_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// Handler for `Update.chat_join_request`.
///
/// When neither `chat_ids` nor `usernames` is provided, every
/// `chat_join_request` update matches.
pub struct ChatJoinRequestHandler {
    callback: HandlerCallback,
    chat_ids: HashSet<i64>,
    usernames: HashSet<String>,
    block: bool,
}

impl ChatJoinRequestHandler {
    /// Create a new `ChatJoinRequestHandler`.
    ///
    /// Both `chat_ids` and `usernames` may be empty.
    pub fn new(
        callback: HandlerCallback,
        chat_ids: HashSet<i64>,
        usernames: HashSet<String>,
        block: bool,
    ) -> Self {
        // Normalize usernames to lowercase without leading `@`.
        let usernames = usernames
            .into_iter()
            .map(|u| u.trim_start_matches('@').to_lowercase())
            .collect();
        Self {
            callback,
            chat_ids,
            usernames,
            block,
        }
    }
}

impl Handler for ChatJoinRequestHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        let cjr = update.chat_join_request()?;

        // No filters -> accept all.
        if self.chat_ids.is_empty() && self.usernames.is_empty() {
            return Some(MatchResult::Empty);
        }

        // Check chat ID.
        if self.chat_ids.contains(&cjr.chat.id) {
            return Some(MatchResult::Empty);
        }

        // Check from_user username.
        if let Some(username) = cjr.from_user.username.as_deref() {
            if self.usernames.contains(&username.to_lowercase()) {
                return Some(MatchResult::Empty);
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
