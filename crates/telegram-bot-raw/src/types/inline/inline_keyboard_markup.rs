use serde::{Deserialize, Serialize};

use super::inline_keyboard_button::InlineKeyboardButton;

/// An inline keyboard that appears right next to the message it belongs to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an array of `InlineKeyboardButton` objects.
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,

    /// Pass `true` to show the reply interface to the user, as if they had
    /// manually selected the bot's message and tapped 'Reply'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_reply: Option<bool>,
}

impl InlineKeyboardMarkup {
    /// Create a new inline keyboard from rows of buttons.
    pub fn new(rows: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self {
            inline_keyboard: rows,
            force_reply: None,
        }
    }

    /// Create an inline keyboard with a single button.
    pub fn from_button(button: InlineKeyboardButton) -> Self {
        Self {
            inline_keyboard: vec![vec![button]],
            force_reply: None,
        }
    }

    /// Create an inline keyboard with a single row of buttons.
    pub fn from_row(buttons: Vec<InlineKeyboardButton>) -> Self {
        Self {
            inline_keyboard: vec![buttons],
            force_reply: None,
        }
    }

    /// Create an inline keyboard where each button is on its own row (vertical layout).
    pub fn from_column(buttons: Vec<InlineKeyboardButton>) -> Self {
        Self {
            inline_keyboard: buttons.into_iter().map(|b| vec![b]).collect(),
            force_reply: None,
        }
    }
}
