use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;
use crate::types::message_entity::MessageEntity;

/// Represents a link to a voice recording in an .ogg container encoded with OPUS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InlineQueryResultVoice {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL for the voice recording.
    pub voice_url: String,

    /// Recording title.
    pub title: String,

    /// Recording duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<i64>,

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

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
