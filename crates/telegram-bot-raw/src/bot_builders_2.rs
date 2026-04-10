//! Builder pattern for Admin, Forum, and Sticker Telegram Bot API methods.
//!
//! These builders follow the same pattern as those in [`bot_builders`](crate::bot_builders):
//! 1. Created via the corresponding `Bot` factory method with only required parameters.
//! 2. Chained setter calls for optional parameters.
//! 3. `.await?` (or `.send().await?`) to execute the request.

#![allow(clippy::too_many_arguments)]

use crate::bot::{Bot, ChatId};
use crate::error::Result;
use crate::request::request_parameter::{InputFileRef, RequestParameter};
use crate::types::{
    bot_command, bot_command_scope, bot_description, bot_name, chat_administrator_rights, files,
    forum_topic, menu_button,
};
use serde::Serialize;

// ---------------------------------------------------------------------------
// Private helpers (duplicated from bot_builders.rs since those are private)
// ---------------------------------------------------------------------------

fn push_opt_file(
    params: &mut Vec<RequestParameter>,
    name: &'static str,
    val: Option<files::input_file::InputFile>,
) {
    if let Some(f) = val {
        params.push(input_file_param(name, f));
    }
}

fn input_file_param(name: &'static str, file: files::input_file::InputFile) -> RequestParameter {
    match file {
        files::input_file::InputFile::FileId(id) => {
            RequestParameter::new(name, serde_json::Value::String(id))
        }
        files::input_file::InputFile::Url(url) => {
            RequestParameter::new(name, serde_json::Value::String(url))
        }
        files::input_file::InputFile::Bytes { filename, data } => {
            let file_ref = InputFileRef {
                attach_name: None,
                bytes: data,
                mime_type: None,
                file_name: Some(filename),
            };
            RequestParameter::file_only(name, file_ref)
        }
        files::input_file::InputFile::Path(path) => {
            let filename = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let path_str = path.to_string_lossy().to_string();
            let file_ref = InputFileRef {
                attach_name: None,
                bytes: Vec::new(),
                mime_type: None,
                file_name: Some(filename),
            };
            RequestParameter {
                name: std::borrow::Cow::Borrowed(name),
                value: Some(serde_json::Value::String(format!(
                    "__filepath__:{path_str}"
                ))),
                input_files: Some(vec![file_ref]),
            }
        }
    }
}

macro_rules! impl_into_future {
    ($builder:ident, $output:ty) => {
        impl<'a> std::future::IntoFuture for $builder<'a> {
            type Output = Result<$output>;
            type IntoFuture =
                std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'a>>;
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(self.send())
            }
        }
    };
}

// =========================================================================
//  ADMIN BUILDERS (13 methods from admin.rs)
// =========================================================================

// =========================================================================
// SetChatMenuButtonBuilder
// =========================================================================

/// Builder for the [`setChatMenuButton`] API method.
#[derive(Serialize)]
pub struct SetChatMenuButtonBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    menu_button: Option<menu_button::MenuButton>,
}

impl<'a> SetChatMenuButtonBuilder<'a> {
    /// Sets the `chat_id` parameter.
    pub fn chat_id(mut self, val: i64) -> Self {
        self.chat_id = Some(val);
        self
    }
    /// Sets the `menu_button` parameter.
    pub fn menu_button(mut self, val: menu_button::MenuButton) -> Self {
        self.menu_button = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("setChatMenuButton", &payload).await
    }
}

impl_into_future!(SetChatMenuButtonBuilder, bool);

// =========================================================================
// GetChatMenuButtonBuilder
// =========================================================================

/// Builder for the [`getChatMenuButton`] API method.
#[derive(Serialize)]
pub struct GetChatMenuButtonBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<i64>,
}

impl<'a> GetChatMenuButtonBuilder<'a> {
    /// Sets the `chat_id` parameter.
    pub fn chat_id(mut self, val: i64) -> Self {
        self.chat_id = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<menu_button::MenuButton> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("getChatMenuButton", &payload).await
    }
}

impl_into_future!(GetChatMenuButtonBuilder, menu_button::MenuButton);

// =========================================================================
// SetMyCommandsBuilder
// =========================================================================

/// Builder for the [`setMyCommands`] API method.
#[derive(Serialize)]
pub struct SetMyCommandsBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    commands: Vec<bot_command::BotCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<bot_command_scope::BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl<'a> SetMyCommandsBuilder<'a> {
    /// Sets the `scope` parameter.
    pub fn scope(mut self, val: bot_command_scope::BotCommandScope) -> Self {
        self.scope = Some(val);
        self
    }
    /// Sets the `language_code` parameter.
    pub fn language_code(mut self, val: impl Into<String>) -> Self {
        self.language_code = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("setMyCommands", &payload).await
    }
}

impl_into_future!(SetMyCommandsBuilder, bool);

// =========================================================================
// GetMyCommandsBuilder
// =========================================================================

