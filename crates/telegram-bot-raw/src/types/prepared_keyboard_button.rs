use serde::{Deserialize, Serialize};

/// A keyboard button to be used by a user of a Mini App.
///
/// Corresponds to the Bot API [`PreparedKeyboardButton`](https://core.telegram.org/bots/api#preparedkeyboardbutton) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PreparedKeyboardButton {
    /// Unique identifier of the keyboard button.
    pub id: String,
}
