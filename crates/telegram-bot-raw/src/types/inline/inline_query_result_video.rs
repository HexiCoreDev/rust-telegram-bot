use serde::{Deserialize, Serialize};

use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;
use crate::types::message_entity::MessageEntity;

/// Represents a link to a page containing an embedded video player or a video file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InlineQueryResultVideo {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL for the embedded video player or video file.
    pub video_url: String,

    /// Mime type of the content of video url, "text/html" or "video/mp4".
    pub mime_type: String,

    /// URL of the thumbnail (JPEG only) for the video.
    pub thumbnail_url: String,

    /// Title for the result.
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<i32>,

    /// Video duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
}

impl_new!(InlineQueryResultVideo {
    id: String,
    video_url: String,
    mime_type: String,
    thumbnail_url: String,
    title: String,
});
