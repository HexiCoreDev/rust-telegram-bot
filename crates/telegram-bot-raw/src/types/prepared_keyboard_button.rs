use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A keyboard button to be used by a user of a Mini App.
///
/// Corresponds to the Bot API [`PreparedKeyboardButton`](https://core.telegram.org/bots/api#preparedkeyboardbutton) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreparedKeyboardButton {
    /// Unique identifier of the keyboard button.
    pub id: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
