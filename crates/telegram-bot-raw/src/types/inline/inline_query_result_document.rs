use serde::{Deserialize, Serialize};

use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;
use crate::types::message_entity::MessageEntity;

/// Represents a link to a file. Currently, only .PDF and .ZIP files can be sent using this method.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InlineQueryResultDocument {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL for the file.
    pub document_url: String,

    /// Title for the result.
    pub title: String,

    /// Mime type of the content of the file, either "application/pdf" or "application/zip".
    pub mime_type: String,

    /// Caption of the document to be sent, 0-1024 characters after entities parsing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Mode for parsing entities in the document caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// List of special entities that appear in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Short description of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    /// URL of the thumbnail for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,

    /// Thumbnail width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i32>,

    /// Thumbnail height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i32>,
}

impl_new!(InlineQueryResultDocument {
    id: String,
    title: String,
    document_url: String,
    mime_type: String,
});
