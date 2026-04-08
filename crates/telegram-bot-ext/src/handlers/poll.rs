//! [`PollHandler`] -- handles updates that contain a poll.
//!
//! Ported from `python-telegram-bot`'s `PollHandler`. This is the simplest
//! handler: it matches any update that has `Update.poll` set.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use telegram_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// Handler for `Update.poll`.
///
/// Bots receive poll updates only for polls they sent or stopped polls.
pub struct PollHandler {
    callback: HandlerCallback,
    block: bool,
}

impl PollHandler {
    /// Create a new `PollHandler`.
    pub fn new(callback: HandlerCallback, block: bool) -> Self {
        Self { callback, block }
    }
}

impl Handler for PollHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        if update.poll().is_some() {
            Some(MatchResult::Empty)
        } else {
            None
        }
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
