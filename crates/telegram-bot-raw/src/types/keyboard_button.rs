
use serde::{Deserialize, Serialize};

use super::keyboard_button_poll_type::KeyboardButtonPollType;
use super::keyboard_button_request::{
    KeyboardButtonRequestChat, KeyboardButtonRequestManagedBot, KeyboardButtonRequestUsers,
};
use super::web_app_info::WebAppInfo;

/// One button of a reply keyboard.
///
/// For simple text buttons `text` is the only field needed; the other optional
/// fields are mutually exclusive and enable richer interactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButton {
    /// Label text shown on the button.
    pub text: String,

    /// If `true`, the user's phone number is sent when the button is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,

    /// If `true`, the user's current location is sent when the button is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,

    /// If set, the user is asked to create a poll and send it to the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,

    /// If set, the described Web App is launched when the button is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,

    /// If set, pressing the button opens a user-selection list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_users: Option<KeyboardButtonRequestUsers>,

    /// If set, pressing the button opens a chat-selection list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<KeyboardButtonRequestChat>,

    /// If specified, pressing the button will ask the user to create and share a bot
    /// that will be managed by the current bot. Available in private chats only.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_managed_bot: Option<KeyboardButtonRequestManagedBot>,

    /// Visual style for the button: `"primary"`, `"success"`, or `"danger"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,

    /// Unique identifier of the custom emoji shown before the button text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}
