
use serde::{Deserialize, Serialize};

use super::chat_administrator_rights::ChatAdministratorRights;

/// Criteria for requesting one or more users via a `KeyboardButton`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonRequestUsers {
    /// Signed 32-bit identifier of the request; unique within the message.
    pub request_id: i32,

    /// `true` to request a bot; `false` to request a regular user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,

    /// `true` to request a Telegram Premium user; `false` to request a non-premium user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,

    /// Maximum number of users to select (1-10, defaults to 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<i32>,

    /// `true` to request the users' first and last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_name: Option<bool>,

    /// `true` to request the users' username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,

    /// `true` to request the users' photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

/// Criteria for requesting a chat via a `KeyboardButton`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonRequestChat {
    /// Signed 32-bit identifier of the request; unique within the message.
    pub request_id: i32,

    /// `true` to request a channel; `false` to request a group or supergroup.
    pub chat_is_channel: bool,

    /// `true` to request a forum supergroup; `false` to request a non-forum chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<bool>,

    /// `true` to request a chat with a username; `false` to request one without.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<bool>,

    /// `true` to request a chat owned by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<bool>,

    /// Required administrator rights of the user in the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<ChatAdministratorRights>,

    /// Required administrator rights of the bot in the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<ChatAdministratorRights>,

    /// `true` to require the bot to be a member of the requested chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_is_member: Option<bool>,

    /// `true` to request the chat's title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_title: Option<bool>,

    /// `true` to request the chat's username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,

    /// `true` to request the chat's photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

/// Parameters for the creation of a managed bot via a `KeyboardButton`.
///
/// Information about the created bot will be shared with the bot using the
/// `managed_bot` update and a `Message` with the `managed_bot_created` field.
///
/// Corresponds to the Bot API [`KeyboardButtonRequestManagedBot`](https://core.telegram.org/bots/api#keyboardbuttonrequestmanagedbot) object.
///
/// Added in Bot API 9.6.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonRequestManagedBot {
    /// Signed 32-bit identifier of the request. Must be unique within the message.
    pub request_id: i32,

    /// Suggested name for the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_name: Option<String>,

    /// Suggested username for the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_username: Option<String>,
}
