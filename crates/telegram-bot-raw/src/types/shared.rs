use serde::{Deserialize, Serialize};

use super::files::photo_size::PhotoSize;

/// Information about a user shared with the bot via a `KeyboardButtonRequestUsers` button.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SharedUser {
    /// Identifier of the shared user.
    pub user_id: i64,

    /// First name of the user, if requested by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// Last name of the user, if requested by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// Username of the user, if requested by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Available sizes of the user's photo, if requested by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}

/// Information about users shared with the bot via a `KeyboardButtonRequestUsers` button.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UsersShared {
    /// Identifier of the request.
    pub request_id: i64,

    /// Information about the users shared with the bot.
    pub users: Vec<SharedUser>,
}

/// Information about a chat shared with the bot via a `KeyboardButtonRequestChat` button.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatShared {
    /// Identifier of the request.
    pub request_id: i64,

    /// Identifier of the shared chat.
    pub chat_id: i64,

    /// Title of the chat, if requested by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Username of the chat, if requested by the bot and available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Available sizes of the chat photo, if requested by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}
