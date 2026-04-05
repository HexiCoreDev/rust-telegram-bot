//! [`ShippingQueryHandler`] -- handles shipping query updates.
//!
//! Ported from `python-telegram-bot`'s `ShippingQueryHandler`. Matches any
//! update with `Update.shipping_query` set.

use std::future::Future;
use std::pin::Pin;

use telegram_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// Handler for `Update.shipping_query`.
///
/// Only applicable to invoices with flexible pricing.
pub struct ShippingQueryHandler {
    callback: HandlerCallback,
    block: bool,
}

impl ShippingQueryHandler {
    /// Create a new `ShippingQueryHandler`.
    pub fn new(callback: HandlerCallback, block: bool) -> Self {
        Self { callback, block }
    }
}

impl Handler for ShippingQueryHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        if update.shipping_query.is_some() {
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
