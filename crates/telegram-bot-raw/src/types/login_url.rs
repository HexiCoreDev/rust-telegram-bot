use serde::{Deserialize, Serialize};

/// An inline keyboard button parameter used to automatically authorize a user via Telegram Login.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LoginUrl {
    /// HTTPS URL opened with user authorization data appended to the query string.
    pub url: String,

    /// Replacement text for the button in forwarded messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<String>,

    /// Username of the bot used for user authorization (defaults to current bot).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,

    /// If `true`, the bot is permitted to send messages to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<bool>,
}

impl_new!(LoginUrl { url: String });
