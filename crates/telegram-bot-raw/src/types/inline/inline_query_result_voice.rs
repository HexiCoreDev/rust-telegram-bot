use serde::{Deserialize, Serialize};

use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;
use crate::types::message_entity::MessageEntity;

/// Represents a link to a voice recording in an .ogg container encoded with OPUS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
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

    /// Caption of the voice message, 0-1024 characters after entities parsing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Mode for parsing entities in the voice message caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// List of special entities that appear in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the voice recording.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

impl_new!(InlineQueryResultVoice {
    id: String,
    voice_url: String,
    title: String,
});
