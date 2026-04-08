use serde::{Deserialize, Serialize};

/// Data sent from a Web App to the bot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebAppData {
    /// The data payload. A bad client can send arbitrary data in this field.
    pub data: String,

    /// Text of the keyboard button from which the Web App was opened.
    pub button_text: String,
}
