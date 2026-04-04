use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;

/// Represents a link to a sticker stored on the Telegram servers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InlineQueryResultCachedSticker {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid file identifier of the sticker.
    pub sticker_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
