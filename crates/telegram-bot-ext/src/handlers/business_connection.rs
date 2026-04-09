//! [`BusinessConnectionHandler`] -- handles business connection updates.
//!
//! Ported from `python-telegram-bot`'s `BusinessConnectionHandler`. Supports
//! optional filtering by user ID and/or username.

use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use rust_tg_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// Handler for `Update.business_connection`.
///
/// When neither `user_ids` nor `usernames` is provided, every
/// `business_connection` update matches.
pub struct BusinessConnectionHandler {
    callback: HandlerCallback,
    user_ids: HashSet<i64>,
    usernames: HashSet<String>,
    block: bool,
}

impl BusinessConnectionHandler {
    /// Create a new `BusinessConnectionHandler`.
    pub fn new(
        callback: HandlerCallback,
        user_ids: HashSet<i64>,
        usernames: HashSet<String>,
        block: bool,
    ) -> Self {
        let usernames = usernames
            .into_iter()
            .map(|u| u.trim_start_matches('@').to_lowercase())
            .collect();
        Self {
            callback,
            user_ids,
            usernames,
            block,
        }
    }
}

impl Handler for BusinessConnectionHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        let bc = update.business_connection()?;

        // No filters -> accept all.
        if self.user_ids.is_empty() && self.usernames.is_empty() {
            return Some(MatchResult::Empty);
        }

        // Check user.id.
        if self.user_ids.contains(&bc.user.id) {
            return Some(MatchResult::Empty);
        }

        // Check user.username.
        if let Some(username) = bc.user.username.as_deref() {
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
