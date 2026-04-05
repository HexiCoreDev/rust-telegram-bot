//! Regex filter for message text.
//!
//! Searches the message `text` with [`regex::Regex`] and returns captured
//! groups as [`FilterResult::MatchWithData`].

use std::collections::HashMap;

use crate::filters::base::{effective_message_val, to_value, Filter, FilterResult, Update};

/// Filters messages whose `text` matches a compiled regex pattern.
///
/// Returns [`FilterResult::MatchWithData`] with all capture groups collected
/// under the `"matches"` key.
pub struct RegexFilter {
    pattern: regex::Regex,
    display: String,
}

impl RegexFilter {
    /// Compile a pattern string into a regex filter.
    ///
    /// # Panics
    ///
    /// Panics if the pattern is not a valid regex.
    pub fn new(pattern: &str) -> Self {
        let re = regex::Regex::new(pattern).expect("invalid regex pattern");
        let display = format!("filters.Regex({})", pattern);
        Self {
            pattern: re,
            display,
        }
    }

    /// Create from a pre-compiled [`regex::Regex`].
    pub fn from_regex(re: regex::Regex) -> Self {
        let display = format!("filters.Regex({})", re.as_str());
        Self {
            pattern: re,
            display,
        }
    }

    /// Return a reference to the inner compiled pattern.
    pub fn pattern(&self) -> &regex::Regex {
        &self.pattern
    }
}

impl Filter for RegexFilter {
    fn check_update(&self, update: &Update) -> FilterResult { let __v = to_value(update);
        let text = match effective_message_val(&__v)
            .and_then(|m| m.get("text"))
            .and_then(|v| v.as_str())
        {
            Some(t) => t,
            None => return FilterResult::NoMatch,
        };

        match self.pattern.captures(text) {
            Some(caps) => {
                let captures_vec: Vec<String> = caps
                    .iter()
                    .filter_map(|m| m.map(|mat| mat.as_str().to_owned()))
                    .collect();
                FilterResult::MatchWithData(HashMap::from([(
                    "matches".to_owned(),
                    captures_vec,
                )]))
            }
            None => FilterResult::NoMatch,
        }
    }

    fn name(&self) -> &str {
        &self.display
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn text_update(text: &str) -> Update {
        serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": text
            }
        })).unwrap()
    }

    #[test]
    fn regex_matches() {
        let f = RegexFilter::new(r"hel+o");
        assert!(f.check_update(&text_update("say hello world")).is_match());
    }

    #[test]
    fn regex_no_match() {
        let f = RegexFilter::new(r"^goodbye$");
        assert!(!f.check_update(&text_update("hello")).is_match());
    }

    #[test]
    fn regex_no_text() {
        let f = RegexFilter::new(r".");
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"}
            }
        })).unwrap();
        assert!(!f.check_update(&update).is_match());
    }

    #[test]
    fn regex_from_compiled() {
        let re = regex::Regex::new(r"(?i)help").unwrap();
        let f = RegexFilter::from_regex(re);
        assert!(f.check_update(&text_update("HELP me")).is_match());
    }

    #[test]
    fn regex_returns_captures() {
        let f = RegexFilter::new(r"(\w+)\s(\w+)");
        let result = f.check_update(&text_update("hello world"));
        match result {
            FilterResult::MatchWithData(data) => {
                let matches = data.get("matches").unwrap();
                assert_eq!(matches[0], "hello world"); // full match
                assert_eq!(matches[1], "hello");
                assert_eq!(matches[2], "world");
            }
            _ => panic!("expected MatchWithData"),
        }
    }
}
