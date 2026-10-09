use serde::{Deserialize, Serialize};

use super::input_rich_block::{InputRichBlock, InputRichMessageMedia};

// ---------------------------------------------------------------------------
// InputRichMessage
// ---------------------------------------------------------------------------

/// A rich message to be sent: explicit blocks, or HTML / Markdown markup.
///
/// Exactly one of `blocks`, `html` or `markdown` must be used. When `html` or
/// `markdown` is used, media referenced via `tg://photo?id=` style links is
/// supplied through `media`.
///
/// Corresponds to the Bot API [`InputRichMessage`](https://core.telegram.org/bots/api#inputrichmessage) type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputRichMessage {
    /// Content described as a list of blocks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<InputRichBlock>>,

    /// Content described using HTML formatting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,

    /// Content described using Markdown formatting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,

    /// Media referenced from `html` or `markdown` through `tg://` links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<InputRichMessageMedia>>,

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

    /// Creates a rich message from an explicit block list.
    pub fn blocks(blocks: Vec<InputRichBlock>) -> Self {
        Self {
            blocks: Some(blocks),
            ..Default::default()
        }
    }

    /// Sets the media referenced from `html` or `markdown` through `tg://` links.
    pub fn media(mut self, media: Vec<InputRichMessageMedia>) -> Self {
        self.media = Some(media);
        self
    }

    /// Shows the rich message right-to-left.
    pub fn rtl(mut self) -> Self {
        self.is_rtl = Some(true);
        self
    }

    /// Skips automatic entity detection (URLs, mentions, hashtags, commands, phones).
    pub fn skip_entity_detection(mut self) -> Self {
        self.skip_entity_detection = Some(true);
        self
    }
}
