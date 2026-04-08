//! [`MessageHandler`] -- handles Telegram messages filtered by a predicate.
//!
//! Ported from `python-telegram-bot`'s `MessageHandler`. Accepts any
//! `Update` that passes an optional filter. When no filter is provided the
//! handler matches every `Update`.
//!
//! ## Filter integration
//!
//! The handler accepts `Option<crate::filters::base::F>`, the composable
//! `Filter` wrapper from the filter system. Filter data extracted via
//! `FilterResult::MatchWithData` is forwarded to the handler callback
//! through `MatchResult::Custom`.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use telegram_bot_raw::types::update::Update;

use super::base::{ContextCallback, Handler, HandlerCallback, HandlerResult, MatchResult};
use crate::context::CallbackContext;
use crate::filters::base::{self, Filter, FilterResult};

/// Legacy type alias kept for backward compatibility. New code should use
/// `crate::filters::base::F` directly.
pub type FilterFn = Arc<dyn Fn(&Update) -> bool + Send + Sync>;

/// Handler that matches updates based on the composable [`Filter`](base::Filter)
/// trait system.
///
/// This is the most general-purpose handler. It mirrors the Python
/// `MessageHandler` which delegates to a `BaseFilter` tree. The Rust port
/// uses `crate::filters::base::F` -- the operator-overloaded filter wrapper
/// -- so filters compose naturally with `&`, `|`, `^`, `!`.
///
/// When a filter returns `FilterResult::MatchWithData`, the extracted data
/// is stored in `MatchResult::Custom` and flows through to the handler
/// callback.
///
/// # Ergonomic constructor
///
/// ```rust,ignore
/// use telegram_bot_ext::prelude::*;
///
/// async fn echo(update: Update, context: Context) -> HandlerResult {
///     let text = update.effective_message().and_then(|m| m.text.as_deref()).unwrap_or("");
///     context.reply_text(&update, text).await?;
///     Ok(())
/// }
///
/// MessageHandler::new(TEXT & !COMMAND, echo);
/// ```
///
/// # Full-control constructor
///
/// ```rust,ignore
/// use telegram_bot_ext::handlers::message::MessageHandler;
/// use telegram_bot_ext::handlers::base::*;
/// use telegram_bot_ext::filters::base::F;
/// use telegram_bot_ext::filters::base::All;
/// use std::sync::Arc;
///
/// let handler = MessageHandler::with_options(
///     Some(F::new(All)),
///     Arc::new(|update, _mr| Box::pin(async move { HandlerResult::Continue })),
///     true,
/// );
/// ```
pub struct MessageHandler {
    filter: Option<base::F>,
    callback: HandlerCallback,
    block: bool,
    /// Optional context-aware callback for the ergonomic API.
    context_callback: Option<ContextCallback>,
}

impl MessageHandler {
    /// Ergonomic constructor matching python-telegram-bot's
    /// `MessageHandler(filters, callback)`.
    ///
    /// Accepts a composable filter and an async handler function with
    /// signature `async fn(Update, Context) -> HandlerResult`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use telegram_bot_ext::prelude::*;
    ///
    /// async fn echo(update: Update, context: Context) -> HandlerResult {
    ///     let text = update.effective_message().and_then(|m| m.text.as_deref()).unwrap_or("");
    ///     context.reply_text(&update, text).await?;
    ///     Ok(())
    /// }
    ///
    /// MessageHandler::new(TEXT & !COMMAND, echo);
    /// ```
    pub fn new<Cb, Fut>(filter: base::F, callback: Cb) -> Self
    where
        Cb: Fn(Arc<Update>, CallbackContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), crate::application::HandlerError>> + Send + 'static,
    {
        let cb = Arc::new(callback);
        let context_cb: ContextCallback = Arc::new(move |update, ctx| {
            let fut = cb(update, ctx);
            Box::pin(fut)
                as Pin<
                    Box<dyn Future<Output = Result<(), crate::application::HandlerError>> + Send>,
                >
        });

        // The raw callback is a no-op; handle_update_with_context is used instead.
        let noop_callback: HandlerCallback =
            Arc::new(|_update, _mr| Box::pin(async { HandlerResult::Continue }));

        Self {
            filter: Some(filter),
            callback: noop_callback,
            block: true,
            context_callback: Some(context_cb),
        }
    }

    /// Full-control constructor for advanced use cases.
    ///
    /// If `filter` is `None`, every `Update` matches (equivalent to the
    /// Python `filters.ALL`).
    pub fn with_options(filter: Option<base::F>, callback: HandlerCallback, block: bool) -> Self {
        Self {
            filter,
            callback,
            block,
            context_callback: None,
        }
    }

