use serde::{Deserialize, Serialize};

use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;
use crate::types::message_entity::MessageEntity;

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the
/// Telegram servers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InlineQueryResultCachedMpeg4Gif {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid file identifier for the MP4 file.
    pub mpeg4_file_id: String,

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

impl_new!(InlineQueryResultCachedMpeg4Gif {
    id: String,
    mpeg4_file_id: String,
});
