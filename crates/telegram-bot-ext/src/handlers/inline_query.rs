//! [`InlineQueryHandler`] -- handles updates containing an inline query.
//!
//! Ported from `python-telegram-bot`'s `InlineQueryHandler`. Supports
//! optional regex matching on `inline_query.query` and optional
//! `chat_types` filtering.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use regex::Regex;
use telegram_bot_raw::types::update::Update;

use crate::context::CallbackContext;
use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// Handler for `Update.inline_query`.
///
/// # Matching rules
///
/// 1. The update must carry an `inline_query`.
/// 2. If `chat_types` is set, the inline query's `chat_type` must be in the
///    list. Queries without `chat_type` (e.g. from secret chats) are rejected.
/// 3. If `pattern` is set, `re.match(pattern, query)` must succeed.
/// 4. When all checks pass, captured groups (if any) are returned as
///    `MatchResult::RegexMatch` or `MatchResult::RegexMatchWithNames` (when
///    the pattern contains named capture groups).
pub struct InlineQueryHandler {
    callback: HandlerCallback,
    pattern: Option<Regex>,
    chat_types: Option<Vec<String>>,
    block: bool,
}

impl InlineQueryHandler {
    /// Create a new `InlineQueryHandler`.
    pub fn new(
        callback: HandlerCallback,
        pattern: Option<Regex>,
        chat_types: Option<Vec<String>>,
        block: bool,
    ) -> Self {
        Self {
            callback,
            pattern,
            chat_types,
            block,
        }
    }
}

impl Handler for InlineQueryHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        let iq = update.inline_query.as_ref()?;

        // chat_types filter
        if let Some(ref allowed) = self.chat_types {
            let chat_type = iq.chat_type.as_deref()?;
            if !allowed.iter().any(|t| t == chat_type) {
                return None;
            }
        }

        // Pattern filter
        if let Some(ref re) = self.pattern {
            let query = &iq.query;
            let caps = re.captures(query)?;

            let positional: Vec<String> = caps
                .iter()
                .filter_map(|m| m.map(|m| m.as_str().to_owned()))
                .collect();

            // Collect named groups (only those that matched).
            let mut named: HashMap<String, String> = HashMap::new();
            for name in re.capture_names().flatten() {
                if let Some(m) = caps.name(name) {
                    named.insert(name.to_owned(), m.as_str().to_owned());
                }
            }

            return if named.is_empty() {
                Some(MatchResult::RegexMatch(positional))
            } else {
                Some(MatchResult::RegexMatchWithNames { positional, named })
            };
        }

        Some(MatchResult::Empty)
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
