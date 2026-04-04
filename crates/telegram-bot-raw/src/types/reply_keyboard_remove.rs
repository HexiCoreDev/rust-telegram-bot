use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Upon receiving a message with this object, Telegram clients will remove the current custom
/// keyboard and display the default letter-keyboard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard. Always `true`.
    pub remove_keyboard: bool,

    /// `true` to remove the keyboard for specific users only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
