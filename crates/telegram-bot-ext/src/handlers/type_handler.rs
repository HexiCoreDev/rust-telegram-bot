//! [`TypeHandler`] -- generic predicate-based handler.
//!
//! Adapted from `python-telegram-bot`'s `TypeHandler`. The Python version
//! dispatches based on `isinstance(update, type)`. In Rust there is no
//! runtime type hierarchy for updates, so this handler instead uses a
//! generic `Fn(&Update) -> bool` predicate -- the most idiomatic Rust
//! approach.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use telegram_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// A predicate function that inspects an [`Update`] and returns `true` if the
/// handler should process it.
pub type PredicateFn = Arc<dyn Fn(&Update) -> bool + Send + Sync>;

/// Generic predicate handler.
///
/// This is the most flexible handler. It accepts any `Fn(&Update) -> bool`
/// predicate and triggers when the predicate returns `true`.
///
/// # Example
///
/// ```rust,ignore
/// use telegram_bot_ext::handlers::type_handler::TypeHandler;
/// use telegram_bot_ext::handlers::base::*;
/// use std::sync::Arc;
///
/// // Handle any update that has a poll.
/// let handler = TypeHandler::new(
///     Arc::new(|u| u.poll.is_some()),
///     Arc::new(|update, mr| Box::pin(async move { HandlerResult::Continue })),
///     true,
/// );
/// ```
pub struct TypeHandler {
    predicate: PredicateFn,
    callback: HandlerCallback,
    block: bool,
}

impl TypeHandler {
    /// Create a new `TypeHandler` with a predicate function.
    pub fn new(predicate: PredicateFn, callback: HandlerCallback, block: bool) -> Self {
        Self {
            predicate,
            callback,
            block,
        }
    }
}

impl Handler for TypeHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        if (self.predicate)(update) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn noop_callback() -> HandlerCallback {
        Arc::new(|_update, _mr| Box::pin(async { HandlerResult::Continue }))
    }

    #[test]
    fn predicate_true_matches() {
        let h = TypeHandler::new(
            Arc::new(|_u| true),
            noop_callback(),
            true,
        );
        let update: Update = serde_json::from_str(r#"{"update_id": 1}"#).unwrap();
        assert!(h.check_update(&update).is_some());
    }

    #[test]
    fn predicate_false_rejects() {
        let h = TypeHandler::new(
            Arc::new(|_u| false),
            noop_callback(),
            true,
        );
        let update: Update = serde_json::from_str(r#"{"update_id": 1}"#).unwrap();
        assert!(h.check_update(&update).is_none());
    }
}
