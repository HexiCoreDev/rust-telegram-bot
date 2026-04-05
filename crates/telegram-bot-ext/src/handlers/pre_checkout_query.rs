//! [`PreCheckoutQueryHandler`] -- handles pre-checkout query updates.
//!
//! Ported from `python-telegram-bot`'s `PreCheckoutQueryHandler`. Supports
//! optional regex matching on `pre_checkout_query.invoice_payload`.

use std::future::Future;
use std::pin::Pin;

use regex::Regex;
use telegram_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// Handler for `Update.pre_checkout_query`.
///
/// Without a pattern, any pre-checkout query matches. With a pattern, only
/// queries whose `invoice_payload` matches the regex are accepted.
pub struct PreCheckoutQueryHandler {
    callback: HandlerCallback,
    pattern: Option<Regex>,
    block: bool,
}

impl PreCheckoutQueryHandler {
    /// Create a new `PreCheckoutQueryHandler`.
    pub fn new(callback: HandlerCallback, pattern: Option<Regex>, block: bool) -> Self {
        Self {
            callback,
            pattern,
            block,
        }
    }
}

impl Handler for PreCheckoutQueryHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        let pcq = update.pre_checkout_query.as_ref()?;

        if let Some(ref re) = self.pattern {
            let payload = &pcq.invoice_payload;
            if re.is_match(payload) {
                Some(MatchResult::Empty)
            } else {
                None
            }
        } else {
            Some(MatchResult::Empty)
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