/// Builder for the [`getMyCommands`] API method.
#[derive(Serialize)]
pub struct GetMyCommandsBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<bot_command_scope::BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl<'a> GetMyCommandsBuilder<'a> {
    /// Sets the `scope` parameter.
    pub fn scope(mut self, val: bot_command_scope::BotCommandScope) -> Self {
        self.scope = Some(val);
        self
    }
    /// Sets the `language_code` parameter.
    pub fn language_code(mut self, val: impl Into<String>) -> Self {
        self.language_code = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<Vec<bot_command::BotCommand>> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("getMyCommands", &payload).await
    }
}

impl_into_future!(GetMyCommandsBuilder, Vec<bot_command::BotCommand>);

// =========================================================================
// DeleteMyCommandsBuilder
// =========================================================================

/// Builder for the [`deleteMyCommands`] API method.
#[derive(Serialize)]
pub struct DeleteMyCommandsBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<bot_command_scope::BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl<'a> DeleteMyCommandsBuilder<'a> {
    /// Sets the `scope` parameter.
    pub fn scope(mut self, val: bot_command_scope::BotCommandScope) -> Self {
        self.scope = Some(val);
        self
    }
    /// Sets the `language_code` parameter.
    pub fn language_code(mut self, val: impl Into<String>) -> Self {
        self.language_code = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("deleteMyCommands", &payload).await
    }
}

impl_into_future!(DeleteMyCommandsBuilder, bool);

// =========================================================================
// SetMyDefaultAdministratorRightsBuilder
// =========================================================================

/// Builder for the [`setMyDefaultAdministratorRights`] API method.
#[derive(Serialize)]
pub struct SetMyDefaultAdministratorRightsBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    rights: Option<chat_administrator_rights::ChatAdministratorRights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_channels: Option<bool>,
}

impl<'a> SetMyDefaultAdministratorRightsBuilder<'a> {
    /// Sets the `rights` parameter.
    pub fn rights(mut self, val: chat_administrator_rights::ChatAdministratorRights) -> Self {
        self.rights = Some(val);
        self
    }
    /// Sets the `for_channels` parameter.
    pub fn for_channels(mut self, val: bool) -> Self {
        self.for_channels = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("setMyDefaultAdministratorRights", &payload)
            .await
    }
}

impl_into_future!(SetMyDefaultAdministratorRightsBuilder, bool);

// =========================================================================
// GetMyDefaultAdministratorRightsBuilder
// =========================================================================

/// Builder for the [`getMyDefaultAdministratorRights`] API method.
#[derive(Serialize)]
pub struct GetMyDefaultAdministratorRightsBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_channels: Option<bool>,
}

impl<'a> GetMyDefaultAdministratorRightsBuilder<'a> {
    /// Sets the `for_channels` parameter.
    pub fn for_channels(mut self, val: bool) -> Self {
        self.for_channels = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<chat_administrator_rights::ChatAdministratorRights> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("getMyDefaultAdministratorRights", &payload)
            .await
    }
}

impl_into_future!(
    GetMyDefaultAdministratorRightsBuilder,
    chat_administrator_rights::ChatAdministratorRights
);

// =========================================================================
// SetMyDescriptionBuilder
// =========================================================================

/// Builder for the [`setMyDescription`] API method.
#[derive(Serialize)]
pub struct SetMyDescriptionBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl<'a> SetMyDescriptionBuilder<'a> {
    /// Sets the `description` parameter.
    pub fn description(mut self, val: impl Into<String>) -> Self {
        self.description = Some(val.into());
        self
    }
    /// Sets the `language_code` parameter.
    pub fn language_code(mut self, val: impl Into<String>) -> Self {
        self.language_code = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("setMyDescription", &payload).await
    }
}

impl_into_future!(SetMyDescriptionBuilder, bool);

// =========================================================================
// GetMyDescriptionBuilder
// =========================================================================

/// Builder for the [`getMyDescription`] API method.
#[derive(Serialize)]
pub struct GetMyDescriptionBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl<'a> GetMyDescriptionBuilder<'a> {
    /// Sets the `language_code` parameter.
    pub fn language_code(mut self, val: impl Into<String>) -> Self {
        self.language_code = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bot_description::BotDescription> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("getMyDescription", &payload).await
    }
}

impl_into_future!(GetMyDescriptionBuilder, bot_description::BotDescription);

// =========================================================================
// SetMyShortDescriptionBuilder
// =========================================================================

/// Builder for the [`setMyShortDescription`] API method.
#[derive(Serialize)]
pub struct SetMyShortDescriptionBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl<'a> SetMyShortDescriptionBuilder<'a> {
    /// Sets the `short_description` parameter.
    pub fn short_description(mut self, val: impl Into<String>) -> Self {
        self.short_description = Some(val.into());
        self
    }
    /// Sets the `language_code` parameter.
    pub fn language_code(mut self, val: impl Into<String>) -> Self {
        self.language_code = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("setMyShortDescription", &payload)
            .await
    }
}

impl_into_future!(SetMyShortDescriptionBuilder, bool);

