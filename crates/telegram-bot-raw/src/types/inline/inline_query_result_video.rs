use serde::{Deserialize, Serialize};

use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;
use crate::types::message_entity::MessageEntity;

/// Represents a link to a page containing an embedded video player or a video file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
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

    /// Caption of the video to be sent, 0-1024 characters after entities parsing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Mode for parsing entities in the video caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// List of special entities that appear in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Video width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<i32>,

    /// Video height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<i32>,

    /// Video duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<i64>,

    /// Short description of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    /// Pass `true` if the caption must be shown above the message media.
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
