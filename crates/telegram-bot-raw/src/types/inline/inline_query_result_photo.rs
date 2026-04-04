use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;
use crate::types::message_entity::MessageEntity;

/// Represents a link to a photo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InlineQueryResultPhoto {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL of the photo. Photo must be in JPEG format. Photo size must not exceed 5MB.
    pub photo_url: String,

    /// URL of the thumbnail for the photo.
    pub thumbnail_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

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

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
