use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;

/// Represents a link to an article or web page.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultArticle {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// Title of the result.
    pub title: String,

    /// Content of the message to be sent.
    pub input_message_content: InputMessageContent,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i32>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
