//! Command filter.
//!
//! - [`CommandStarts`] / [`COMMAND`] -- messages starting with a bot command.
//! - [`CommandFilter`] -- configurable: `only_start` controls whether the
//!   command must be at offset 0 or can appear anywhere in the text.

use crate::filters::base::{effective_message_val, to_value, Filter, FilterResult, Update};

// ---------------------------------------------------------------------------
// CommandFilter
// ---------------------------------------------------------------------------

/// Filters messages containing a `/command` entity.
///
/// By default only matches when the command is at offset 0 (`only_start = true`).
/// Set `only_start` to `false` to match commands anywhere in the text.
pub struct CommandFilter {
    only_start: bool,
    display: &'static str,
}

impl CommandFilter {
    /// Command must be at the start of the message (default).
    pub fn starts() -> Self {
        Self {
            only_start: true,
            display: "filters.COMMAND",
        }
    }

    /// Command can appear anywhere in the message.
    pub fn anywhere() -> Self {
        Self {
            only_start: false,
            display: "filters.Command(false)",
        }
    }
}

impl Filter for CommandFilter {
    fn check_update(&self, update: &Update) -> FilterResult { let __v = to_value(update);
        let v = to_value(update);
        let msg = match effective_message_val(&v) {
            Some(m) => m,
            None => return FilterResult::NoMatch,
        };
        let entities = match msg.get("entities").and_then(|v| v.as_array()) {
            Some(arr) => arr,
            None => return FilterResult::NoMatch,
        };

        let matched = if self.only_start {
            // Only the first entity matters; it must be a bot_command at offset 0.
            entities.first().map_or(false, |e| {
                e.get("type").and_then(|v| v.as_str()) == Some("bot_command")
                    && e.get("offset").and_then(|v| v.as_u64()) == Some(0)
            })
        } else {
            entities.iter().any(|e| {
                e.get("type").and_then(|v| v.as_str()) == Some("bot_command")
            })
        };

        if matched {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        self.display
    }
}

/// Constant shortcut -- `filters::COMMAND` (only matches commands at the start).
pub const COMMAND: CommandFilter = CommandFilter {
    only_start: true,
    display: "filters.COMMAND",
};

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn command_update(text: &str, entities: serde_json::Value) -> Update {
        serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": text,
                "entities": entities
            }
        }))
        .unwrap()
    }

    #[test]
    fn command_at_start() {
        let update = command_update(
            "/start hello",
            json!([{"type": "bot_command", "offset": 0, "length": 6}]),
        );
        assert!(COMMAND.check_update(&update).is_match());
    }

    #[test]
    fn command_not_at_start_rejected_by_default() {
        let update = command_update(
            "hello /start",
            json!([{"type": "bot_command", "offset": 6, "length": 6}]),
        );
        assert!(!COMMAND.check_update(&update).is_match());
    }

    #[test]
    fn command_anywhere_accepts_mid_text() {
        let f = CommandFilter::anywhere();
        let update = command_update(
            "hello /start",
            json!([{"type": "bot_command", "offset": 6, "length": 6}]),
        );
        assert!(f.check_update(&update).is_match());
    }

    #[test]
    fn no_entities_no_match() {
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": "just text"
            }
        }))
        .unwrap();
        assert!(!COMMAND.check_update(&update).is_match());
    }

    #[test]
    fn wrong_entity_type() {
        let update = command_update(
            "@mention",
            json!([{"type": "mention", "offset": 0, "length": 8}]),
        );
        assert!(!COMMAND.check_update(&update).is_match());
    }
}
