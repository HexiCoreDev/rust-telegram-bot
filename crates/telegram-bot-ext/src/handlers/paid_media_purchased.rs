//! [`PaidMediaPurchasedHandler`] -- handles purchased paid media updates.
//!
//! Ported from `python-telegram-bot`'s `PaidMediaPurchasedHandler`. Supports
//! optional filtering by user ID and/or username.

use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use rust_tg_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// Handler for `Update.purchased_paid_media`.
///
/// When neither `user_ids` nor `usernames` is provided, every
/// `purchased_paid_media` update matches.
pub struct PaidMediaPurchasedHandler {
    callback: HandlerCallback,
    user_ids: HashSet<i64>,
    usernames: HashSet<String>,
    block: bool,
}

impl PaidMediaPurchasedHandler {
    /// Create a new `PaidMediaPurchasedHandler`.
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

impl Handler for PaidMediaPurchasedHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        let ppm = update.purchased_paid_media()?;

        // No filters -> accept all.
        if self.user_ids.is_empty() && self.usernames.is_empty() {
            return Some(MatchResult::Empty);
        }

        // Check from_user.id.
        if self.user_ids.contains(&ppm.from_user.id) {
            return Some(MatchResult::Empty);
        }

        // Check from_user.username.
        if let Some(username) = ppm.from_user.username.as_deref() {
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
