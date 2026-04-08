
use serde::{Deserialize, Serialize};

use crate::types::copy_text_button::CopyTextButton;
use crate::types::games::callback_game::CallbackGame;
use crate::types::login_url::LoginUrl;
use crate::types::switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat;
use crate::types::web_app_info::WebAppInfo;

/// One button of an inline keyboard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
