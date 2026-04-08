//! [`ChosenInlineResultHandler`] -- handles updates containing a chosen
//! inline result.
//!
//! Ported from `python-telegram-bot`'s `ChosenInlineResultHandler`. Supports
//! optional regex matching on `chosen_inline_result.result_id`.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use regex::Regex;
use telegram_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// Handler for `Update.chosen_inline_result`.
///
/// When `pattern` is set, `result_id` is matched against the compiled regex.
/// Without a pattern every `chosen_inline_result` update matches.
pub struct ChosenInlineResultHandler {
    callback: HandlerCallback,
    pattern: Option<Regex>,
    block: bool,
}

impl ChosenInlineResultHandler {
    /// Create a new `ChosenInlineResultHandler`.
    pub fn new(callback: HandlerCallback, pattern: Option<Regex>, block: bool) -> Self {
        Self {
            callback,
            pattern,
            block,
        }
    }
}

impl Handler for ChosenInlineResultHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        let cir = update.chosen_inline_result.as_ref()?;

        if let Some(ref re) = self.pattern {
            let result_id = &cir.result_id;
            let caps = re.captures(result_id)?;
            let groups: Vec<String> = caps
                .iter()
                .filter_map(|m| m.map(|m| m.as_str().to_owned()))
                .collect();
            return Some(MatchResult::RegexMatch(groups));
        }

        Some(MatchResult::Empty)
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
