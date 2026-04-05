//! [`BusinessMessagesDeletedHandler`] -- handles deleted business messages.
//!
//! Ported from `python-telegram-bot`'s `BusinessMessagesDeletedHandler`.
//! Supports optional filtering by chat ID and/or username.

use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;

use telegram_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// Handler for `Update.deleted_business_messages`.
///
/// When neither `chat_ids` nor `usernames` is provided, every
/// `deleted_business_messages` update matches.
pub struct BusinessMessagesDeletedHandler {
    callback: HandlerCallback,
    chat_ids: HashSet<i64>,
    usernames: HashSet<String>,
    block: bool,
}

impl BusinessMessagesDeletedHandler {
    /// Create a new `BusinessMessagesDeletedHandler`.
    pub fn new(
        callback: HandlerCallback,
        chat_ids: HashSet<i64>,
        usernames: HashSet<String>,
        block: bool,
    ) -> Self {
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

impl Handler for BusinessMessagesDeletedHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        let dbm = update.deleted_business_messages.as_ref()?;

        // No filters -> accept all.
        if self.chat_ids.is_empty() && self.usernames.is_empty() {
            return Some(MatchResult::Empty);
        }

        // Check chat.id.
        if self.chat_ids.contains(&dbm.chat.id) {
            return Some(MatchResult::Empty);
        }

        // Check chat.username.
        if let Some(username) = dbm.chat.username.as_deref() {
            if self.usernames.contains(&username.to_lowercase()) {
                return Some(MatchResult::Empty);
            }
        }

        None
    }

    fn handle_update(
        &self,
        update: Update,
        match_result: MatchResult,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> {
        (self.callback)(update, match_result)
    }

    fn block(&self) -> bool {
        self.block
    }
}
