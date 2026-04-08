
use serde::{Deserialize, Serialize};

use super::web_app_info::WebAppInfo;

/// Payload for `MenuButtonCommands` — no additional fields beyond the tag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MenuButtonCommandsData {
}

/// Payload for `MenuButtonWebApp`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MenuButtonWebAppData {
    /// Label shown on the button.
    pub text: String,

    /// The Web App that will be launched when the button is pressed.
    pub web_app: WebAppInfo,
}

/// Payload for `MenuButtonDefault` — no additional fields beyond the tag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MenuButtonDefaultData {
}

/// The bot's menu button in a private chat.
///
/// Serialized with a `"type"` tag that selects the variant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MenuButton {
    /// Opens the bot's list of commands.
    Commands(MenuButtonCommandsData),

    /// Launches a Web App.
    WebApp(MenuButtonWebAppData),

    /// No specific value set; the default button (command list) is used.
    Default(MenuButtonDefaultData),
}
