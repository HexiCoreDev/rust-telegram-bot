//! [`PrefixHandler`] -- handles custom prefix commands.
//!
//! Ported from `python-telegram-bot`'s `PrefixHandler`. This is an
//! intermediate handler between [`MessageHandler`](super::message) and
//! [`CommandHandler`](super::command). It matches messages whose first word
//! equals one of the cartesian products of `prefix x command`
//! (case-insensitive).
//!
//! ## Filter integration
//!
//! An optional `filter_fn` runs *before* prefix matching. The default filter
//! accepts updates with `message` or `edited_message`, matching Python's
//! `filters.UpdateType.MESSAGES` default.

use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use telegram_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// Type alias for the optional update filter closure.
pub type UpdateFilter = Arc<dyn Fn(&Update) -> bool + Send + Sync>;

/// The default update filter: accepts updates with `message` or
/// `edited_message`, matching Python's `filters.UpdateType.MESSAGES`.
fn default_update_filter(update: &Update) -> bool {
    update.message.is_some() || update.edited_message.is_some()
}

/// Handler for custom prefix commands (e.g. `!help`, `#info`).
///
/// # Example
///
/// ```rust,ignore
/// use telegram_bot_ext::handlers::prefix::PrefixHandler;
/// use telegram_bot_ext::handlers::base::*;
/// use std::sync::Arc;
///
/// // Responds to "!test", "#test", "!help", "#help"
/// let handler = PrefixHandler::new(
///     vec!["!".into(), "#".into()],
///     vec!["test".into(), "help".into()],
///     Arc::new(|update, mr| Box::pin(async move { HandlerResult::Continue })),
///     true,
/// );
/// ```
pub struct PrefixHandler {
    /// Pre-computed set of `prefix + command` strings (lowercased).
    commands: HashSet<String>,
    callback: HandlerCallback,
    block: bool,
    /// Optional filter applied before prefix matching. When `None` the
    /// default behaviour is to accept updates with `message` or
    /// `edited_message` (matching Python's `UpdateType.MESSAGES`).
    filter_fn: Option<UpdateFilter>,
}

impl PrefixHandler {
    /// Create a new `PrefixHandler`.
    ///
    /// The handler will respond to every combination of `prefix` and
    /// `command`, all lowercased.
    pub fn new(
        prefixes: Vec<String>,
        commands: Vec<String>,
        callback: HandlerCallback,
        block: bool,
    ) -> Self {
        let mut combined = HashSet::new();
        for p in &prefixes {
            for c in &commands {
                combined.insert(format!("{}{}", p.to_lowercase(), c.to_lowercase()));
            }
        }
        Self {
            commands: combined,
            callback,
            block,
            filter_fn: None,
        }
    }

    /// Set a custom update filter.
    ///
    /// The filter runs *before* any prefix matching. If it returns `false`
    /// the update is immediately rejected.
    ///
    /// When no custom filter is supplied the default behaviour is to require
    /// `update.message` or `update.edited_message` to be `Some`, matching
    /// Python's `UpdateType.MESSAGES`.
    pub fn with_filter(mut self, filter: UpdateFilter) -> Self {
        self.filter_fn = Some(filter);
        self
    }
}

impl Handler for PrefixHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        // Apply filter before prefix matching.
        let passes_filter = match &self.filter_fn {
            Some(f) => f(update),
            None => default_update_filter(update),
        };
        if !passes_filter {
            return None;
        }

        let message = update.effective_message()?;
        let text = message.text.as_ref()?;

        let mut words = text.split_whitespace();
        let first_word = words.next()?;

        if !self.commands.contains(&first_word.to_lowercase()) {
            return None;
        }

        let args: Vec<String> = words.map(String::from).collect();
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
    use serde_json::json;

    fn noop_callback() -> HandlerCallback {
        Arc::new(|_u, _m| Box::pin(async { HandlerResult::Continue }))
    }

    #[test]
    fn cartesian_product_commands() {
        let h = PrefixHandler::new(
            vec!["!".into(), "#".into()],
            vec!["test".into(), "help".into()],
            noop_callback(),
            true,
        );
        assert!(h.commands.contains("!test"));
        assert!(h.commands.contains("#test"));
        assert!(h.commands.contains("!help"));
        assert!(h.commands.contains("#help"));
        assert_eq!(h.commands.len(), 4);
    }

    #[test]
    fn default_filter_accepts_message() {
        let h = PrefixHandler::new(
            vec!["!".into()],
            vec!["test".into()],
            noop_callback(),
            true,
        );
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": "!test arg1"
            }
        }))
        .expect("valid");
        assert!(h.check_update(&update).is_some());
    }

    #[test]
    fn default_filter_accepts_edited_message() {
        let h = PrefixHandler::new(
            vec!["!".into()],
            vec!["test".into()],
            noop_callback(),
            true,
        );
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "edited_message": {
                "message_id": 1,
                "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": "!test arg1"
            }
        }))
        .expect("valid");
        assert!(h.check_update(&update).is_some());
    }

    #[test]
    fn default_filter_rejects_channel_post() {
        let h = PrefixHandler::new(
            vec!["!".into()],
            vec!["test".into()],
            noop_callback(),
            true,
        );
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "channel_post": {
                "message_id": 1,
                "date": 0,
                "chat": {"id": -100, "type": "channel"},
                "text": "!test"
            }
        }))
        .expect("valid");
        assert!(h.check_update(&update).is_none());
    }

    #[test]
    fn custom_filter_rejects() {
        let h = PrefixHandler::new(
            vec!["!".into()],
            vec!["test".into()],
            noop_callback(),
            true,
        )
        .with_filter(Arc::new(|_u| false));
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": "!test"
            }
        }))
        .expect("valid");
        assert!(h.check_update(&update).is_none());
    }

    #[test]
    fn custom_filter_allows_channel_post() {
        let h = PrefixHandler::new(
            vec!["!".into()],
            vec!["test".into()],
            noop_callback(),
            true,
        )
        .with_filter(Arc::new(|u| {
            u.message.is_some() || u.channel_post.is_some()
        }));
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "channel_post": {
                "message_id": 1,
                "date": 0,
                "chat": {"id": -100, "type": "channel"},
                "text": "!test arg"
            }
        }))
        .expect("valid");
        assert!(h.check_update(&update).is_some());
    }
}
