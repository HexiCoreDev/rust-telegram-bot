//! [`StringRegexHandler`] -- handles messages whose text matches a regex.
//!
//! Adapted from `python-telegram-bot`'s `StringRegexHandler`. The Python
//! version operates on raw strings, not Telegram updates. Per the design
//! decision, this Rust version operates on `Update` objects, extracting
//! message text and matching it against a compiled regex.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use regex::Regex;
use rust_tg_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};
use crate::context::CallbackContext;

/// Handler that matches messages whose text matches a regex pattern.
///
/// Uses `regex::Regex::captures` (anchored at the start of the string,
/// matching `re.match` from Python).
///
/// When the regex contains named capture groups, both `context.matches`
/// (positional) and `context.named_matches` (named groups map) are
/// populated.
///
/// # Example
///
/// ```rust,ignore
/// use rust_tg_bot_ext::handlers::string_regex::StringRegexHandler;
/// use rust_tg_bot_ext::handlers::base::*;
/// use regex::Regex;
/// use std::sync::Arc;
///
/// let handler = StringRegexHandler::new(
///     Regex::new(r"^hello (\w+)").unwrap(),
///     Arc::new(|update, mr| Box::pin(async move { HandlerResult::Continue })),
///     true,
/// );
/// ```
pub struct StringRegexHandler {
    pattern: Regex,
    callback: HandlerCallback,
    block: bool,
}

impl StringRegexHandler {
    /// Create a new `StringRegexHandler`.
    pub fn new(pattern: Regex, callback: HandlerCallback, block: bool) -> Self {
        Self {
            pattern,
            callback,
            block,
        }
    }
}

impl Handler for StringRegexHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        let message = update.effective_message()?;
        let text = message.text.as_ref()?;

        let caps = self.pattern.captures(text)?;

        let positional: Vec<String> = caps
            .iter()
            .filter_map(|m| m.map(|m| m.as_str().to_owned()))
            .collect();

        // Collect named groups (only those that matched).
        let mut named: HashMap<String, String> = HashMap::new();
        for name in self.pattern.capture_names().flatten() {
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

    fn make_message_update(text: &str) -> Update {
        serde_json::from_str(&format!(
            r#"{{"update_id":1,"message":{{"message_id":1,"date":0,"chat":{{"id":1,"type":"private"}},"text":"{text}"}}}}"#
        ))
        .unwrap()
    }

    #[test]
    fn matches_regex() {
        let h =
            StringRegexHandler::new(Regex::new(r"^hello (\w+)").unwrap(), noop_callback(), true);
        let update: Update = serde_json::from_str(
            r#"{"update_id":1,"message":{"message_id":1,"date":0,"chat":{"id":1,"type":"private"},"text":"hello world"}}"#,
        ).unwrap();
        let result = h.check_update(&update);
        assert!(result.is_some());
        if let Some(MatchResult::RegexMatch(groups)) = result {
            assert_eq!(groups.len(), 2);
            assert_eq!(groups[0], "hello world");
            assert_eq!(groups[1], "world");
        } else {
            panic!("expected RegexMatch");
        }
    }

    #[test]
    fn no_match_returns_none() {
        let h = StringRegexHandler::new(Regex::new(r"^goodbye").unwrap(), noop_callback(), true);
        let update: Update = serde_json::from_str(
            r#"{"update_id":1,"message":{"message_id":1,"date":0,"chat":{"id":1,"type":"private"},"text":"hello world"}}"#,
        ).unwrap();
        assert!(h.check_update(&update).is_none());
    }

    #[test]
    fn named_group_returns_regex_match_with_names() {
        let h = StringRegexHandler::new(
            Regex::new(r"^hello (?P<name>\w+)").unwrap(),
            noop_callback(),
            true,
        );
        let update = make_message_update("hello alice");
        match h.check_update(&update) {
            Some(MatchResult::RegexMatchWithNames { positional, named }) => {
                assert_eq!(positional[0], "hello alice");
                assert_eq!(named.get("name").map(String::as_str), Some("alice"));
            }
            other => panic!("expected RegexMatchWithNames, got {other:?}"),
        }
    }

    #[test]
    fn collect_context_populates_matches() {
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

        let h = StringRegexHandler::new(Regex::new(r"x").unwrap(), noop_callback(), true);
        let mr = MatchResult::RegexMatch(vec!["hello".into()]);
        h.collect_additional_context(&mut ctx, &mr);
        assert_eq!(ctx.matches, Some(vec!["hello".into()]));
        assert!(ctx.named_matches.is_none());
    }

    #[test]
    fn collect_context_populates_named_matches() {
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

        let h = StringRegexHandler::new(Regex::new(r"x").unwrap(), noop_callback(), true);
        let mut named = HashMap::new();
        named.insert("name".into(), "alice".into());
        let mr = MatchResult::RegexMatchWithNames {
            positional: vec!["hello alice".into(), "alice".into()],
            named,
        };
        h.collect_additional_context(&mut ctx, &mr);
        assert_eq!(
            ctx.matches,
            Some(vec!["hello alice".into(), "alice".into()])
        );
        assert_eq!(
            ctx.named_matches
                .as_ref()
                .and_then(|m| m.get("name"))
                .map(String::as_str),
            Some("alice")
        );
    }
}
