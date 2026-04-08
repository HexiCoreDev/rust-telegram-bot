use serde::{Deserialize, Serialize};

use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;
use crate::types::message_entity::MessageEntity;

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InlineQueryResultMpeg4Gif {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL for the MP4 file.
    pub mpeg4_url: String,

    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result.
    pub thumbnail_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<i32>,

    /// Video duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<i64>,

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

impl_new!(InlineQueryResultMpeg4Gif {
    id: String,
    mpeg4_url: String,
    thumbnail_url: String,
});
