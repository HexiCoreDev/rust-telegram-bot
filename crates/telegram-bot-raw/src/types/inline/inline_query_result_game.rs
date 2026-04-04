use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::inline_keyboard_markup::InlineKeyboardMarkup;

/// Represents a Game.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InlineQueryResultGame {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// Short name of the game.
    pub game_short_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