    /// Create a `MessageHandler` from a legacy closure filter.
    ///
    /// This is a convenience constructor for backward compatibility with code
    /// that uses `Fn(&Update) -> bool` closures instead of the `Filter` trait.
    pub fn from_fn(filter: Option<FilterFn>, callback: HandlerCallback, block: bool) -> Self {
        let f = filter.map(|closure| base::F::new(ClosureFilter(closure)));
        Self {
            filter: f,
            callback,
            block,
            context_callback: None,
        }
    }
}

/// Internal adapter: wraps a `Fn(&Update) -> bool` closure as a `Filter`.
///
/// Now that the `Filter` trait and the handler both use the same typed
/// `Update`, no conversion is needed.
struct ClosureFilter(Arc<dyn Fn(&Update) -> bool + Send + Sync>);

impl base::Filter for ClosureFilter {
    fn check_update(&self, update: &base::Update) -> FilterResult {
        if (self.0)(update) {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "ClosureFilter"
    }
}

impl Handler for MessageHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        match &self.filter {
            Some(f) => {
                let result = f.check_update(update);
                match result {
                    FilterResult::NoMatch => None,
                    FilterResult::Match => Some(MatchResult::Empty),
                    FilterResult::MatchWithData(data) => Some(MatchResult::Custom(Box::new(data))),
                }
            }
            None => Some(MatchResult::Empty),
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

    fn handle_update_with_context(
        &self,
        update: Arc<Update>,
        match_result: MatchResult,
        context: CallbackContext,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> {
        if let Some(ref cb) = self.context_callback {
            let fut = cb(update, context);
            Box::pin(async move {
                match fut.await {
                    Ok(()) => HandlerResult::Continue,
                    Err(crate::application::HandlerError::HandlerStop { .. }) => {
                        HandlerResult::Stop
                    }
                    Err(crate::application::HandlerError::Other(e)) => HandlerResult::Error(e),
                }
            })
        } else {
            (self.callback)(update, match_result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::base::{All, FnFilter, F};

    fn noop_callback() -> HandlerCallback {
        Arc::new(|_update, _mr| Box::pin(async { HandlerResult::Continue }))
    }

    fn empty_update() -> Update {
        serde_json::from_str(r#"{"update_id": 1}"#).unwrap()
    }

    fn text_update() -> Update {
        serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": "hello"
            }
        }))
        .unwrap()
    }

    #[test]
    fn no_filter_matches_everything() {
        let h = MessageHandler::with_options(None, noop_callback(), true);
        assert!(h.check_update(&empty_update()).is_some());
    }

    #[test]
    fn filter_trait_all_matches_message() {
        let h = MessageHandler::with_options(Some(F::new(All)), noop_callback(), true);
        assert!(h.check_update(&text_update()).is_some());
    }

    #[test]
    fn filter_trait_all_rejects_empty() {
        let h = MessageHandler::with_options(Some(F::new(All)), noop_callback(), true);
        assert!(h.check_update(&empty_update()).is_none());
    }

    #[test]
    fn filter_not_combinator() {
        // !ALL should reject messages.
        let h = MessageHandler::with_options(Some(!F::new(All)), noop_callback(), true);
        assert!(h.check_update(&text_update()).is_none());
        // ...but accept empty updates.
        assert!(h.check_update(&empty_update()).is_some());
    }

    #[test]
    fn from_fn_filter_rejects() {
        let h = MessageHandler::from_fn(Some(Arc::new(|_u| false)), noop_callback(), true);
        assert!(h.check_update(&empty_update()).is_none());
    }

    #[test]
    fn from_fn_filter_accepts() {
        let h = MessageHandler::from_fn(Some(Arc::new(|_u| true)), noop_callback(), true);
        assert!(h.check_update(&empty_update()).is_some());
    }

    #[test]
    fn filter_data_flows_through() {
        let f = FnFilter::new("always", |_| true);
        let h = MessageHandler::with_options(Some(F::new(f)), noop_callback(), true);
        let result = h.check_update(&empty_update());
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), MatchResult::Empty));
    }

    #[test]
    fn composed_filters_work() {
        let always = FnFilter::new("always", |_| true);
        let never = FnFilter::new("never", |_| false);
        let h = MessageHandler::with_options(
            Some(F::new(always) & F::new(never)),
            noop_callback(),
            true,
        );
        assert!(h.check_update(&empty_update()).is_none());
    }

    #[test]
    fn or_composed_filters_work() {
        let always = FnFilter::new("always", |_| true);
        let never = FnFilter::new("never", |_| false);
        let h = MessageHandler::with_options(
            Some(F::new(always) | F::new(never)),
            noop_callback(),
            true,
        );
        assert!(h.check_update(&empty_update()).is_some());
    }
}
