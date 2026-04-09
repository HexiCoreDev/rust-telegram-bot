use serde::{Deserialize, Serialize};

use super::inline_keyboard_markup::InlineKeyboardMarkup;

/// Represents a Game.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InlineQueryResultGame {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// Short name of the game.
    pub game_short_name: String,

    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl_new!(InlineQueryResultGame {
    id: String,
    game_short_name: String,
});
