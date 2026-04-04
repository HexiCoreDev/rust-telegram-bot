use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;

/// Represents a contact with a phone number.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InlineQueryResultContact {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// Contact's phone number.
    pub phone_number: String,

    /// Contact's first name.
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// Additional data about the contact in the form of a vCard, 0-2048 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i32>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
