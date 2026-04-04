use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::inline_keyboard_button::InlineKeyboardButton;

/// An inline keyboard that appears right next to the message it belongs to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an array of `InlineKeyboardButton` objects.
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
