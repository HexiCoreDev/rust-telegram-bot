use serde::{Deserialize, Serialize};

use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;
use crate::types::message_entity::MessageEntity;

/// Represents a link to an animated GIF file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InlineQueryResultGif {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL for the GIF file.
    pub gif_url: String,

    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result.
    pub thumbnail_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<i32>,

    /// Duration of the GIF in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
}

impl_new!(InlineQueryResultGif {
    id: String,
    gif_url: String,
    thumbnail_url: String,
});
