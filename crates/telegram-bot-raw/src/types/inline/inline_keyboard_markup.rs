use serde::{Deserialize, Serialize};

use super::inline_keyboard_button::InlineKeyboardButton;

/// An inline keyboard that appears right next to the message it belongs to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an array of `InlineKeyboardButton` objects.
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    /// Create a new inline keyboard from rows of buttons.
    pub fn new(rows: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self {
            inline_keyboard: rows,
        }
    }

    /// Create an inline keyboard with a single button.
    pub fn from_button(button: InlineKeyboardButton) -> Self {
        Self {
            inline_keyboard: vec![vec![button]],
        }
    }

    /// Create an inline keyboard with a single row of buttons.
    pub fn from_row(buttons: Vec<InlineKeyboardButton>) -> Self {
        Self {
            inline_keyboard: vec![buttons],
        }
    }

    /// Create an inline keyboard where each button is on its own row (vertical layout).
    pub fn from_column(buttons: Vec<InlineKeyboardButton>) -> Self {
        Self {
            inline_keyboard: buttons.into_iter().map(|b| vec![b]).collect(),
        }
    }
}