// =========================================================================
// GetMyShortDescriptionBuilder
// =========================================================================

/// Builder for the [`getMyShortDescription`] API method.
#[derive(Serialize)]
pub struct GetMyShortDescriptionBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl<'a> GetMyShortDescriptionBuilder<'a> {
    /// Sets the `language_code` parameter.
    pub fn language_code(mut self, val: impl Into<String>) -> Self {
        self.language_code = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bot_description::BotShortDescription> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("getMyShortDescription", &payload)
            .await
    }
}

impl_into_future!(
    GetMyShortDescriptionBuilder,
    bot_description::BotShortDescription
);

// =========================================================================
// SetMyNameBuilder
// =========================================================================

/// Builder for the [`setMyName`] API method.
#[derive(Serialize)]
pub struct SetMyNameBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl<'a> SetMyNameBuilder<'a> {
    /// Sets the `name` parameter.
    pub fn name(mut self, val: impl Into<String>) -> Self {
        self.name = Some(val.into());
        self
    }
    /// Sets the `language_code` parameter.
    pub fn language_code(mut self, val: impl Into<String>) -> Self {
        self.language_code = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("setMyName", &payload).await
    }
}

impl_into_future!(SetMyNameBuilder, bool);

// =========================================================================
// GetMyNameBuilder
// =========================================================================

/// Builder for the [`getMyName`] API method.
#[derive(Serialize)]
pub struct GetMyNameBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl<'a> GetMyNameBuilder<'a> {
    /// Sets the `language_code` parameter.
    pub fn language_code(mut self, val: impl Into<String>) -> Self {
        self.language_code = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bot_name::BotName> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("getMyName", &payload).await
    }
}

impl_into_future!(GetMyNameBuilder, bot_name::BotName);

// =========================================================================
//  FORUM BUILDERS (12 methods from forum.rs)
// =========================================================================

// =========================================================================
// CreateForumTopicBuilder
// =========================================================================

/// Builder for the [`createForumTopic`] API method.
#[derive(Serialize)]
pub struct CreateForumTopicBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_color: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_custom_emoji_id: Option<String>,
}

impl<'a> CreateForumTopicBuilder<'a> {
    /// Sets the `icon_color` parameter.
    pub fn icon_color(mut self, val: i64) -> Self {
        self.icon_color = Some(val);
        self
    }
    /// Sets the `icon_custom_emoji_id` parameter.
    pub fn icon_custom_emoji_id(mut self, val: impl Into<String>) -> Self {
        self.icon_custom_emoji_id = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<forum_topic::ForumTopic> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("createForumTopic", &payload).await
    }
}

impl_into_future!(CreateForumTopicBuilder, forum_topic::ForumTopic);

// =========================================================================
// EditForumTopicBuilder
// =========================================================================

/// Builder for the [`editForumTopic`] API method.
#[derive(Serialize)]
pub struct EditForumTopicBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    message_thread_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_custom_emoji_id: Option<String>,
}

impl<'a> EditForumTopicBuilder<'a> {
    /// Sets the `name` parameter.
    pub fn name(mut self, val: impl Into<String>) -> Self {
        self.name = Some(val.into());
        self
    }
    /// Sets the `icon_custom_emoji_id` parameter.
    pub fn icon_custom_emoji_id(mut self, val: impl Into<String>) -> Self {
        self.icon_custom_emoji_id = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("editForumTopic", &payload).await
    }
}

impl_into_future!(EditForumTopicBuilder, bool);

// =========================================================================
// CloseForumTopicBuilder
// =========================================================================

/// Builder for the [`closeForumTopic`] API method.
#[derive(Serialize)]
pub struct CloseForumTopicBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    message_thread_id: i64,
}

impl<'a> CloseForumTopicBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("closeForumTopic", &payload).await
    }
}

impl_into_future!(CloseForumTopicBuilder, bool);

// =========================================================================
// ReopenForumTopicBuilder
// =========================================================================

/// Builder for the [`reopenForumTopic`] API method.
#[derive(Serialize)]
pub struct ReopenForumTopicBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    message_thread_id: i64,
}

impl<'a> ReopenForumTopicBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("reopenForumTopic", &payload).await
    }
}

impl_into_future!(ReopenForumTopicBuilder, bool);

// =========================================================================
// DeleteForumTopicBuilder
// =========================================================================

/// Builder for the [`deleteForumTopic`] API method.
#[derive(Serialize)]
pub struct DeleteForumTopicBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    message_thread_id: i64,
}

impl<'a> DeleteForumTopicBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("deleteForumTopic", &payload).await
    }
}

impl_into_future!(DeleteForumTopicBuilder, bool);

// =========================================================================
// UnpinAllForumTopicMessagesBuilder
// =========================================================================

/// Builder for the [`unpinAllForumTopicMessages`] API method.
#[derive(Serialize)]
pub struct UnpinAllForumTopicMessagesBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    message_thread_id: i64,
}

impl<'a> UnpinAllForumTopicMessagesBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("unpinAllForumTopicMessages", &payload)
            .await
    }
}

impl_into_future!(UnpinAllForumTopicMessagesBuilder, bool);

// =========================================================================
// UnpinAllGeneralForumTopicMessagesBuilder
// =========================================================================

/// Builder for the [`unpinAllGeneralForumTopicMessages`] API method.
#[derive(Serialize)]
pub struct UnpinAllGeneralForumTopicMessagesBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
}

impl<'a> UnpinAllGeneralForumTopicMessagesBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("unpinAllGeneralForumTopicMessages", &payload)
            .await
    }
}

impl_into_future!(UnpinAllGeneralForumTopicMessagesBuilder, bool);

// =========================================================================
// EditGeneralForumTopicBuilder
// =========================================================================

/// Builder for the [`editGeneralForumTopic`] API method.
#[derive(Serialize)]
pub struct EditGeneralForumTopicBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    name: String,
}

impl<'a> EditGeneralForumTopicBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("editGeneralForumTopic", &payload)
            .await
    }
}

impl_into_future!(EditGeneralForumTopicBuilder, bool);

// =========================================================================
// CloseGeneralForumTopicBuilder
// =========================================================================

/// Builder for the [`closeGeneralForumTopic`] API method.
#[derive(Serialize)]
pub struct CloseGeneralForumTopicBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
}

impl<'a> CloseGeneralForumTopicBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("closeGeneralForumTopic", &payload)
            .await
    }
}

impl_into_future!(CloseGeneralForumTopicBuilder, bool);

// =========================================================================
// ReopenGeneralForumTopicBuilder
// =========================================================================

/// Builder for the [`reopenGeneralForumTopic`] API method.
#[derive(Serialize)]
pub struct ReopenGeneralForumTopicBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
}

impl<'a> ReopenGeneralForumTopicBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("reopenGeneralForumTopic", &payload)
            .await
    }
}

impl_into_future!(ReopenGeneralForumTopicBuilder, bool);

// =========================================================================
// HideGeneralForumTopicBuilder
// =========================================================================

/// Builder for the [`hideGeneralForumTopic`] API method.
#[derive(Serialize)]
pub struct HideGeneralForumTopicBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
}

impl<'a> HideGeneralForumTopicBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("hideGeneralForumTopic", &payload)
            .await
    }
}

impl_into_future!(HideGeneralForumTopicBuilder, bool);

// =========================================================================
// UnhideGeneralForumTopicBuilder
// =========================================================================

/// Builder for the [`unhideGeneralForumTopic`] API method.
#[derive(Serialize)]
pub struct UnhideGeneralForumTopicBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
}

impl<'a> UnhideGeneralForumTopicBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("unhideGeneralForumTopic", &payload)
            .await
    }
}

impl_into_future!(UnhideGeneralForumTopicBuilder, bool);

// =========================================================================
//  STICKER BUILDERS (16 methods from stickers.rs)
// =========================================================================

// =========================================================================
// GetStickerSetBuilder
// =========================================================================

/// Builder for the [`getStickerSet`] API method.
#[derive(Serialize)]
pub struct GetStickerSetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    name: String,
}

impl<'a> GetStickerSetBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<files::sticker::StickerSet> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("getStickerSet", &payload).await
    }
}

impl_into_future!(GetStickerSetBuilder, files::sticker::StickerSet);

// =========================================================================
// GetCustomEmojiStickersBuilder
// =========================================================================

/// Builder for the [`getCustomEmojiStickers`] API method.
#[derive(Serialize)]
pub struct GetCustomEmojiStickersBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    custom_emoji_ids: Vec<String>,
}

impl<'a> GetCustomEmojiStickersBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<Vec<files::sticker::Sticker>> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("getCustomEmojiStickers", &payload)
            .await
    }
}

impl_into_future!(GetCustomEmojiStickersBuilder, Vec<files::sticker::Sticker>);

// =========================================================================
// UploadStickerFileBuilder
// =========================================================================

/// Builder for the [`uploadStickerFile`] API method.
pub struct UploadStickerFileBuilder<'a> {
    bot: &'a Bot,
    user_id: i64,
    sticker: files::input_file::InputFile,
    sticker_format: String,
}

impl<'a> UploadStickerFileBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<files::file::File> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(self.user_id)?),
            input_file_param("sticker", self.sticker),
            RequestParameter::new(
                "sticker_format",
                serde_json::Value::String(self.sticker_format),
            ),
        ];
        self.bot.do_api_request("uploadStickerFile", params).await
    }
}

impl_into_future!(UploadStickerFileBuilder, files::file::File);

// =========================================================================
// CreateNewStickerSetBuilder
// =========================================================================

/// Builder for the [`createNewStickerSet`] API method.
#[derive(Serialize)]
pub struct CreateNewStickerSetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    user_id: i64,
    name: String,
    title: String,
    stickers: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sticker_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    needs_repainting: Option<bool>,
}

