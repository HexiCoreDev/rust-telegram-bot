
use serde::{Deserialize, Serialize};

use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;
use crate::types::message_entity::MessageEntity;

/// Represents a link to an mp3 audio file stored on the Telegram servers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InlineQueryResultCachedAudio {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid file identifier for the audio file.
    pub audio_file_id: String,

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
}

impl_new!(InlineQueryResultCachedAudio {
    id: String,
    audio_file_id: String,
});
