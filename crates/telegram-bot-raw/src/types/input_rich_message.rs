use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// InputRichMessage
// ---------------------------------------------------------------------------

/// A rich message to be sent, described with HTML or Markdown markup.
///
/// Exactly one of `html` or `markdown` must be used.
///
/// Corresponds to the Bot API [`InputRichMessage`](https://core.telegram.org/bots/api#inputrichmessage) type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputRichMessage {
    /// Content described using HTML formatting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,

    /// Content described using Markdown formatting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,

    /// `true` if the rich message must be shown right-to-left.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rtl: Option<bool>,

    /// `true` to skip automatic detection of entities (URLs, mentions, hashtags,
    /// bot commands, phone numbers) in the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_entity_detection: Option<bool>,
}

impl InputRichMessage {
    /// Creates a rich message from HTML markup.
    pub fn html(html: impl Into<String>) -> Self {
        Self {
            html: Some(html.into()),
            ..Default::default()
        }
    }

    /// Creates a rich message from Markdown markup.
    pub fn markdown(markdown: impl Into<String>) -> Self {
        Self {
            markdown: Some(markdown.into()),
            ..Default::default()
        }
    }
}
