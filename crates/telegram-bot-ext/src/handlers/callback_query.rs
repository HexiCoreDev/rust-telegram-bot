//! [`CallbackQueryHandler`] -- handles updates containing a callback query.
//!
//! Ported from `python-telegram-bot`'s `CallbackQueryHandler`. Supports
//! optional regex pattern matching on `callback_query.data` and
//! `callback_query.game_short_name`, as well as predicate functions for
//! Rust-idiomatic `callable(data)` and type-check patterns.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use regex::Regex;
use rust_tg_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};
use crate::context::CallbackContext;

/// Pattern to match against callback query data or game short name.
#[derive(Clone)]
#[non_exhaustive]
pub enum CallbackPattern {
    /// Match `callback_query.data` against this compiled regex.
    Data(Regex),
    /// Match `callback_query.game_short_name` against this compiled regex.
    Game(Regex),
    /// Match both: data against the first regex, game against the second.
    Both {
        /// Regex to match against `callback_query.data`.
        data: Regex,
        /// Regex to match against `callback_query.game_short_name`.
        game: Regex,
    },
    /// Match `callback_query.data` using an arbitrary predicate function.
    ///
    /// This covers Python's `callable(data)` and `isinstance(data, type)`
    /// patterns in a Rust-idiomatic way. The predicate receives the data
    /// string and returns `true` if the callback query should be handled.
    Predicate(Arc<dyn Fn(&str) -> bool + Send + Sync>),
}

/// Handler for `Update.callback_query`.
///
/// When no pattern is set, any callback query matches. When a `Data` pattern
/// is set, only queries with matching `.data` are accepted (queries with only
/// `.game_short_name` are rejected, and vice versa).
///
/// Named capture groups in the regex pattern are exposed via
/// `context.named_matches` (a `HashMap<String, String>`), while all captures
/// (positional) are available as `context.matches`. This mirrors Python's
/// behaviour of putting the full `re.Match` object into `context.matches`.
///
/// # Example
///
/// ```rust,ignore
/// use rust_tg_bot_ext::handlers::callback_query::{CallbackQueryHandler, CallbackPattern};
/// use rust_tg_bot_ext::handlers::base::*;
/// use regex::Regex;
/// use std::sync::Arc;
///
/// // Regex-based matching:
/// let handler = CallbackQueryHandler::new(
///     Arc::new(|update, mr| Box::pin(async move { HandlerResult::Continue })),
///     Some(CallbackPattern::Data(Regex::new(r"^btn_(\d+)$").unwrap())),
///     true,
/// );
///
/// // Predicate-based matching (covers callable/type patterns):
/// let handler2 = CallbackQueryHandler::new(
///     Arc::new(|update, mr| Box::pin(async move { HandlerResult::Continue })),
///     Some(CallbackPattern::Predicate(Arc::new(|data| data.starts_with("action_")))),
///     true,
/// );
/// ```
pub struct CallbackQueryHandler {
    callback: HandlerCallback,
    pattern: Option<CallbackPattern>,
    block: bool,
}

impl CallbackQueryHandler {
    /// Create a new `CallbackQueryHandler`.
    pub fn new(callback: HandlerCallback, pattern: Option<CallbackPattern>, block: bool) -> Self {
        Self {
            callback,
            pattern,
            block,
        }
    }

    /// Attempt regex match, returning captured groups as a `MatchResult`.
    ///
    /// When the regex contains at least one named capture group, returns
    /// `MatchResult::RegexMatchWithNames` so that callers can access both
    /// positional captures and the named-group map. Otherwise returns
    /// `MatchResult::RegexMatch` (positional-only, backwards compatible).
    fn try_regex(re: &Regex, text: &str) -> Option<MatchResult> {
        let caps = re.captures(text)?;

        let positional: Vec<String> = caps
            .iter()
            .filter_map(|m| m.map(|m| m.as_str().to_owned()))
            .collect();

        // Collect named groups. `capture_names()` yields `Option<&str>` for
        // each capture slot (None for unnamed slots). We only include names
        // that actually matched.
        let mut named: HashMap<String, String> = HashMap::new();
        for name in re.capture_names().flatten() {
            if let Some(m) = caps.name(name) {
                named.insert(name.to_owned(), m.as_str().to_owned());
            }
        }

        if named.is_empty() {
            Some(MatchResult::RegexMatch(positional))
        } else {
            Some(MatchResult::RegexMatchWithNames { positional, named })
        }
    }
}

