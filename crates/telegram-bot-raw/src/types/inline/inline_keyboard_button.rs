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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_text: Option<CopyTextButton>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<CallbackGame>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,

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
