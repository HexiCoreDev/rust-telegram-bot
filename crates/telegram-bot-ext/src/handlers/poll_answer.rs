//! [`PollAnswerHandler`] -- handles updates that contain a poll answer.
//!
//! Ported from `python-telegram-bot`'s `PollAnswerHandler`. Matches any
//! update with `Update.poll_answer` set.

use std::future::Future;
use std::pin::Pin;

use telegram_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// Handler for `Update.poll_answer`.
///
/// Bots receive poll answer updates only for polls they sent.
pub struct PollAnswerHandler {
    callback: HandlerCallback,
    block: bool,
}

impl PollAnswerHandler {
    /// Create a new `PollAnswerHandler`.
    pub fn new(callback: HandlerCallback, block: bool) -> Self {
        Self { callback, block }
    }
}

impl Handler for PollAnswerHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        if update.poll_answer.is_some() {
            Some(MatchResult::Empty)
        } else {
            None
        }
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