impl<'a> CreateNewStickerSetBuilder<'a> {
    /// Sets the `sticker_type` parameter.
    pub fn sticker_type(mut self, val: impl Into<String>) -> Self {
        self.sticker_type = Some(val.into());
        self
    }
    /// Sets the `needs_repainting` parameter.
    pub fn needs_repainting(mut self, val: bool) -> Self {
        self.needs_repainting = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("createNewStickerSet", &payload).await
    }
}

impl_into_future!(CreateNewStickerSetBuilder, bool);

// =========================================================================
// AddStickerToSetBuilder
// =========================================================================

/// Builder for the [`addStickerToSet`] API method.
#[derive(Serialize)]
pub struct AddStickerToSetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    user_id: i64,
    name: String,
    sticker: serde_json::Value,
}

impl<'a> AddStickerToSetBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("addStickerToSet", &payload).await
    }
}

impl_into_future!(AddStickerToSetBuilder, bool);

// =========================================================================
// SetStickerPositionInSetBuilder
// =========================================================================

/// Builder for the [`setStickerPositionInSet`] API method.
#[derive(Serialize)]
pub struct SetStickerPositionInSetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    sticker: String,
    position: i64,
}

impl<'a> SetStickerPositionInSetBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("setStickerPositionInSet", &payload)
            .await
    }
}

impl_into_future!(SetStickerPositionInSetBuilder, bool);

// =========================================================================
// DeleteStickerFromSetBuilder
// =========================================================================

/// Builder for the [`deleteStickerFromSet`] API method.
#[derive(Serialize)]
pub struct DeleteStickerFromSetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    sticker: String,
}

impl<'a> DeleteStickerFromSetBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("deleteStickerFromSet", &payload)
            .await
    }
}

impl_into_future!(DeleteStickerFromSetBuilder, bool);

// =========================================================================
// ReplaceStickerInSetBuilder
// =========================================================================

/// Builder for the [`replaceStickerInSet`] API method.
#[derive(Serialize)]
pub struct ReplaceStickerInSetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    user_id: i64,
    name: String,
    old_sticker: String,
    sticker: serde_json::Value,
}

impl<'a> ReplaceStickerInSetBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("replaceStickerInSet", &payload).await
    }
}

impl_into_future!(ReplaceStickerInSetBuilder, bool);

// =========================================================================
// SetStickerEmojiListBuilder
// =========================================================================

/// Builder for the [`setStickerEmojiList`] API method.
#[derive(Serialize)]
pub struct SetStickerEmojiListBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    sticker: String,
    emoji_list: Vec<String>,
}

impl<'a> SetStickerEmojiListBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("setStickerEmojiList", &payload).await
    }
}

impl_into_future!(SetStickerEmojiListBuilder, bool);

// =========================================================================
// SetStickerKeywordsBuilder
// =========================================================================

/// Builder for the [`setStickerKeywords`] API method.
#[derive(Serialize)]
pub struct SetStickerKeywordsBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    sticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<Vec<String>>,
}

impl<'a> SetStickerKeywordsBuilder<'a> {
    /// Sets the `keywords` parameter.
    pub fn keywords(mut self, val: Vec<String>) -> Self {
        self.keywords = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("setStickerKeywords", &payload).await
    }
}

impl_into_future!(SetStickerKeywordsBuilder, bool);

// =========================================================================
// SetStickerMaskPositionBuilder
// =========================================================================

/// Builder for the [`setStickerMaskPosition`] API method.
#[derive(Serialize)]
pub struct SetStickerMaskPositionBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    sticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    mask_position: Option<files::sticker::MaskPosition>,
}

impl<'a> SetStickerMaskPositionBuilder<'a> {
    /// Sets the `mask_position` parameter.
    pub fn mask_position(mut self, val: files::sticker::MaskPosition) -> Self {
        self.mask_position = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("setStickerMaskPosition", &payload)
            .await
    }
}

impl_into_future!(SetStickerMaskPositionBuilder, bool);

// =========================================================================
// SetStickerSetThumbnailBuilder
// =========================================================================

/// Builder for the [`setStickerSetThumbnail`] API method.
///
/// This builder uses the `RequestParameter` approach because `thumbnail`
/// can be a file upload.
pub struct SetStickerSetThumbnailBuilder<'a> {
    bot: &'a Bot,
    name: String,
    user_id: i64,
    format: String,
    thumbnail: Option<files::input_file::InputFile>,
}

impl<'a> SetStickerSetThumbnailBuilder<'a> {
    /// Sets the `thumbnail` parameter.
    pub fn thumbnail(mut self, val: files::input_file::InputFile) -> Self {
        self.thumbnail = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("name", serde_json::Value::String(self.name)),
            RequestParameter::new("user_id", serde_json::to_value(self.user_id)?),
            RequestParameter::new("format", serde_json::Value::String(self.format)),
        ];
        push_opt_file(&mut params, "thumbnail", self.thumbnail);
        self.bot
            .do_api_request("setStickerSetThumbnail", params)
            .await
    }
}

