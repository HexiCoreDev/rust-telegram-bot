use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::keyboard_button::KeyboardButton;

/// Custom keyboard with reply options. Not supported in channels and for messages sent on behalf
/// of a Telegram Business account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
