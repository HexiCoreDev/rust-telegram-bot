//! [`StringCommandHandler`] -- handles `/command` strings extracted from
//! message text.
//!
//! Adapted from `python-telegram-bot`'s `StringCommandHandler`. The Python
//! version operates on raw strings put into the queue, not Telegram updates.
//! Per the design decision, this Rust version operates on `Update` objects,
//! extracting message text and checking for `/command` syntax.

use std::future::Future;
use std::pin::Pin;

use telegram_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// Handler that matches messages whose text starts with `/command`.
///
/// Unlike [`CommandHandler`](super::command), this handler does **not**
/// require the message to have a `bot_command` entity. It performs a plain
/// string prefix check on the message text.
///
/// # Matching rules
///
/// 1. The update must carry an effective message with non-empty `text`.
/// 2. The text must start with `/<command>` (case-sensitive).
/// 3. Arguments are the remaining words after the command.
pub struct StringCommandHandler {
    /// The command to listen for (without leading `/`).
    command: String,
    callback: HandlerCallback,
    block: bool,
}

impl StringCommandHandler {
    /// Create a new `StringCommandHandler`.
    pub fn new(command: String, callback: HandlerCallback, block: bool) -> Self {
        Self {
            command,
            callback,
            block,
        }
    }
}

impl Handler for StringCommandHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        let message = update.effective_message()?;
        let text = message.text.as_ref()?;

        if !text.starts_with('/') {
            return None;
        }

        let without_slash = &text[1..];
        let mut parts = without_slash.split_whitespace();
        let cmd = parts.next()?;

        if cmd != self.command {
            return None;
        }

        let args: Vec<String> = parts.map(String::from).collect();
        Some(MatchResult::Args(args))
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    fn noop_callback() -> HandlerCallback {
        Arc::new(|_update, _mr| Box::pin(async { HandlerResult::Continue }))
    }

    #[test]
    fn matches_correct_command() {
        let h = StringCommandHandler::new("start".into(), noop_callback(), true);
        // Build a minimal update with message text "/start hello world"
        let update: Update = serde_json::from_str(
            r#"{"update_id":1,"message":{"message_id":1,"date":0,"chat":{"id":1,"type":"private"},"text":"/start hello world"}}"#,
        ).unwrap();
        let result = h.check_update(&update);
        assert!(result.is_some());
        if let Some(MatchResult::Args(args)) = result {
            assert_eq!(args, vec!["hello", "world"]);
        } else {
            panic!("expected Args");
        }
    }

    #[test]
    fn rejects_wrong_command() {
        let h = StringCommandHandler::new("start".into(), noop_callback(), true);
        let update: Update = serde_json::from_str(
            r#"{"update_id":1,"message":{"message_id":1,"date":0,"chat":{"id":1,"type":"private"},"text":"/help"}}"#,
        ).unwrap();
        assert!(h.check_update(&update).is_none());
    }
}