impl_into_future!(SetStickerSetThumbnailBuilder, bool);

// =========================================================================
// SetStickerSetTitleBuilder
// =========================================================================

/// Builder for the [`setStickerSetTitle`] API method.
#[derive(Serialize)]
pub struct SetStickerSetTitleBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    name: String,
    title: String,
}

impl<'a> SetStickerSetTitleBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("setStickerSetTitle", &payload).await
    }
}

impl_into_future!(SetStickerSetTitleBuilder, bool);

// =========================================================================
// SetCustomEmojiStickerSetThumbnailBuilder
// =========================================================================

/// Builder for the [`setCustomEmojiStickerSetThumbnail`] API method.
#[derive(Serialize)]
pub struct SetCustomEmojiStickerSetThumbnailBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_emoji_id: Option<String>,
}

impl<'a> SetCustomEmojiStickerSetThumbnailBuilder<'a> {
    /// Sets the `custom_emoji_id` parameter.
    pub fn custom_emoji_id(mut self, val: impl Into<String>) -> Self {
        self.custom_emoji_id = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("setCustomEmojiStickerSetThumbnail", &payload)
            .await
    }
}

impl_into_future!(SetCustomEmojiStickerSetThumbnailBuilder, bool);

// =========================================================================
// DeleteStickerSetBuilder
// =========================================================================

/// Builder for the [`deleteStickerSet`] API method.
#[derive(Serialize)]
pub struct DeleteStickerSetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    name: String,
}

impl<'a> DeleteStickerSetBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("deleteStickerSet", &payload).await
    }
}

impl_into_future!(DeleteStickerSetBuilder, bool);

// =========================================================================
// GetForumTopicIconStickersBuilder
// =========================================================================

/// Builder for the [`getForumTopicIconStickers`] API method.
#[derive(Serialize)]
pub struct GetForumTopicIconStickersBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
}

impl<'a> GetForumTopicIconStickersBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<Vec<files::sticker::Sticker>> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("getForumTopicIconStickers", &payload)
            .await
    }
}

impl_into_future!(
    GetForumTopicIconStickersBuilder,
    Vec<files::sticker::Sticker>
);

// =========================================================================
// Factory methods on Bot
// =========================================================================

impl Bot {
    // =====================================================================
    // Admin builders
    // =====================================================================

