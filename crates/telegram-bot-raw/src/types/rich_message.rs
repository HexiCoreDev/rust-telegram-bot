use serde::{Deserialize, Serialize};

use super::copy_text_button::CopyTextButton;
use super::inline::inline_keyboard_button::DisabledButton;
use super::login_url::LoginUrl;
use super::rich_block::RichBlock;
use super::rich_text::RichText;
use super::switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat;
use super::web_app_info::WebAppInfo;

// ---------------------------------------------------------------------------
// RichMessage
// ---------------------------------------------------------------------------

/// A rich formatted message.
///
/// Corresponds to the Bot API [`RichMessage`](https://core.telegram.org/bots/api#richmessage) type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct RichMessage {
    /// Content of the message.
    pub blocks: Vec<RichBlock>,

    /// `true` if the rich message must be shown right-to-left.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rtl: Option<bool>,
}

impl RichMessage {
    /// Creates a rich message from a list of blocks.
    pub fn new(blocks: Vec<RichBlock>) -> Self {
        Self {
            blocks,
            is_rtl: None,
        }
    }
}

// ---------------------------------------------------------------------------
// RichMessageButton
// ---------------------------------------------------------------------------

/// A button in a rich message. Exactly one of the fields other than `text`
/// and `style` must be used to specify the button type.
///
/// Corresponds to the Bot API [`RichMessageButton`](https://core.telegram.org/bots/api#richmessagebutton) type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichMessageButton {
    /// Text of the button: plain text, custom emoji and date-time entities only.
    pub text: RichText,
    /// Style of the button: `"danger"`, `"success"`, `"primary"` or `"link"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    /// HTTP or tg:// URL to be opened when the button is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Data to be sent in a callback query when the button is pressed, 1-64 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    /// Description of the Web App to launch when the button is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
    /// HTTPS URL used to automatically authorize the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,
    /// If set, pressing the button prompts the user to select a chat and insert
    /// the bot's username with the inline query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,
    /// If set, pressing the button inserts the bot's username and inline query
    /// in the current chat's input field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
    /// If set, pressing the button prompts the user to select a chat of the
    /// specified type for an inline query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    /// Button that copies the specified text to the clipboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_text: Option<CopyTextButton>,
    /// If set, the button is disabled and does nothing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<DisabledButton>,
}
