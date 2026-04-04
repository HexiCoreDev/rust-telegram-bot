use std::collections::HashMap;

use serde_json::Value;

/// JSON dictionary type used for Telegram API responses.
pub type JsonDict = HashMap<String, Value>;

/// Markdown parse mode version (1 or 2).
pub type MarkdownVersion = u8;

/// Valid poll correct-option IDs (0–9).
pub type CorrectOptionId = u8;

/// HTTP versions accepted by the Telegram Bot API client.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpVersion {
    /// HTTP/1.1
    Http1_1,
    /// HTTP/2
    Http2_0,
}