impl Handler for CallbackQueryHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        let cq = update.callback_query()?;

        match &self.pattern {
            None => {
                // No pattern: accept any callback query.
                Some(MatchResult::Empty)
            }
            Some(CallbackPattern::Data(re)) => {
                let data = cq.data.as_ref()?;
                Self::try_regex(re, data)
            }
            Some(CallbackPattern::Game(re)) => {
                let game = cq.game_short_name.as_ref()?;
                Self::try_regex(re, game)
            }
            Some(CallbackPattern::Both {
                data: dre,
                game: gre,
            }) => {
                // Match whichever field is present.
                if let Some(data) = cq.data.as_ref() {
                    if let Some(mr) = Self::try_regex(dre, data) {
                        return Some(mr);
                    }
                }
                if let Some(game) = cq.game_short_name.as_ref() {
                    return Self::try_regex(gre, game);
                }
                None
            }
            Some(CallbackPattern::Predicate(pred)) => {
                let data = cq.data.as_ref()?;
                if pred(data) {
                    Some(MatchResult::Empty)
                } else {
                    None
                }
            }
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

    /// Populate `context.matches` (positional) and `context.named_matches`
    /// (named groups) from the regex match result.
    ///
    /// Mirrors Python's `CallbackQueryHandler.collect_additional_context`
    /// which injects the `re.Match` object into `context.matches`.
    fn collect_additional_context(
        &self,
        context: &mut CallbackContext,
        match_result: &MatchResult,
    ) {
        match match_result {
            MatchResult::RegexMatch(groups) => {
                context.matches = Some(groups.clone());
            }
            MatchResult::RegexMatchWithNames { positional, named } => {
                context.matches = Some(positional.clone());
                context.named_matches = Some(named.clone());
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    fn noop_callback() -> HandlerCallback {
        Arc::new(|_update, _mr| Box::pin(async { HandlerResult::Continue }))
    }

    fn make_callback_query_update(data: &str) -> Update {
        serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "callback_query": {
                "id": "42",
                "from": {"id": 1, "is_bot": false, "first_name": "Test"},
                "chat_instance": "ci",
                "data": data
            }
        }))
        .expect("valid callback query update")
    }

    #[test]
    fn no_callback_query_returns_none() {
        let h = CallbackQueryHandler::new(noop_callback(), None, true);
        let update: Update = serde_json::from_str(r#"{"update_id": 1}"#).unwrap();
        assert!(h.check_update(&update).is_none());
    }

    #[test]
    fn predicate_accepts_matching_data() {
        let h = CallbackQueryHandler::new(
            noop_callback(),
            Some(CallbackPattern::Predicate(Arc::new(|data| {
                data.starts_with("btn_")
            }))),
            true,
        );
        let update = make_callback_query_update("btn_42");
        assert!(h.check_update(&update).is_some());
    }

    #[test]
    fn predicate_rejects_non_matching_data() {
        let h = CallbackQueryHandler::new(
            noop_callback(),
            Some(CallbackPattern::Predicate(Arc::new(|data| {
                data.starts_with("btn_")
            }))),
            true,
        );
        let update = make_callback_query_update("action_42");
        assert!(h.check_update(&update).is_none());
    }

    #[test]
    fn predicate_requires_data_field() {
        // Callback query without data should not match Predicate.
        let h = CallbackQueryHandler::new(
            noop_callback(),
            Some(CallbackPattern::Predicate(Arc::new(|_| true))),
            true,
        );
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "callback_query": {
                "id": "42",
                "from": {"id": 1, "is_bot": false, "first_name": "Test"},
                "chat_instance": "ci",
                "game_short_name": "mygame"
            }
        }))
        .expect("valid");
        assert!(h.check_update(&update).is_none());
    }

    #[test]
    fn no_pattern_accepts_any_callback_query() {
        let h = CallbackQueryHandler::new(noop_callback(), None, true);
        let update = make_callback_query_update("anything");
        assert!(h.check_update(&update).is_some());
    }

    #[test]
    fn regex_data_pattern_matches() {
        let h = CallbackQueryHandler::new(
            noop_callback(),
            Some(CallbackPattern::Data(Regex::new(r"^btn_(\d+)$").unwrap())),
            true,
        );
        let update = make_callback_query_update("btn_123");
        let result = h.check_update(&update);
        assert!(result.is_some());
        if let Some(MatchResult::RegexMatch(groups)) = result {
            assert_eq!(groups[0], "btn_123");
            assert_eq!(groups[1], "123");
        } else {
            panic!("expected RegexMatch");
        }
    }

    #[test]
    fn named_group_pattern_returns_regex_match_with_names() {
        let re = Regex::new(r"^btn_(?P<id>\d+)$").unwrap();
        let h = CallbackQueryHandler::new(noop_callback(), Some(CallbackPattern::Data(re)), true);
        let update = make_callback_query_update("btn_99");
        match h.check_update(&update) {
            Some(MatchResult::RegexMatchWithNames { positional, named }) => {
                assert_eq!(positional[0], "btn_99");
                assert_eq!(named.get("id").map(String::as_str), Some("99"));
            }
            other => panic!("expected RegexMatchWithNames, got {other:?}"),
        }
    }

    #[test]
    fn collect_context_positional() {
        use crate::context::CallbackContext;
        use crate::ext_bot::test_support::mock_request;
        use rust_tg_bot_raw::bot::Bot;

        let bot = Arc::new(crate::ext_bot::ExtBot::from_bot(Bot::new(
            "test",
            mock_request(),
        )));
        let stores = (
            Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        );
        let mut ctx = CallbackContext::new(bot, None, None, stores.0, stores.1, stores.2);

        let h = CallbackQueryHandler::new(noop_callback(), None, true);
        let mr = MatchResult::RegexMatch(vec!["full".into(), "group1".into()]);
        h.collect_additional_context(&mut ctx, &mr);

        assert_eq!(ctx.matches, Some(vec!["full".into(), "group1".into()]));
        assert!(ctx.named_matches.is_none());
    }

    #[test]
    fn collect_context_named() {
        use crate::context::CallbackContext;
        use crate::ext_bot::test_support::mock_request;
        use rust_tg_bot_raw::bot::Bot;

        let bot = Arc::new(crate::ext_bot::ExtBot::from_bot(Bot::new(
            "test",
            mock_request(),
        )));
        let stores = (
            Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        );
        let mut ctx = CallbackContext::new(bot, None, None, stores.0, stores.1, stores.2);

        let h = CallbackQueryHandler::new(noop_callback(), None, true);
        let mut named = HashMap::new();
        named.insert("id".into(), "99".into());
        let mr = MatchResult::RegexMatchWithNames {
            positional: vec!["btn_99".into(), "99".into()],
            named,
        };
        h.collect_additional_context(&mut ctx, &mr);

        assert_eq!(ctx.matches, Some(vec!["btn_99".into(), "99".into()]));
        assert_eq!(
            ctx.named_matches
                .as_ref()
                .and_then(|m| m.get("id"))
                .map(String::as_str),
            Some("99")
        );
    }
}
