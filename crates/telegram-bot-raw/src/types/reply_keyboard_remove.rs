
use serde::{Deserialize, Serialize};

/// Upon receiving a message with this object, Telegram clients will remove the current custom
/// keyboard and display the default letter-keyboard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard. Always `true`.
    pub remove_keyboard: bool,

    /// `true` to remove the keyboard for specific users only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

impl ReplyKeyboardRemove {
    /// Creates a new `ReplyKeyboardRemove` with `remove_keyboard` set to `true`.
    pub fn new() -> Self {
        Self {
            remove_keyboard: true,
            ..Default::default()
        }
    }

    /// Remove the keyboard for specific users only.
    pub fn selective(mut self) -> Self {
        self.selective = Some(true);
        self
    }
}
