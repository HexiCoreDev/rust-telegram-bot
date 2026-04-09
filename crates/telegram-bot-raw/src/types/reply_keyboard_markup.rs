use serde::{Deserialize, Serialize};

use super::keyboard_button::KeyboardButton;

/// Custom keyboard with reply options. Not supported in channels and for messages sent on behalf
/// of a Telegram Business account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an array of `KeyboardButton` objects.
    pub keyboard: Vec<Vec<KeyboardButton>>,

    /// `true` to request clients to resize the keyboard vertically for optimal fit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,

    /// `true` to request clients to hide the keyboard as soon as it has been used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,

    /// `true` to show the keyboard to specific users only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,

    /// Placeholder shown in the input field when the keyboard is active; 1-64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,

    /// `true` to always show the keyboard when the regular keyboard is hidden.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_persistent: Option<bool>,
}

impl ReplyKeyboardMarkup {
    /// Create a new reply keyboard from rows of buttons.
    pub fn new(keyboard: Vec<Vec<KeyboardButton>>) -> Self {
        Self {
            keyboard,
            ..Default::default()
        }
    }

    /// Make the keyboard disappear after a single press.
    pub fn one_time(mut self) -> Self {
        self.one_time_keyboard = Some(true);
        self
    }

    /// Request clients to resize the keyboard vertically for optimal fit.
    pub fn resize(mut self) -> Self {
        self.resize_keyboard = Some(true);
        self
    }

    /// Always show the keyboard when the regular keyboard is hidden.
    pub fn persistent(mut self) -> Self {
        self.is_persistent = Some(true);
        self
    }

    /// Set a placeholder shown in the input field when the keyboard is active.
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.input_field_placeholder = Some(text.into());
        self
    }

    /// Show the keyboard to specific users only.
    pub fn selective(mut self) -> Self {
        self.selective = Some(true);
        self
    }
}