    /// Build a `setChatMenuButton` request.
    pub fn set_chat_menu_button(&self) -> SetChatMenuButtonBuilder<'_> {
        SetChatMenuButtonBuilder {
            bot: self,
            chat_id: None,
            menu_button: None,
        }
    }

    /// Build a `getChatMenuButton` request.
    pub fn get_chat_menu_button(&self) -> GetChatMenuButtonBuilder<'_> {
        GetChatMenuButtonBuilder {
            bot: self,
            chat_id: None,
        }
    }

    /// Build a `setMyCommands` request.
    pub fn set_my_commands(
        &self,
        commands: Vec<bot_command::BotCommand>,
    ) -> SetMyCommandsBuilder<'_> {
        SetMyCommandsBuilder {
            bot: self,
            commands,
            scope: None,
            language_code: None,
        }
    }

    /// Build a `getMyCommands` request.
    pub fn get_my_commands(&self) -> GetMyCommandsBuilder<'_> {
        GetMyCommandsBuilder {
            bot: self,
            scope: None,
            language_code: None,
        }
    }

    /// Build a `deleteMyCommands` request.
    pub fn delete_my_commands(&self) -> DeleteMyCommandsBuilder<'_> {
        DeleteMyCommandsBuilder {
            bot: self,
            scope: None,
            language_code: None,
        }
    }

    /// Build a `setMyDefaultAdministratorRights` request.
    pub fn set_my_default_administrator_rights(
        &self,
    ) -> SetMyDefaultAdministratorRightsBuilder<'_> {
        SetMyDefaultAdministratorRightsBuilder {
            bot: self,
            rights: None,
            for_channels: None,
        }
    }

    /// Build a `getMyDefaultAdministratorRights` request.
    pub fn get_my_default_administrator_rights(
        &self,
    ) -> GetMyDefaultAdministratorRightsBuilder<'_> {
        GetMyDefaultAdministratorRightsBuilder {
            bot: self,
            for_channels: None,
        }
    }

    /// Build a `setMyDescription` request.
    pub fn set_my_description(&self) -> SetMyDescriptionBuilder<'_> {
        SetMyDescriptionBuilder {
            bot: self,
            description: None,
            language_code: None,
        }
    }

    /// Build a `getMyDescription` request.
    pub fn get_my_description(&self) -> GetMyDescriptionBuilder<'_> {
        GetMyDescriptionBuilder {
            bot: self,
            language_code: None,
        }
    }

    /// Build a `setMyShortDescription` request.
    pub fn set_my_short_description(&self) -> SetMyShortDescriptionBuilder<'_> {
        SetMyShortDescriptionBuilder {
            bot: self,
            short_description: None,
            language_code: None,
        }
    }

    /// Build a `getMyShortDescription` request.
    pub fn get_my_short_description(&self) -> GetMyShortDescriptionBuilder<'_> {
        GetMyShortDescriptionBuilder {
            bot: self,
            language_code: None,
        }
    }

    /// Build a `setMyName` request.
    pub fn set_my_name(&self) -> SetMyNameBuilder<'_> {
        SetMyNameBuilder {
            bot: self,
            name: None,
            language_code: None,
        }
    }

    /// Build a `getMyName` request.
    pub fn get_my_name(&self) -> GetMyNameBuilder<'_> {
        GetMyNameBuilder {
            bot: self,
            language_code: None,
        }
    }

    // =====================================================================
    // Forum builders
    // =====================================================================

    /// Build a `createForumTopic` request.
    pub fn create_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        name: impl Into<String>,
    ) -> CreateForumTopicBuilder<'_> {
        CreateForumTopicBuilder {
            bot: self,
            chat_id: chat_id.into(),
            name: name.into(),
            icon_color: None,
            icon_custom_emoji_id: None,
        }
    }

    /// Build an `editForumTopic` request.
    pub fn edit_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
    ) -> EditForumTopicBuilder<'_> {
        EditForumTopicBuilder {
            bot: self,
            chat_id: chat_id.into(),
            message_thread_id,
            name: None,
            icon_custom_emoji_id: None,
        }
    }

    /// Build a `closeForumTopic` request.
    pub fn close_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
    ) -> CloseForumTopicBuilder<'_> {
        CloseForumTopicBuilder {
            bot: self,
            chat_id: chat_id.into(),
            message_thread_id,
        }
    }

    /// Build a `reopenForumTopic` request.
    pub fn reopen_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
    ) -> ReopenForumTopicBuilder<'_> {
        ReopenForumTopicBuilder {
            bot: self,
            chat_id: chat_id.into(),
            message_thread_id,
        }
    }

    /// Build a `deleteForumTopic` request.
    pub fn delete_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
    ) -> DeleteForumTopicBuilder<'_> {
        DeleteForumTopicBuilder {
            bot: self,
            chat_id: chat_id.into(),
            message_thread_id,
        }
    }

    /// Build an `unpinAllForumTopicMessages` request.
    pub fn unpin_all_forum_topic_messages(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
    ) -> UnpinAllForumTopicMessagesBuilder<'_> {
        UnpinAllForumTopicMessagesBuilder {
            bot: self,
            chat_id: chat_id.into(),
            message_thread_id,
        }
    }

    /// Build an `unpinAllGeneralForumTopicMessages` request.
    pub fn unpin_all_general_forum_topic_messages(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> UnpinAllGeneralForumTopicMessagesBuilder<'_> {
        UnpinAllGeneralForumTopicMessagesBuilder {
            bot: self,
            chat_id: chat_id.into(),
        }
    }

    /// Build an `editGeneralForumTopic` request.
    pub fn edit_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        name: impl Into<String>,
    ) -> EditGeneralForumTopicBuilder<'_> {
        EditGeneralForumTopicBuilder {
            bot: self,
            chat_id: chat_id.into(),
            name: name.into(),
        }
    }

    /// Build a `closeGeneralForumTopic` request.
    pub fn close_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> CloseGeneralForumTopicBuilder<'_> {
        CloseGeneralForumTopicBuilder {
            bot: self,
            chat_id: chat_id.into(),
        }
    }

    /// Build a `reopenGeneralForumTopic` request.
    pub fn reopen_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> ReopenGeneralForumTopicBuilder<'_> {
        ReopenGeneralForumTopicBuilder {
            bot: self,
            chat_id: chat_id.into(),
        }
    }

    /// Build a `hideGeneralForumTopic` request.
    pub fn hide_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> HideGeneralForumTopicBuilder<'_> {
        HideGeneralForumTopicBuilder {
            bot: self,
            chat_id: chat_id.into(),
        }
    }

    /// Build an `unhideGeneralForumTopic` request.
    pub fn unhide_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> UnhideGeneralForumTopicBuilder<'_> {
        UnhideGeneralForumTopicBuilder {
            bot: self,
            chat_id: chat_id.into(),
        }
    }

    // =====================================================================
    // Sticker builders
    // =====================================================================

    /// Build a `getStickerSet` request.
    pub fn get_sticker_set(&self, name: impl Into<String>) -> GetStickerSetBuilder<'_> {
        GetStickerSetBuilder {
            bot: self,
            name: name.into(),
        }
    }

    /// Build a `getCustomEmojiStickers` request.
    pub fn get_custom_emoji_stickers(
        &self,
        custom_emoji_ids: Vec<String>,
    ) -> GetCustomEmojiStickersBuilder<'_> {
        GetCustomEmojiStickersBuilder {
            bot: self,
            custom_emoji_ids,
        }
    }

    /// Build an `uploadStickerFile` request.
    pub fn upload_sticker_file(
        &self,
        user_id: i64,
        sticker: files::input_file::InputFile,
        sticker_format: impl Into<String>,
    ) -> UploadStickerFileBuilder<'_> {
        UploadStickerFileBuilder {
            bot: self,
            user_id,
            sticker,
            sticker_format: sticker_format.into(),
        }
    }

    /// Build a `createNewStickerSet` request.
    pub fn create_new_sticker_set(
        &self,
        user_id: i64,
        name: impl Into<String>,
        title: impl Into<String>,
        stickers: Vec<serde_json::Value>,
    ) -> CreateNewStickerSetBuilder<'_> {
        CreateNewStickerSetBuilder {
            bot: self,
            user_id,
            name: name.into(),
            title: title.into(),
            stickers,
            sticker_type: None,
            needs_repainting: None,
        }
    }

    /// Build an `addStickerToSet` request.
    pub fn add_sticker_to_set(
        &self,
        user_id: i64,
        name: impl Into<String>,
        sticker: serde_json::Value,
    ) -> AddStickerToSetBuilder<'_> {
        AddStickerToSetBuilder {
            bot: self,
            user_id,
            name: name.into(),
            sticker,
        }
    }

    /// Build a `setStickerPositionInSet` request.
    pub fn set_sticker_position_in_set(
        &self,
        sticker: impl Into<String>,
        position: i64,
    ) -> SetStickerPositionInSetBuilder<'_> {
        SetStickerPositionInSetBuilder {
            bot: self,
            sticker: sticker.into(),
            position,
        }
    }

    /// Build a `deleteStickerFromSet` request.
    pub fn delete_sticker_from_set(
        &self,
        sticker: impl Into<String>,
    ) -> DeleteStickerFromSetBuilder<'_> {
        DeleteStickerFromSetBuilder {
            bot: self,
            sticker: sticker.into(),
        }
    }

    /// Build a `replaceStickerInSet` request.
    pub fn replace_sticker_in_set(
        &self,
        user_id: i64,
        name: impl Into<String>,
        old_sticker: impl Into<String>,
        sticker: serde_json::Value,
    ) -> ReplaceStickerInSetBuilder<'_> {
        ReplaceStickerInSetBuilder {
            bot: self,
            user_id,
            name: name.into(),
            old_sticker: old_sticker.into(),
            sticker,
        }
    }

    /// Build a `setStickerEmojiList` request.
    pub fn set_sticker_emoji_list(
        &self,
        sticker: impl Into<String>,
        emoji_list: Vec<String>,
    ) -> SetStickerEmojiListBuilder<'_> {
        SetStickerEmojiListBuilder {
            bot: self,
            sticker: sticker.into(),
            emoji_list,
        }
    }

    /// Build a `setStickerKeywords` request.
    pub fn set_sticker_keywords(
        &self,
        sticker: impl Into<String>,
    ) -> SetStickerKeywordsBuilder<'_> {
        SetStickerKeywordsBuilder {
            bot: self,
            sticker: sticker.into(),
            keywords: None,
        }
    }

    /// Build a `setStickerMaskPosition` request.
    pub fn set_sticker_mask_position(
        &self,
        sticker: impl Into<String>,
    ) -> SetStickerMaskPositionBuilder<'_> {
        SetStickerMaskPositionBuilder {
            bot: self,
            sticker: sticker.into(),
            mask_position: None,
        }
    }

    /// Build a `setStickerSetThumbnail` request.
    pub fn set_sticker_set_thumbnail(
        &self,
        name: impl Into<String>,
        user_id: i64,
        format: impl Into<String>,
    ) -> SetStickerSetThumbnailBuilder<'_> {
        SetStickerSetThumbnailBuilder {
            bot: self,
            name: name.into(),
            user_id,
            format: format.into(),
            thumbnail: None,
        }
    }

    /// Build a `setStickerSetTitle` request.
    pub fn set_sticker_set_title(
        &self,
        name: impl Into<String>,
        title: impl Into<String>,
    ) -> SetStickerSetTitleBuilder<'_> {
        SetStickerSetTitleBuilder {
            bot: self,
            name: name.into(),
            title: title.into(),
        }
    }

    /// Build a `setCustomEmojiStickerSetThumbnail` request.
    pub fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        name: impl Into<String>,
    ) -> SetCustomEmojiStickerSetThumbnailBuilder<'_> {
        SetCustomEmojiStickerSetThumbnailBuilder {
            bot: self,
            name: name.into(),
            custom_emoji_id: None,
        }
    }

    /// Build a `deleteStickerSet` request.
    pub fn delete_sticker_set(&self, name: impl Into<String>) -> DeleteStickerSetBuilder<'_> {
        DeleteStickerSetBuilder {
            bot: self,
            name: name.into(),
        }
    }

    /// Build a `getForumTopicIconStickers` request.
    pub fn get_forum_topic_icon_stickers(&self) -> GetForumTopicIconStickersBuilder<'_> {
        GetForumTopicIconStickersBuilder { bot: self }
    }
}
