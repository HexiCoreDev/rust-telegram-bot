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

    /// Width of the GIF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<i32>,

    /// Height of the GIF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<i32>,

    /// Duration of the GIF in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<i64>,

    /// MIME type of the thumbnail, must be one of "image/jpeg", "image/gif", or "video/mp4".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_mime_type: Option<String>,

    /// Title for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Caption of the GIF file to be sent, 0-1024 characters after entities parsing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Mode for parsing entities in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// List of special entities that appear in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the GIF animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    /// Pass `true` if the caption must be shown above the message media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
}

impl_new!(InlineQueryResultGif {
    id: String,
    gif_url: String,
    thumbnail_url: String,
});
