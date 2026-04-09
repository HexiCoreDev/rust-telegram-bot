use serde::{Deserialize, Serialize};

use crate::types::copy_text_button::CopyTextButton;
use crate::types::games::callback_game::CallbackGame;
use crate::types::login_url::LoginUrl;
use crate::types::switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat;
use crate::types::web_app_info::WebAppInfo;

/// One button of an inline keyboard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InlineKeyboardButton {
    /// Label text on the button.
    pub text: String,

    /// HTTP or tg:// url to be opened when the button is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// An HTTPS URL used to automatically authorize the user via the Login Widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,

    /// Data to be sent in a callback query to the bot when the button is pressed, 1-64 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,

    /// Description of the Web App that will be launched when the user presses the button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,

    /// Pressing the button will prompt the user to select one of their chats and send inline query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,

    /// Pressing the button will insert the bot's username and inline query in the current chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,

    /// Pressing the button will prompt the user to select one of their chats of the specified type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,

    /// Description of the button that copies the specified text to the clipboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_text: Option<CopyTextButton>,

    /// Description of the game that will be launched when the user presses the button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<CallbackGame>,

    /// Specify `true` to send a Pay button. Must be the first button in the first row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<bool>,

    /// Optional style of the button (e.g. color or shape).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,

    /// Custom emoji identifier for the button icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

impl InlineKeyboardButton {
    /// Create a button that sends callback data when pressed.
    ///
    /// This is the most common button type for interactive inline keyboards.
    pub fn callback(text: impl Into<String>, callback_data: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            callback_data: Some(callback_data.into()),
            ..Default::default()
        }
    }

    /// Create a button that opens a URL when pressed.
    pub fn url(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: Some(url.into()),
            ..Default::default()
        }
    }

    /// Create a button that switches to inline query mode in any chat.
    pub fn switch_inline_query(text: impl Into<String>, query: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            switch_inline_query: Some(query.into()),
            ..Default::default()
        }
    }

    /// Create a button that switches to inline query mode in the current chat.
    pub fn switch_inline_query_current_chat(
        text: impl Into<String>,
        query: impl Into<String>,
    ) -> Self {
        Self {
            text: text.into(),
            switch_inline_query_current_chat: Some(query.into()),
            ..Default::default()
        }
    }

    /// Create a button that opens a Web App when pressed.
    pub fn web_app(text: impl Into<String>, web_app_url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            web_app: Some(WebAppInfo {
                url: web_app_url.into(),
            }),
            ..Default::default()
        }
    }

    /// Create a Pay button. Must always be the first button in the first row.
    pub fn pay(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            pay: Some(true),
            ..Default::default()
        }
    }
}
