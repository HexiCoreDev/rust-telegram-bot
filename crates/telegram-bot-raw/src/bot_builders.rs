//! Builder pattern for Telegram Bot API methods.
//!
//! Instead of passing a long list of optional parameters, builders let callers
//! set only the parameters they care about:
//!
//! ```ignore
//! bot.send_message(chat_id, "Hello!")
//!     .parse_mode(ParseMode::Html)
//!     .disable_notification(true)
//!     .await?;
//! ```
//!
//! Every builder follows the same pattern:
//! 1. Created via the corresponding `Bot` factory method with only required parameters.
//! 2. Chained setter calls for optional parameters.
//! 3. `.await?` (or `.send().await?`) to execute the request.

#![allow(clippy::too_many_arguments)]

use crate::bot::{Bot, ChatId, MessageOrBool};
use crate::error::Result;
use crate::request::request_parameter::{InputFileRef, RequestParameter};
use crate::types::{
    chat_full_info, chat_invite_link, chat_member, chat_permissions, files, inline,
    link_preview_options, message, message_entity, message_id, prepared_keyboard_button, reply,
    suggested_post,
};
use serde::Serialize;

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

// ---------------------------------------------------------------------------
// Private helpers (duplicated from bot.rs since those are private)
// ---------------------------------------------------------------------------

fn push_opt<T: Serialize>(
    params: &mut Vec<RequestParameter>,
    name: &'static str,
    val: &Option<T>,
) -> std::result::Result<(), serde_json::Error> {
    if let Some(v) = val {
        params.push(RequestParameter::new(name, serde_json::to_value(v)?));
    }
    Ok(())
}

fn push_opt_str(params: &mut Vec<RequestParameter>, name: &'static str, val: &Option<String>) {
    if let Some(v) = val {
        params.push(RequestParameter::new(
            name,
            serde_json::Value::String(v.clone()),
        ));
    }
}

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

// =========================================================================
// SendMessageBuilder
// =========================================================================

/// Builder for the [`sendMessage`] API method.
#[derive(Serialize)]
pub struct SendMessageBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<Vec<message_entity::MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_preview_options: Option<link_preview_options::LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<reply::ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_messages_topic_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendMessageBuilder<'a> {
    /// Sets the `parse_mode` parameter.
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self {
        self.parse_mode = Some(val.into());
        self
    }
    /// Sets the `entities` parameter.
    pub fn entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.entities = Some(val);
        self
    }
    /// Sets the `link_preview_options` parameter.
    pub fn link_preview_options(mut self, val: link_preview_options::LinkPreviewOptions) -> Self {
        self.link_preview_options = Some(val);
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("sendMessage", &payload).await
    }
}

impl_into_future!(SendMessageBuilder, message::Message);

// =========================================================================
// SendPhotoBuilder
// =========================================================================

/// Builder for the [`sendPhoto`] API method.
pub struct SendPhotoBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    photo: files::input_file::InputFile,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<message_entity::MessageEntity>>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<reply::ReplyParameters>,
    reply_markup: Option<serde_json::Value>,
    message_thread_id: Option<i64>,
    has_spoiler: Option<bool>,
    business_connection_id: Option<String>,
    message_effect_id: Option<String>,
    allow_paid_broadcast: Option<bool>,
    show_caption_above_media: Option<bool>,
    direct_messages_topic_id: Option<i64>,
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendPhotoBuilder<'a> {
    /// Sets the `caption` parameter.
    pub fn caption(mut self, val: impl Into<String>) -> Self {
        self.caption = Some(val.into());
        self
    }
    /// Sets the `parse_mode` parameter.
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self {
        self.parse_mode = Some(val.into());
        self
    }
    /// Sets the `caption_entities` parameter.
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.caption_entities = Some(val);
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `has_spoiler` parameter.
    pub fn has_spoiler(mut self, val: bool) -> Self {
        self.has_spoiler = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `show_caption_above_media` parameter.
    pub fn show_caption_above_media(mut self, val: bool) -> Self {
        self.show_caption_above_media = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            input_file_param("photo", self.photo),
        ];
        push_opt_str(&mut params, "caption", &self.caption);
        push_opt_str(&mut params, "parse_mode", &self.parse_mode);
        push_opt(&mut params, "caption_entities", &self.caption_entities)?;
        push_opt(
            &mut params,
            "disable_notification",
            &self.disable_notification,
        )?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt(&mut params, "has_spoiler", &self.has_spoiler)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            &self.business_connection_id,
        );
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(
            &mut params,
            "allow_paid_broadcast",
            &self.allow_paid_broadcast,
        )?;
        push_opt(
            &mut params,
            "show_caption_above_media",
            &self.show_caption_above_media,
        )?;
        push_opt(
            &mut params,
            "direct_messages_topic_id",
            &self.direct_messages_topic_id,
        )?;
        push_opt(
            &mut params,
            "suggested_post_parameters",
            &self.suggested_post_parameters,
        )?;
        self.bot.do_api_request("sendPhoto", params).await
    }
}

impl_into_future!(SendPhotoBuilder, message::Message);

// =========================================================================
// SendDocumentBuilder
// =========================================================================

/// Builder for the [`sendDocument`] API method.
pub struct SendDocumentBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    document: files::input_file::InputFile,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<message_entity::MessageEntity>>,
    disable_content_type_detection: Option<bool>,
    thumbnail: Option<files::input_file::InputFile>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<reply::ReplyParameters>,
    reply_markup: Option<serde_json::Value>,
    message_thread_id: Option<i64>,
    business_connection_id: Option<String>,
    message_effect_id: Option<String>,
    allow_paid_broadcast: Option<bool>,
    direct_messages_topic_id: Option<i64>,
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendDocumentBuilder<'a> {
    /// Sets the `caption` parameter.
    pub fn caption(mut self, val: impl Into<String>) -> Self {
        self.caption = Some(val.into());
        self
    }
    /// Sets the `parse_mode` parameter.
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self {
        self.parse_mode = Some(val.into());
        self
    }
    /// Sets the `caption_entities` parameter.
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.caption_entities = Some(val);
        self
    }
    /// Sets the `disable_content_type_detection` parameter.
    pub fn disable_content_type_detection(mut self, val: bool) -> Self {
        self.disable_content_type_detection = Some(val);
        self
    }
    /// Sets the `thumbnail` parameter.
    pub fn thumbnail(mut self, val: files::input_file::InputFile) -> Self {
        self.thumbnail = Some(val);
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            input_file_param("document", self.document),
        ];
        push_opt_str(&mut params, "caption", &self.caption);
        push_opt_str(&mut params, "parse_mode", &self.parse_mode);
        push_opt(&mut params, "caption_entities", &self.caption_entities)?;
        push_opt(
            &mut params,
            "disable_content_type_detection",
            &self.disable_content_type_detection,
        )?;
        push_opt_file(&mut params, "thumbnail", self.thumbnail);
        push_opt(
            &mut params,
            "disable_notification",
            &self.disable_notification,
        )?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            &self.business_connection_id,
        );
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(
            &mut params,
            "allow_paid_broadcast",
            &self.allow_paid_broadcast,
        )?;
        push_opt(
            &mut params,
            "direct_messages_topic_id",
            &self.direct_messages_topic_id,
        )?;
        push_opt(
            &mut params,
            "suggested_post_parameters",
            &self.suggested_post_parameters,
        )?;
        self.bot.do_api_request("sendDocument", params).await
    }
}

impl_into_future!(SendDocumentBuilder, message::Message);

// =========================================================================
// SendVideoBuilder
// =========================================================================

/// Builder for the [`sendVideo`] API method.
pub struct SendVideoBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    video: files::input_file::InputFile,
    duration: Option<i64>,
    width: Option<i64>,
    height: Option<i64>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<message_entity::MessageEntity>>,
    supports_streaming: Option<bool>,
    thumbnail: Option<files::input_file::InputFile>,
    has_spoiler: Option<bool>,
    show_caption_above_media: Option<bool>,
    cover: Option<files::input_file::InputFile>,
    start_timestamp: Option<i64>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<reply::ReplyParameters>,
    reply_markup: Option<serde_json::Value>,
    message_thread_id: Option<i64>,
    business_connection_id: Option<String>,
    message_effect_id: Option<String>,
    allow_paid_broadcast: Option<bool>,
    direct_messages_topic_id: Option<i64>,
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendVideoBuilder<'a> {
    /// Sets the `duration` parameter.
    pub fn duration(mut self, val: i64) -> Self {
        self.duration = Some(val);
        self
    }
    /// Sets the `width` parameter.
    pub fn width(mut self, val: i64) -> Self {
        self.width = Some(val);
        self
    }
    /// Sets the `height` parameter.
    pub fn height(mut self, val: i64) -> Self {
        self.height = Some(val);
        self
    }
    /// Sets the `caption` parameter.
    pub fn caption(mut self, val: impl Into<String>) -> Self {
        self.caption = Some(val.into());
        self
    }
    /// Sets the `parse_mode` parameter.
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self {
        self.parse_mode = Some(val.into());
        self
    }
    /// Sets the `caption_entities` parameter.
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.caption_entities = Some(val);
        self
    }
    /// Sets the `supports_streaming` parameter.
    pub fn supports_streaming(mut self, val: bool) -> Self {
        self.supports_streaming = Some(val);
        self
    }
    /// Sets the `thumbnail` parameter.
    pub fn thumbnail(mut self, val: files::input_file::InputFile) -> Self {
        self.thumbnail = Some(val);
        self
    }
    /// Sets the `has_spoiler` parameter.
    pub fn has_spoiler(mut self, val: bool) -> Self {
        self.has_spoiler = Some(val);
        self
    }
    /// Sets the `show_caption_above_media` parameter.
    pub fn show_caption_above_media(mut self, val: bool) -> Self {
        self.show_caption_above_media = Some(val);
        self
    }
    /// Sets the `cover` parameter.
    pub fn cover(mut self, val: files::input_file::InputFile) -> Self {
        self.cover = Some(val);
        self
    }
    /// Sets the `start_timestamp` parameter.
    pub fn start_timestamp(mut self, val: i64) -> Self {
        self.start_timestamp = Some(val);
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            input_file_param("video", self.video),
        ];
        push_opt(&mut params, "duration", &self.duration)?;
        push_opt(&mut params, "width", &self.width)?;
        push_opt(&mut params, "height", &self.height)?;
        push_opt_str(&mut params, "caption", &self.caption);
        push_opt_str(&mut params, "parse_mode", &self.parse_mode);
        push_opt(&mut params, "caption_entities", &self.caption_entities)?;
        push_opt(&mut params, "supports_streaming", &self.supports_streaming)?;
        push_opt_file(&mut params, "thumbnail", self.thumbnail);
        push_opt(&mut params, "has_spoiler", &self.has_spoiler)?;
        push_opt(
            &mut params,
            "show_caption_above_media",
            &self.show_caption_above_media,
        )?;
        push_opt_file(&mut params, "cover", self.cover);
        push_opt(&mut params, "start_timestamp", &self.start_timestamp)?;
        push_opt(
            &mut params,
            "disable_notification",
            &self.disable_notification,
        )?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            &self.business_connection_id,
        );
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(
            &mut params,
            "allow_paid_broadcast",
            &self.allow_paid_broadcast,
        )?;
        push_opt(
            &mut params,
            "direct_messages_topic_id",
            &self.direct_messages_topic_id,
        )?;
        push_opt(
            &mut params,
            "suggested_post_parameters",
            &self.suggested_post_parameters,
        )?;
        self.bot.do_api_request("sendVideo", params).await
    }
}

impl_into_future!(SendVideoBuilder, message::Message);

// =========================================================================
// SendAudioBuilder
// =========================================================================

/// Builder for the [`sendAudio`] API method.
pub struct SendAudioBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    audio: files::input_file::InputFile,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<message_entity::MessageEntity>>,
    duration: Option<i64>,
    performer: Option<String>,
    title: Option<String>,
    thumbnail: Option<files::input_file::InputFile>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<reply::ReplyParameters>,
    reply_markup: Option<serde_json::Value>,
    message_thread_id: Option<i64>,
    business_connection_id: Option<String>,
    message_effect_id: Option<String>,
    allow_paid_broadcast: Option<bool>,
    direct_messages_topic_id: Option<i64>,
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendAudioBuilder<'a> {
    /// Sets the `caption` parameter.
    pub fn caption(mut self, val: impl Into<String>) -> Self {
        self.caption = Some(val.into());
        self
    }
    /// Sets the `parse_mode` parameter.
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self {
        self.parse_mode = Some(val.into());
        self
    }
    /// Sets the `caption_entities` parameter.
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.caption_entities = Some(val);
        self
    }
    /// Sets the `duration` parameter.
    pub fn duration(mut self, val: i64) -> Self {
        self.duration = Some(val);
        self
    }
    /// Sets the `performer` parameter.
    pub fn performer(mut self, val: impl Into<String>) -> Self {
        self.performer = Some(val.into());
        self
    }
    /// Sets the `title` parameter.
    pub fn title(mut self, val: impl Into<String>) -> Self {
        self.title = Some(val.into());
        self
    }
    /// Sets the `thumbnail` parameter.
    pub fn thumbnail(mut self, val: files::input_file::InputFile) -> Self {
        self.thumbnail = Some(val);
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            input_file_param("audio", self.audio),
        ];
        push_opt_str(&mut params, "caption", &self.caption);
        push_opt_str(&mut params, "parse_mode", &self.parse_mode);
        push_opt(&mut params, "caption_entities", &self.caption_entities)?;
        push_opt(&mut params, "duration", &self.duration)?;
        push_opt_str(&mut params, "performer", &self.performer);
        push_opt_str(&mut params, "title", &self.title);
        push_opt_file(&mut params, "thumbnail", self.thumbnail);
        push_opt(
            &mut params,
            "disable_notification",
            &self.disable_notification,
        )?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            &self.business_connection_id,
        );
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(
            &mut params,
            "allow_paid_broadcast",
            &self.allow_paid_broadcast,
        )?;
        push_opt(
            &mut params,
            "direct_messages_topic_id",
            &self.direct_messages_topic_id,
        )?;
        push_opt(
            &mut params,
            "suggested_post_parameters",
            &self.suggested_post_parameters,
        )?;
        self.bot.do_api_request("sendAudio", params).await
    }
}

impl_into_future!(SendAudioBuilder, message::Message);

// =========================================================================
// SendAnimationBuilder
// =========================================================================

/// Builder for the [`sendAnimation`] API method.
pub struct SendAnimationBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    animation: files::input_file::InputFile,
    duration: Option<i64>,
    width: Option<i64>,
    height: Option<i64>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<message_entity::MessageEntity>>,
    thumbnail: Option<files::input_file::InputFile>,
    has_spoiler: Option<bool>,
    show_caption_above_media: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<reply::ReplyParameters>,
    reply_markup: Option<serde_json::Value>,
    message_thread_id: Option<i64>,
    business_connection_id: Option<String>,
    message_effect_id: Option<String>,
    allow_paid_broadcast: Option<bool>,
    direct_messages_topic_id: Option<i64>,
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendAnimationBuilder<'a> {
    /// Sets the `duration` parameter.
    pub fn duration(mut self, val: i64) -> Self {
        self.duration = Some(val);
        self
    }
    /// Sets the `width` parameter.
    pub fn width(mut self, val: i64) -> Self {
        self.width = Some(val);
        self
    }
    /// Sets the `height` parameter.
    pub fn height(mut self, val: i64) -> Self {
        self.height = Some(val);
        self
    }
    /// Sets the `caption` parameter.
    pub fn caption(mut self, val: impl Into<String>) -> Self {
        self.caption = Some(val.into());
        self
    }
    /// Sets the `parse_mode` parameter.
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self {
        self.parse_mode = Some(val.into());
        self
    }
    /// Sets the `caption_entities` parameter.
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.caption_entities = Some(val);
        self
    }
    /// Sets the `thumbnail` parameter.
    pub fn thumbnail(mut self, val: files::input_file::InputFile) -> Self {
        self.thumbnail = Some(val);
        self
    }
    /// Sets the `has_spoiler` parameter.
    pub fn has_spoiler(mut self, val: bool) -> Self {
        self.has_spoiler = Some(val);
        self
    }
    /// Sets the `show_caption_above_media` parameter.
    pub fn show_caption_above_media(mut self, val: bool) -> Self {
        self.show_caption_above_media = Some(val);
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            input_file_param("animation", self.animation),
        ];
        push_opt(&mut params, "duration", &self.duration)?;
        push_opt(&mut params, "width", &self.width)?;
        push_opt(&mut params, "height", &self.height)?;
        push_opt_str(&mut params, "caption", &self.caption);
        push_opt_str(&mut params, "parse_mode", &self.parse_mode);
        push_opt(&mut params, "caption_entities", &self.caption_entities)?;
        push_opt_file(&mut params, "thumbnail", self.thumbnail);
        push_opt(&mut params, "has_spoiler", &self.has_spoiler)?;
        push_opt(
            &mut params,
            "show_caption_above_media",
            &self.show_caption_above_media,
        )?;
        push_opt(
            &mut params,
            "disable_notification",
            &self.disable_notification,
        )?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            &self.business_connection_id,
        );
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(
            &mut params,
            "allow_paid_broadcast",
            &self.allow_paid_broadcast,
        )?;
        push_opt(
            &mut params,
            "direct_messages_topic_id",
            &self.direct_messages_topic_id,
        )?;
        push_opt(
            &mut params,
            "suggested_post_parameters",
            &self.suggested_post_parameters,
        )?;
        self.bot.do_api_request("sendAnimation", params).await
    }
}

impl_into_future!(SendAnimationBuilder, message::Message);

// =========================================================================
// SendVoiceBuilder
// =========================================================================

/// Builder for the [`sendVoice`] API method.
pub struct SendVoiceBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    voice: files::input_file::InputFile,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<message_entity::MessageEntity>>,
    duration: Option<i64>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<reply::ReplyParameters>,
    reply_markup: Option<serde_json::Value>,
    message_thread_id: Option<i64>,
    business_connection_id: Option<String>,
    message_effect_id: Option<String>,
    allow_paid_broadcast: Option<bool>,
    direct_messages_topic_id: Option<i64>,
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendVoiceBuilder<'a> {
    /// Sets the `caption` parameter.
    pub fn caption(mut self, val: impl Into<String>) -> Self {
        self.caption = Some(val.into());
        self
    }
    /// Sets the `parse_mode` parameter.
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self {
        self.parse_mode = Some(val.into());
        self
    }
    /// Sets the `caption_entities` parameter.
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.caption_entities = Some(val);
        self
    }
    /// Sets the `duration` parameter.
    pub fn duration(mut self, val: i64) -> Self {
        self.duration = Some(val);
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            input_file_param("voice", self.voice),
        ];
        push_opt_str(&mut params, "caption", &self.caption);
        push_opt_str(&mut params, "parse_mode", &self.parse_mode);
        push_opt(&mut params, "caption_entities", &self.caption_entities)?;
        push_opt(&mut params, "duration", &self.duration)?;
        push_opt(
            &mut params,
            "disable_notification",
            &self.disable_notification,
        )?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            &self.business_connection_id,
        );
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(
            &mut params,
            "allow_paid_broadcast",
            &self.allow_paid_broadcast,
        )?;
        push_opt(
            &mut params,
            "direct_messages_topic_id",
            &self.direct_messages_topic_id,
        )?;
        push_opt(
            &mut params,
            "suggested_post_parameters",
            &self.suggested_post_parameters,
        )?;
        self.bot.do_api_request("sendVoice", params).await
    }
}

impl_into_future!(SendVoiceBuilder, message::Message);

// =========================================================================
// SendVideoNoteBuilder
// =========================================================================

/// Builder for the [`sendVideoNote`] API method.
pub struct SendVideoNoteBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    video_note: files::input_file::InputFile,
    duration: Option<i64>,
    length: Option<i64>,
    thumbnail: Option<files::input_file::InputFile>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<reply::ReplyParameters>,
    reply_markup: Option<serde_json::Value>,
    message_thread_id: Option<i64>,
    business_connection_id: Option<String>,
    message_effect_id: Option<String>,
    allow_paid_broadcast: Option<bool>,
    direct_messages_topic_id: Option<i64>,
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendVideoNoteBuilder<'a> {
    /// Sets the `duration` parameter.
    pub fn duration(mut self, val: i64) -> Self {
        self.duration = Some(val);
        self
    }
    /// Sets the `length` parameter.
    pub fn length(mut self, val: i64) -> Self {
        self.length = Some(val);
        self
    }
    /// Sets the `thumbnail` parameter.
    pub fn thumbnail(mut self, val: files::input_file::InputFile) -> Self {
        self.thumbnail = Some(val);
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            input_file_param("video_note", self.video_note),
        ];
        push_opt(&mut params, "duration", &self.duration)?;
        push_opt(&mut params, "length", &self.length)?;
        push_opt_file(&mut params, "thumbnail", self.thumbnail);
        push_opt(
            &mut params,
            "disable_notification",
            &self.disable_notification,
        )?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            &self.business_connection_id,
        );
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(
            &mut params,
            "allow_paid_broadcast",
            &self.allow_paid_broadcast,
        )?;
        push_opt(
            &mut params,
            "direct_messages_topic_id",
            &self.direct_messages_topic_id,
        )?;
        push_opt(
            &mut params,
            "suggested_post_parameters",
            &self.suggested_post_parameters,
        )?;
        self.bot.do_api_request("sendVideoNote", params).await
    }
}

impl_into_future!(SendVideoNoteBuilder, message::Message);

// =========================================================================
// SendLocationBuilder
// =========================================================================

/// Builder for the [`sendLocation`] API method.
#[derive(Serialize)]
pub struct SendLocationBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    latitude: f64,
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proximity_alert_radius: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<reply::ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_messages_topic_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendLocationBuilder<'a> {
    /// Sets the `horizontal_accuracy` parameter.
    pub fn horizontal_accuracy(mut self, val: f64) -> Self {
        self.horizontal_accuracy = Some(val);
        self
    }
    /// Sets the `live_period` parameter.
    pub fn live_period(mut self, val: i64) -> Self {
        self.live_period = Some(val);
        self
    }
    /// Sets the `heading` parameter.
    pub fn heading(mut self, val: i64) -> Self {
        self.heading = Some(val);
        self
    }
    /// Sets the `proximity_alert_radius` parameter.
    pub fn proximity_alert_radius(mut self, val: i64) -> Self {
        self.proximity_alert_radius = Some(val);
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("sendLocation", &payload).await
    }
}

impl_into_future!(SendLocationBuilder, message::Message);

// =========================================================================
// SendVenueBuilder
// =========================================================================

/// Builder for the [`sendVenue`] API method.
#[derive(Serialize)]
pub struct SendVenueBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<reply::ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_messages_topic_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendVenueBuilder<'a> {
    /// Sets the `foursquare_id` parameter.
    pub fn foursquare_id(mut self, val: impl Into<String>) -> Self {
        self.foursquare_id = Some(val.into());
        self
    }
    /// Sets the `foursquare_type` parameter.
    pub fn foursquare_type(mut self, val: impl Into<String>) -> Self {
        self.foursquare_type = Some(val.into());
        self
    }
    /// Sets the `google_place_id` parameter.
    pub fn google_place_id(mut self, val: impl Into<String>) -> Self {
        self.google_place_id = Some(val.into());
        self
    }
    /// Sets the `google_place_type` parameter.
    pub fn google_place_type(mut self, val: impl Into<String>) -> Self {
        self.google_place_type = Some(val.into());
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("sendVenue", &payload).await
    }
}

impl_into_future!(SendVenueBuilder, message::Message);

// =========================================================================
// SendContactBuilder
// =========================================================================

/// Builder for the [`sendContact`] API method.
#[derive(Serialize)]
pub struct SendContactBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    phone_number: String,
    first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<reply::ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_messages_topic_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendContactBuilder<'a> {
    /// Sets the `last_name` parameter.
    pub fn last_name(mut self, val: impl Into<String>) -> Self {
        self.last_name = Some(val.into());
        self
    }
    /// Sets the `vcard` parameter.
    pub fn vcard(mut self, val: impl Into<String>) -> Self {
        self.vcard = Some(val.into());
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("sendContact", &payload).await
    }
}

impl_into_future!(SendContactBuilder, message::Message);

// =========================================================================
// SendPollBuilder
// =========================================================================

/// Builder for the [`sendPoll`] API method.
#[derive(Serialize)]
pub struct SendPollBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    question: String,
    options: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_anonymous: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    poll_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allows_multiple_answers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correct_option_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation_entities: Option<Vec<message_entity::MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<reply::ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    question_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    question_entities: Option<Vec<message_entity::MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_messages_topic_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendPollBuilder<'a> {
    /// Sets the `is_anonymous` parameter.
    pub fn is_anonymous(mut self, val: bool) -> Self {
        self.is_anonymous = Some(val);
        self
    }
    /// Sets the `poll_type` parameter.
    pub fn poll_type(mut self, val: impl Into<String>) -> Self {
        self.poll_type = Some(val.into());
        self
    }
    /// Sets the `allows_multiple_answers` parameter.
    pub fn allows_multiple_answers(mut self, val: bool) -> Self {
        self.allows_multiple_answers = Some(val);
        self
    }
    /// Sets the `correct_option_id` parameter.
    pub fn correct_option_id(mut self, val: i64) -> Self {
        self.correct_option_id = Some(val);
        self
    }
    /// Sets the `explanation` parameter.
    pub fn explanation(mut self, val: impl Into<String>) -> Self {
        self.explanation = Some(val.into());
        self
    }
    /// Sets the `explanation_parse_mode` parameter.
    pub fn explanation_parse_mode(mut self, val: impl Into<String>) -> Self {
        self.explanation_parse_mode = Some(val.into());
        self
    }
    /// Sets the `explanation_entities` parameter.
    pub fn explanation_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.explanation_entities = Some(val);
        self
    }
    /// Sets the `open_period` parameter.
    pub fn open_period(mut self, val: i64) -> Self {
        self.open_period = Some(val);
        self
    }
    /// Sets the `close_date` parameter.
    pub fn close_date(mut self, val: i64) -> Self {
        self.close_date = Some(val);
        self
    }
    /// Sets the `is_closed` parameter.
    pub fn is_closed(mut self, val: bool) -> Self {
        self.is_closed = Some(val);
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `question_parse_mode` parameter.
    pub fn question_parse_mode(mut self, val: impl Into<String>) -> Self {
        self.question_parse_mode = Some(val.into());
        self
    }
    /// Sets the `question_entities` parameter.
    pub fn question_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.question_entities = Some(val);
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("sendPoll", &payload).await
    }
}

impl_into_future!(SendPollBuilder, message::Message);

// =========================================================================
// SendDiceBuilder
// =========================================================================

/// Builder for the [`sendDice`] API method.
#[derive(Serialize)]
pub struct SendDiceBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<reply::ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_messages_topic_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendDiceBuilder<'a> {
    /// Sets the `emoji` parameter.
    pub fn emoji(mut self, val: impl Into<String>) -> Self {
        self.emoji = Some(val.into());
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("sendDice", &payload).await
    }
}

impl_into_future!(SendDiceBuilder, message::Message);

// =========================================================================
// SendStickerBuilder
// =========================================================================

/// Builder for the [`sendSticker`] API method.
pub struct SendStickerBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    sticker: files::input_file::InputFile,
    emoji: Option<String>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<reply::ReplyParameters>,
    reply_markup: Option<serde_json::Value>,
    message_thread_id: Option<i64>,
    business_connection_id: Option<String>,
    message_effect_id: Option<String>,
    allow_paid_broadcast: Option<bool>,
    direct_messages_topic_id: Option<i64>,
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendStickerBuilder<'a> {
    /// Sets the `emoji` parameter.
    pub fn emoji(mut self, val: impl Into<String>) -> Self {
        self.emoji = Some(val.into());
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            input_file_param("sticker", self.sticker),
        ];
        push_opt_str(&mut params, "emoji", &self.emoji);
        push_opt(
            &mut params,
            "disable_notification",
            &self.disable_notification,
        )?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            &self.business_connection_id,
        );
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(
            &mut params,
            "allow_paid_broadcast",
            &self.allow_paid_broadcast,
        )?;
        push_opt(
            &mut params,
            "direct_messages_topic_id",
            &self.direct_messages_topic_id,
        )?;
        push_opt(
            &mut params,
            "suggested_post_parameters",
            &self.suggested_post_parameters,
        )?;
        self.bot.do_api_request("sendSticker", params).await
    }
}

impl_into_future!(SendStickerBuilder, message::Message);

// =========================================================================
// EditMessageTextBuilder
// =========================================================================

/// Builder for the [`editMessageText`] API method.
#[derive(Serialize)]
pub struct EditMessageTextBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<Vec<message_entity::MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_preview_options: Option<link_preview_options::LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
}

impl<'a> EditMessageTextBuilder<'a> {
    /// Sets the `chat_id` parameter.
    pub fn chat_id(mut self, val: impl Into<ChatId>) -> Self {
        self.chat_id = Some(val.into());
        self
    }
    /// Sets the `message_id` parameter.
    pub fn message_id(mut self, val: i64) -> Self {
        self.message_id = Some(val);
        self
    }
    /// Sets the `inline_message_id` parameter.
    pub fn inline_message_id(mut self, val: impl Into<String>) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }
    /// Sets the `parse_mode` parameter.
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self {
        self.parse_mode = Some(val.into());
        self
    }
    /// Sets the `entities` parameter.
    pub fn entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.entities = Some(val);
        self
    }
    /// Sets the `link_preview_options` parameter.
    pub fn link_preview_options(mut self, val: link_preview_options::LinkPreviewOptions) -> Self {
        self.link_preview_options = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<MessageOrBool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("editMessageText", &payload).await
    }
}

impl_into_future!(EditMessageTextBuilder, MessageOrBool);

// =========================================================================
// EditMessageCaptionBuilder
// =========================================================================

/// Builder for the [`editMessageCaption`] API method.
#[derive(Serialize)]
pub struct EditMessageCaptionBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<Vec<message_entity::MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
}

impl<'a> EditMessageCaptionBuilder<'a> {
    /// Sets the `chat_id` parameter.
    pub fn chat_id(mut self, val: impl Into<ChatId>) -> Self {
        self.chat_id = Some(val.into());
        self
    }
    /// Sets the `message_id` parameter.
    pub fn message_id(mut self, val: i64) -> Self {
        self.message_id = Some(val);
        self
    }
    /// Sets the `inline_message_id` parameter.
    pub fn inline_message_id(mut self, val: impl Into<String>) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }
    /// Sets the `caption` parameter.
    pub fn caption(mut self, val: impl Into<String>) -> Self {
        self.caption = Some(val.into());
        self
    }
    /// Sets the `parse_mode` parameter.
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self {
        self.parse_mode = Some(val.into());
        self
    }
    /// Sets the `caption_entities` parameter.
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.caption_entities = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `show_caption_above_media` parameter.
    pub fn show_caption_above_media(mut self, val: bool) -> Self {
        self.show_caption_above_media = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<MessageOrBool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("editMessageCaption", &payload).await
    }
}

impl_into_future!(EditMessageCaptionBuilder, MessageOrBool);

// =========================================================================
// EditMessageMediaBuilder
// =========================================================================

/// Builder for the [`editMessageMedia`] API method.
#[derive(Serialize)]
pub struct EditMessageMediaBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    media: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
}

impl<'a> EditMessageMediaBuilder<'a> {
    /// Sets the `chat_id` parameter.
    pub fn chat_id(mut self, val: impl Into<ChatId>) -> Self {
        self.chat_id = Some(val.into());
        self
    }
    /// Sets the `message_id` parameter.
    pub fn message_id(mut self, val: i64) -> Self {
        self.message_id = Some(val);
        self
    }
    /// Sets the `inline_message_id` parameter.
    pub fn inline_message_id(mut self, val: impl Into<String>) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<MessageOrBool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("editMessageMedia", &payload).await
    }
}

impl_into_future!(EditMessageMediaBuilder, MessageOrBool);

// =========================================================================
// EditMessageReplyMarkupBuilder
// =========================================================================

/// Builder for the [`editMessageReplyMarkup`] API method.
#[derive(Serialize)]
pub struct EditMessageReplyMarkupBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
}

impl<'a> EditMessageReplyMarkupBuilder<'a> {
    /// Sets the `chat_id` parameter.
    pub fn chat_id(mut self, val: impl Into<ChatId>) -> Self {
        self.chat_id = Some(val.into());
        self
    }
    /// Sets the `message_id` parameter.
    pub fn message_id(mut self, val: i64) -> Self {
        self.message_id = Some(val);
        self
    }
    /// Sets the `inline_message_id` parameter.
    pub fn inline_message_id(mut self, val: impl Into<String>) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<MessageOrBool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("editMessageReplyMarkup", &payload)
            .await
    }
}

impl_into_future!(EditMessageReplyMarkupBuilder, MessageOrBool);

// =========================================================================
// AnswerCallbackQueryBuilder
// =========================================================================

/// Builder for the [`answerCallbackQuery`] API method.
#[derive(Serialize)]
pub struct AnswerCallbackQueryBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    callback_query_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<i64>,
}

impl<'a> AnswerCallbackQueryBuilder<'a> {
    /// Sets the `text` parameter.
    pub fn text(mut self, val: impl Into<String>) -> Self {
        self.text = Some(val.into());
        self
    }
    /// Sets the `show_alert` parameter.
    pub fn show_alert(mut self, val: bool) -> Self {
        self.show_alert = Some(val);
        self
    }
    /// Sets the `url` parameter.
    pub fn url(mut self, val: impl Into<String>) -> Self {
        self.url = Some(val.into());
        self
    }
    /// Sets the `cache_time` parameter.
    pub fn cache_time(mut self, val: i64) -> Self {
        self.cache_time = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("answerCallbackQuery", &payload).await
    }
}

impl_into_future!(AnswerCallbackQueryBuilder, bool);

// =========================================================================
// AnswerInlineQueryBuilder
// =========================================================================

/// Builder for the [`answerInlineQuery`] API method.
#[derive(Serialize)]
pub struct AnswerInlineQueryBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    inline_query_id: String,
    results: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    button: Option<serde_json::Value>,
}

impl<'a> AnswerInlineQueryBuilder<'a> {
    /// Sets the `cache_time` parameter.
    pub fn cache_time(mut self, val: i64) -> Self {
        self.cache_time = Some(val);
        self
    }
    /// Sets the `is_personal` parameter.
    pub fn is_personal(mut self, val: bool) -> Self {
        self.is_personal = Some(val);
        self
    }
    /// Sets the `next_offset` parameter.
    pub fn next_offset(mut self, val: impl Into<String>) -> Self {
        self.next_offset = Some(val.into());
        self
    }
    /// Sets the `button` parameter.
    pub fn button(mut self, val: serde_json::Value) -> Self {
        self.button = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("answerInlineQuery", &payload).await
    }
}

impl_into_future!(AnswerInlineQueryBuilder, bool);

// =========================================================================
// SetWebhookBuilder
// =========================================================================

/// Builder for the [`setWebhook`] API method.
pub struct SetWebhookBuilder<'a> {
    bot: &'a Bot,
    url: String,
    certificate: Option<files::input_file::InputFile>,
    ip_address: Option<String>,
    max_connections: Option<i32>,
    allowed_updates: Option<Vec<String>>,
    drop_pending_updates: Option<bool>,
    secret_token: Option<String>,
}

impl<'a> SetWebhookBuilder<'a> {
    /// Sets the `certificate` parameter.
    pub fn certificate(mut self, val: files::input_file::InputFile) -> Self {
        self.certificate = Some(val);
        self
    }
    /// Sets the `ip_address` parameter.
    pub fn ip_address(mut self, val: impl Into<String>) -> Self {
        self.ip_address = Some(val.into());
        self
    }
    /// Sets the `max_connections` parameter.
    pub fn max_connections(mut self, val: i32) -> Self {
        self.max_connections = Some(val);
        self
    }
    /// Sets the `allowed_updates` parameter.
    pub fn allowed_updates(mut self, val: Vec<String>) -> Self {
        self.allowed_updates = Some(val);
        self
    }
    /// Sets the `drop_pending_updates` parameter.
    pub fn drop_pending_updates(mut self, val: bool) -> Self {
        self.drop_pending_updates = Some(val);
        self
    }
    /// Sets the `secret_token` parameter.
    pub fn secret_token(mut self, val: impl Into<String>) -> Self {
        self.secret_token = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "url",
            serde_json::Value::String(self.url),
        )];
        push_opt_file(&mut params, "certificate", self.certificate);
        push_opt_str(&mut params, "ip_address", &self.ip_address);
        push_opt(&mut params, "max_connections", &self.max_connections)?;
        push_opt(&mut params, "allowed_updates", &self.allowed_updates)?;
        push_opt(
            &mut params,
            "drop_pending_updates",
            &self.drop_pending_updates,
        )?;
        push_opt_str(&mut params, "secret_token", &self.secret_token);
        self.bot.do_api_request("setWebhook", params).await
    }
}

impl_into_future!(SetWebhookBuilder, bool);

// =========================================================================
// DeleteWebhookBuilder
// =========================================================================

/// Builder for the [`deleteWebhook`] API method.
#[derive(Serialize)]
pub struct DeleteWebhookBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_pending_updates: Option<bool>,
}

impl<'a> DeleteWebhookBuilder<'a> {
    /// Sets the `drop_pending_updates` parameter.
    pub fn drop_pending_updates(mut self, val: bool) -> Self {
        self.drop_pending_updates = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("deleteWebhook", &payload).await
    }
}

impl_into_future!(DeleteWebhookBuilder, bool);

// =========================================================================
// GetFileBuilder
// =========================================================================

/// Builder for the [`getFile`] API method.
#[derive(Serialize)]
pub struct GetFileBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    file_id: String,
}

impl<'a> GetFileBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<files::file::File> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("getFile", &payload).await
    }
}

impl_into_future!(GetFileBuilder, files::file::File);

// =========================================================================
// SendInvoiceBuilder
// =========================================================================

/// Builder for the [`sendInvoice`] API method.
#[derive(Serialize)]
pub struct SendInvoiceBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    title: String,
    description: String,
    payload: String,
    currency: String,
    prices: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tip_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_tip_amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_parameter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_flexible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<reply::ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_messages_topic_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendInvoiceBuilder<'a> {
    /// Sets the `provider_token` parameter.
    pub fn provider_token(mut self, val: impl Into<String>) -> Self {
        self.provider_token = Some(val.into());
        self
    }
    /// Sets the `max_tip_amount` parameter.
    pub fn max_tip_amount(mut self, val: i64) -> Self {
        self.max_tip_amount = Some(val);
        self
    }
    /// Sets the `suggested_tip_amounts` parameter.
    pub fn suggested_tip_amounts(mut self, val: Vec<i64>) -> Self {
        self.suggested_tip_amounts = Some(val);
        self
    }
    /// Sets the `start_parameter` parameter.
    pub fn start_parameter(mut self, val: impl Into<String>) -> Self {
        self.start_parameter = Some(val.into());
        self
    }
    /// Sets the `provider_data` parameter.
    pub fn provider_data(mut self, val: impl Into<String>) -> Self {
        self.provider_data = Some(val.into());
        self
    }
    /// Sets the `photo_url` parameter.
    pub fn photo_url(mut self, val: impl Into<String>) -> Self {
        self.photo_url = Some(val.into());
        self
    }
    /// Sets the `photo_size` parameter.
    pub fn photo_size(mut self, val: i64) -> Self {
        self.photo_size = Some(val);
        self
    }
    /// Sets the `photo_width` parameter.
    pub fn photo_width(mut self, val: i64) -> Self {
        self.photo_width = Some(val);
        self
    }
    /// Sets the `photo_height` parameter.
    pub fn photo_height(mut self, val: i64) -> Self {
        self.photo_height = Some(val);
        self
    }
    /// Sets the `need_name` parameter.
    pub fn need_name(mut self, val: bool) -> Self {
        self.need_name = Some(val);
        self
    }
    /// Sets the `need_phone_number` parameter.
    pub fn need_phone_number(mut self, val: bool) -> Self {
        self.need_phone_number = Some(val);
        self
    }
    /// Sets the `need_email` parameter.
    pub fn need_email(mut self, val: bool) -> Self {
        self.need_email = Some(val);
        self
    }
    /// Sets the `need_shipping_address` parameter.
    pub fn need_shipping_address(mut self, val: bool) -> Self {
        self.need_shipping_address = Some(val);
        self
    }
    /// Sets the `send_phone_number_to_provider` parameter.
    pub fn send_phone_number_to_provider(mut self, val: bool) -> Self {
        self.send_phone_number_to_provider = Some(val);
        self
    }
    /// Sets the `send_email_to_provider` parameter.
    pub fn send_email_to_provider(mut self, val: bool) -> Self {
        self.send_email_to_provider = Some(val);
        self
    }
    /// Sets the `is_flexible` parameter.
    pub fn is_flexible(mut self, val: bool) -> Self {
        self.is_flexible = Some(val);
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("sendInvoice", &payload).await
    }
}

impl_into_future!(SendInvoiceBuilder, message::Message);

// =========================================================================
// AnswerShippingQueryBuilder
// =========================================================================

/// Builder for the [`answerShippingQuery`] API method.
#[derive(Serialize)]
pub struct AnswerShippingQueryBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    shipping_query_id: String,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
}

impl<'a> AnswerShippingQueryBuilder<'a> {
    /// Sets the `shipping_options` parameter.
    pub fn shipping_options(mut self, val: Vec<serde_json::Value>) -> Self {
        self.shipping_options = Some(val);
        self
    }
    /// Sets the `error_message` parameter.
    pub fn error_message(mut self, val: impl Into<String>) -> Self {
        self.error_message = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("answerShippingQuery", &payload).await
    }
}

impl_into_future!(AnswerShippingQueryBuilder, bool);

// =========================================================================
// AnswerPreCheckoutQueryBuilder
// =========================================================================

/// Builder for the [`answerPreCheckoutQuery`] API method.
#[derive(Serialize)]
pub struct AnswerPreCheckoutQueryBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    pre_checkout_query_id: String,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
}

impl<'a> AnswerPreCheckoutQueryBuilder<'a> {
    /// Sets the `error_message` parameter.
    pub fn error_message(mut self, val: impl Into<String>) -> Self {
        self.error_message = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("answerPreCheckoutQuery", &payload)
            .await
    }
}

impl_into_future!(AnswerPreCheckoutQueryBuilder, bool);

// =========================================================================
// Bot convenience methods that return builders
// =========================================================================

impl Bot {
    // -- Sending messages -------------------------------------------------

    /// Build a `sendMessage` request.
    pub fn send_message(
        &self,
        chat_id: impl Into<ChatId>,
        text: impl Into<String>,
    ) -> SendMessageBuilder<'_> {
        SendMessageBuilder {
            bot: self,
            chat_id: chat_id.into(),
            text: text.into(),
            parse_mode: None,
            entities: None,
            link_preview_options: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            business_connection_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    // -- Sending media ----------------------------------------------------

    /// Build a `sendPhoto` request.
    pub fn send_photo(
        &self,
        chat_id: impl Into<ChatId>,
        photo: files::input_file::InputFile,
    ) -> SendPhotoBuilder<'_> {
        SendPhotoBuilder {
            bot: self,
            chat_id: chat_id.into(),
            photo,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            has_spoiler: None,
            business_connection_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            show_caption_above_media: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    /// Build a `sendDocument` request.
    pub fn send_document(
        &self,
        chat_id: impl Into<ChatId>,
        document: files::input_file::InputFile,
    ) -> SendDocumentBuilder<'_> {
        SendDocumentBuilder {
            bot: self,
            chat_id: chat_id.into(),
            document,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
            thumbnail: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            business_connection_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    /// Build a `sendVideo` request.
    pub fn send_video(
        &self,
        chat_id: impl Into<ChatId>,
        video: files::input_file::InputFile,
    ) -> SendVideoBuilder<'_> {
        SendVideoBuilder {
            bot: self,
            chat_id: chat_id.into(),
            video,
            duration: None,
            width: None,
            height: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            supports_streaming: None,
            thumbnail: None,
            has_spoiler: None,
            show_caption_above_media: None,
            cover: None,
            start_timestamp: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            business_connection_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    /// Build a `sendAudio` request.
    pub fn send_audio(
        &self,
        chat_id: impl Into<ChatId>,
        audio: files::input_file::InputFile,
    ) -> SendAudioBuilder<'_> {
        SendAudioBuilder {
            bot: self,
            chat_id: chat_id.into(),
            audio,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            performer: None,
            title: None,
            thumbnail: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            business_connection_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    /// Build a `sendAnimation` request.
    pub fn send_animation(
        &self,
        chat_id: impl Into<ChatId>,
        animation: files::input_file::InputFile,
    ) -> SendAnimationBuilder<'_> {
        SendAnimationBuilder {
            bot: self,
            chat_id: chat_id.into(),
            animation,
            duration: None,
            width: None,
            height: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            thumbnail: None,
            has_spoiler: None,
            show_caption_above_media: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            business_connection_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    /// Build a `sendVoice` request.
    pub fn send_voice(
        &self,
        chat_id: impl Into<ChatId>,
        voice: files::input_file::InputFile,
    ) -> SendVoiceBuilder<'_> {
        SendVoiceBuilder {
            bot: self,
            chat_id: chat_id.into(),
            voice,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            business_connection_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    /// Build a `sendVideoNote` request.
    pub fn send_video_note(
        &self,
        chat_id: impl Into<ChatId>,
        video_note: files::input_file::InputFile,
    ) -> SendVideoNoteBuilder<'_> {
        SendVideoNoteBuilder {
            bot: self,
            chat_id: chat_id.into(),
            video_note,
            duration: None,
            length: None,
            thumbnail: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            business_connection_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    // -- Sending other content --------------------------------------------

    /// Build a `sendLocation` request.
    pub fn send_location(
        &self,
        chat_id: impl Into<ChatId>,
        latitude: f64,
        longitude: f64,
    ) -> SendLocationBuilder<'_> {
        SendLocationBuilder {
            bot: self,
            chat_id: chat_id.into(),
            latitude,
            longitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            business_connection_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    /// Build a `sendVenue` request.
    pub fn send_venue(
        &self,
        chat_id: impl Into<ChatId>,
        latitude: f64,
        longitude: f64,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> SendVenueBuilder<'_> {
        SendVenueBuilder {
            bot: self,
            chat_id: chat_id.into(),
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            business_connection_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    /// Build a `sendContact` request.
    pub fn send_contact(
        &self,
        chat_id: impl Into<ChatId>,
        phone_number: impl Into<String>,
        first_name: impl Into<String>,
    ) -> SendContactBuilder<'_> {
        SendContactBuilder {
            bot: self,
            chat_id: chat_id.into(),
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            business_connection_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    /// Build a `sendPoll` request.
    pub fn send_poll(
        &self,
        chat_id: impl Into<ChatId>,
        question: impl Into<String>,
        options: Vec<serde_json::Value>,
    ) -> SendPollBuilder<'_> {
        SendPollBuilder {
            bot: self,
            chat_id: chat_id.into(),
            question: question.into(),
            options,
            is_anonymous: None,
            poll_type: None,
            allows_multiple_answers: None,
            correct_option_id: None,
            explanation: None,
            explanation_parse_mode: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
            is_closed: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            business_connection_id: None,
            question_parse_mode: None,
            question_entities: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    /// Build a `sendDice` request.
    pub fn send_dice(&self, chat_id: impl Into<ChatId>) -> SendDiceBuilder<'_> {
        SendDiceBuilder {
            bot: self,
            chat_id: chat_id.into(),
            emoji: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            business_connection_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    // -- Stickers ---------------------------------------------------------

    /// Build a `sendSticker` request.
    pub fn send_sticker(
        &self,
        chat_id: impl Into<ChatId>,
        sticker: files::input_file::InputFile,
    ) -> SendStickerBuilder<'_> {
        SendStickerBuilder {
            bot: self,
            chat_id: chat_id.into(),
            sticker,
            emoji: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            business_connection_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    // -- Editing messages -------------------------------------------------

    /// Build an `editMessageText` request.
    pub fn edit_message_text(&self, text: impl Into<String>) -> EditMessageTextBuilder<'_> {
        EditMessageTextBuilder {
            bot: self,
            text: text.into(),
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            parse_mode: None,
            entities: None,
            link_preview_options: None,
            reply_markup: None,
            business_connection_id: None,
        }
    }

    /// Build an `editMessageCaption` request.
    pub fn edit_message_caption(&self) -> EditMessageCaptionBuilder<'_> {
        EditMessageCaptionBuilder {
            bot: self,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            show_caption_above_media: None,
            business_connection_id: None,
        }
    }

    /// Build an `editMessageMedia` request.
    pub fn edit_message_media(&self, media: serde_json::Value) -> EditMessageMediaBuilder<'_> {
        EditMessageMediaBuilder {
            bot: self,
            media,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
            business_connection_id: None,
        }
    }

    /// Build an `editMessageReplyMarkup` request.
    pub fn edit_message_reply_markup(&self) -> EditMessageReplyMarkupBuilder<'_> {
        EditMessageReplyMarkupBuilder {
            bot: self,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
            business_connection_id: None,
        }
    }

    // -- Callback & inline queries ----------------------------------------

    /// Build an `answerCallbackQuery` request.
    pub fn answer_callback_query(
        &self,
        callback_query_id: impl Into<String>,
    ) -> AnswerCallbackQueryBuilder<'_> {
        AnswerCallbackQueryBuilder {
            bot: self,
            callback_query_id: callback_query_id.into(),
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
        }
    }

    /// Build an `answerInlineQuery` request.
    pub fn answer_inline_query(
        &self,
        inline_query_id: impl Into<String>,
        results: Vec<serde_json::Value>,
    ) -> AnswerInlineQueryBuilder<'_> {
        AnswerInlineQueryBuilder {
            bot: self,
            inline_query_id: inline_query_id.into(),
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            button: None,
        }
    }

    // -- Webhooks ------------------------------------------------------------

    /// Build a `setWebhook` request.
    pub fn set_webhook(&self, url: impl Into<String>) -> SetWebhookBuilder<'_> {
        SetWebhookBuilder {
            bot: self,
            url: url.into(),
            certificate: None,
            ip_address: None,
            max_connections: None,
            allowed_updates: None,
            drop_pending_updates: None,
            secret_token: None,
        }
    }

    /// Build a `deleteWebhook` request.
    pub fn delete_webhook(&self) -> DeleteWebhookBuilder<'_> {
        DeleteWebhookBuilder {
            bot: self,
            drop_pending_updates: None,
        }
    }

    // -- Files ---------------------------------------------------------------

    /// Build a `getFile` request.
    pub fn get_file(&self, file_id: impl Into<String>) -> GetFileBuilder<'_> {
        GetFileBuilder {
            bot: self,
            file_id: file_id.into(),
        }
    }

    // -- Payments ------------------------------------------------------------

    /// Build a `sendInvoice` request.
    pub fn send_invoice(
        &self,
        chat_id: impl Into<ChatId>,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        currency: impl Into<String>,
        prices: Vec<serde_json::Value>,
    ) -> SendInvoiceBuilder<'_> {
        SendInvoiceBuilder {
            bot: self,
            chat_id: chat_id.into(),
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            currency: currency.into(),
            prices,
            provider_token: None,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            start_parameter: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    /// Build an `answerShippingQuery` request.
    pub fn answer_shipping_query(
        &self,
        shipping_query_id: impl Into<String>,
        ok: bool,
    ) -> AnswerShippingQueryBuilder<'_> {
        AnswerShippingQueryBuilder {
            bot: self,
            shipping_query_id: shipping_query_id.into(),
            ok,
            shipping_options: None,
            error_message: None,
        }
    }

    /// Build an `answerPreCheckoutQuery` request.
    pub fn answer_pre_checkout_query(
        &self,
        pre_checkout_query_id: impl Into<String>,
        ok: bool,
    ) -> AnswerPreCheckoutQueryBuilder<'_> {
        AnswerPreCheckoutQueryBuilder {
            bot: self,
            pre_checkout_query_id: pre_checkout_query_id.into(),
            ok,
            error_message: None,
        }
    }
}

// =========================================================================
// GetManagedBotTokenBuilder
// =========================================================================

/// Builder for the [`getManagedBotToken`] API method.
#[derive(Serialize)]
pub struct GetManagedBotTokenBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    bot_user_id: i64,
}

impl<'a> GetManagedBotTokenBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<String> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("getManagedBotToken", &payload).await
    }
}

impl_into_future!(GetManagedBotTokenBuilder, String);

// =========================================================================
// ReplaceManagedBotTokenBuilder
// =========================================================================

/// Builder for the [`replaceManagedBotToken`] API method.
#[derive(Serialize)]
pub struct ReplaceManagedBotTokenBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    bot_user_id: i64,
}

impl<'a> ReplaceManagedBotTokenBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<String> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("replaceManagedBotToken", &payload)
            .await
    }
}

impl_into_future!(ReplaceManagedBotTokenBuilder, String);

// =========================================================================
// SavePreparedKeyboardButtonBuilder
// =========================================================================

/// Builder for the [`savePreparedKeyboardButton`] API method.
#[derive(Serialize)]
pub struct SavePreparedKeyboardButtonBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    user_id: i64,
    button: inline::inline_keyboard_button::InlineKeyboardButton,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_user_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_bot_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_group_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_channel_chats: Option<bool>,
}

impl<'a> SavePreparedKeyboardButtonBuilder<'a> {
    /// Sets the `allow_user_chats` parameter.
    pub fn allow_user_chats(mut self, val: bool) -> Self {
        self.allow_user_chats = Some(val);
        self
    }
    /// Sets the `allow_bot_chats` parameter.
    pub fn allow_bot_chats(mut self, val: bool) -> Self {
        self.allow_bot_chats = Some(val);
        self
    }
    /// Sets the `allow_group_chats` parameter.
    pub fn allow_group_chats(mut self, val: bool) -> Self {
        self.allow_group_chats = Some(val);
        self
    }
    /// Sets the `allow_channel_chats` parameter.
    pub fn allow_channel_chats(mut self, val: bool) -> Self {
        self.allow_channel_chats = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<prepared_keyboard_button::PreparedKeyboardButton> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("savePreparedKeyboardButton", &payload)
            .await
    }
}

impl_into_future!(
    SavePreparedKeyboardButtonBuilder,
    prepared_keyboard_button::PreparedKeyboardButton
);

// =========================================================================
// Builder factory methods on Bot for the new API methods
// =========================================================================

impl Bot {
    /// Build a `getManagedBotToken` request.
    pub fn get_managed_bot_token(&self, bot_user_id: i64) -> GetManagedBotTokenBuilder<'_> {
        GetManagedBotTokenBuilder {
            bot: self,
            bot_user_id,
        }
    }

    /// Build a `replaceManagedBotToken` request.
    pub fn replace_managed_bot_token(&self, bot_user_id: i64) -> ReplaceManagedBotTokenBuilder<'_> {
        ReplaceManagedBotTokenBuilder {
            bot: self,
            bot_user_id,
        }
    }

    /// Build a `savePreparedKeyboardButton` request.
    pub fn save_prepared_keyboard_button(
        &self,
        user_id: i64,
        button: inline::inline_keyboard_button::InlineKeyboardButton,
    ) -> SavePreparedKeyboardButtonBuilder<'_> {
        SavePreparedKeyboardButtonBuilder {
            bot: self,
            user_id,
            button,
            allow_user_chats: None,
            allow_bot_chats: None,
            allow_group_chats: None,
            allow_channel_chats: None,
        }
    }

    /// Build a `sendChatAction` request.
    pub fn send_chat_action(
        &self,
        chat_id: impl Into<ChatId>,
        action: impl Into<String>,
    ) -> SendChatActionBuilder<'_> {
        SendChatActionBuilder {
            bot: self,
            chat_id: chat_id.into(),
            action: action.into(),
            message_thread_id: None,
            business_connection_id: None,
        }
    }
}

// ---------------------------------------------------------------------------
// SendChatActionBuilder
// ---------------------------------------------------------------------------

/// Builder for the [`sendChatAction`] API method.
#[derive(Serialize)]
pub struct SendChatActionBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
}

impl<'a> SendChatActionBuilder<'a> {
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, id: i64) -> Self {
        self.message_thread_id = Some(id);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, id: impl Into<String>) -> Self {
        self.business_connection_id = Some(id.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("sendChatAction", &payload).await
    }
}

impl_into_future!(SendChatActionBuilder, bool);

// =========================================================================
// CopyMessageBuilder
// =========================================================================

/// Builder for the [`copyMessage`] API method.
#[derive(Serialize)]
pub struct CopyMessageBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<Vec<message_entity::MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<reply::ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_start_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_messages_topic_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_effect_id: Option<String>,
}

impl<'a> CopyMessageBuilder<'a> {
    /// Sets the `caption` parameter.
    pub fn caption(mut self, val: impl Into<String>) -> Self {
        self.caption = Some(val.into());
        self
    }
    /// Sets the `parse_mode` parameter.
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self {
        self.parse_mode = Some(val.into());
        self
    }
    /// Sets the `caption_entities` parameter.
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.caption_entities = Some(val);
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `show_caption_above_media` parameter.
    pub fn show_caption_above_media(mut self, val: bool) -> Self {
        self.show_caption_above_media = Some(val);
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `video_start_timestamp` parameter.
    pub fn video_start_timestamp(mut self, val: i64) -> Self {
        self.video_start_timestamp = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message_id::MessageId> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("copyMessage", &payload).await
    }
}

impl_into_future!(CopyMessageBuilder, message_id::MessageId);

// =========================================================================
// CopyMessagesBuilder
// =========================================================================

/// Builder for the [`copyMessages`] API method.
#[derive(Serialize)]
pub struct CopyMessagesBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_ids: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove_caption: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_messages_topic_id: Option<i64>,
}

impl<'a> CopyMessagesBuilder<'a> {
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `remove_caption` parameter.
    pub fn remove_caption(mut self, val: bool) -> Self {
        self.remove_caption = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<Vec<message_id::MessageId>> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("copyMessages", &payload).await
    }
}

impl_into_future!(CopyMessagesBuilder, Vec<message_id::MessageId>);

// =========================================================================
// DeleteMessageBuilder
// =========================================================================

/// Builder for the [`deleteMessage`] API method.
#[derive(Serialize)]
pub struct DeleteMessageBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    message_id: i64,
}

impl<'a> DeleteMessageBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("deleteMessage", &payload).await
    }
}

impl_into_future!(DeleteMessageBuilder, bool);

// =========================================================================
// DeleteMessagesBuilder
// =========================================================================

/// Builder for the [`deleteMessages`] API method.
#[derive(Serialize)]
pub struct DeleteMessagesBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    message_ids: Vec<i64>,
}

impl<'a> DeleteMessagesBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("deleteMessages", &payload).await
    }
}

impl_into_future!(DeleteMessagesBuilder, bool);

// =========================================================================
// ForwardMessageBuilder
// =========================================================================

/// Builder for the [`forwardMessage`] API method.
#[derive(Serialize)]
pub struct ForwardMessageBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_start_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_messages_topic_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_effect_id: Option<String>,
}

impl<'a> ForwardMessageBuilder<'a> {
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `video_start_timestamp` parameter.
    pub fn video_start_timestamp(mut self, val: i64) -> Self {
        self.video_start_timestamp = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("forwardMessage", &payload).await
    }
}

impl_into_future!(ForwardMessageBuilder, message::Message);

// =========================================================================
// ForwardMessagesBuilder
// =========================================================================

/// Builder for the [`forwardMessages`] API method.
#[derive(Serialize)]
pub struct ForwardMessagesBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_ids: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_messages_topic_id: Option<i64>,
}

impl<'a> ForwardMessagesBuilder<'a> {
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<Vec<message_id::MessageId>> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("forwardMessages", &payload).await
    }
}

impl_into_future!(ForwardMessagesBuilder, Vec<message_id::MessageId>);

// =========================================================================
// SendMessageDraftBuilder
// =========================================================================

/// Builder for the [`sendMessageDraft`] API method.
#[derive(Serialize)]
pub struct SendMessageDraftBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: i64,
    draft_id: i64,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<Vec<message_entity::MessageEntity>>,
}

impl<'a> SendMessageDraftBuilder<'a> {
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `parse_mode` parameter.
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self {
        self.parse_mode = Some(val.into());
        self
    }
    /// Sets the `entities` parameter.
    pub fn entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.entities = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("sendMessageDraft", &payload).await
    }
}

impl_into_future!(SendMessageDraftBuilder, bool);

// =========================================================================
// LeaveChatBuilder
// =========================================================================

/// Builder for the [`leaveChat`] API method.
#[derive(Serialize)]
pub struct LeaveChatBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
}

impl<'a> LeaveChatBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("leaveChat", &payload).await
    }
}

impl_into_future!(LeaveChatBuilder, bool);

// =========================================================================
// GetChatBuilder
// =========================================================================

/// Builder for the [`getChat`] API method.
#[derive(Serialize)]
pub struct GetChatBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
}

impl<'a> GetChatBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<chat_full_info::ChatFullInfo> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("getChat", &payload).await
    }
}

impl_into_future!(GetChatBuilder, chat_full_info::ChatFullInfo);

// =========================================================================
// GetChatAdministratorsBuilder
// =========================================================================

/// Builder for the [`getChatAdministrators`] API method.
#[derive(Serialize)]
pub struct GetChatAdministratorsBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
}

impl<'a> GetChatAdministratorsBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<Vec<chat_member::ChatMember>> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("getChatAdministrators", &payload)
            .await
    }
}

impl_into_future!(GetChatAdministratorsBuilder, Vec<chat_member::ChatMember>);

// =========================================================================
// GetChatMemberCountBuilder
// =========================================================================

/// Builder for the [`getChatMemberCount`] API method.
#[derive(Serialize)]
pub struct GetChatMemberCountBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
}

impl<'a> GetChatMemberCountBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<i64> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("getChatMemberCount", &payload).await
    }
}

impl_into_future!(GetChatMemberCountBuilder, i64);

// =========================================================================
// GetChatMemberBuilder
// =========================================================================

/// Builder for the [`getChatMember`] API method.
#[derive(Serialize)]
pub struct GetChatMemberBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    user_id: i64,
}

impl<'a> GetChatMemberBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<chat_member::ChatMember> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("getChatMember", &payload).await
    }
}

impl_into_future!(GetChatMemberBuilder, chat_member::ChatMember);

// =========================================================================
// BanChatMemberBuilder
// =========================================================================

/// Builder for the [`banChatMember`] API method.
#[derive(Serialize)]
pub struct BanChatMemberBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoke_messages: Option<bool>,
}

impl<'a> BanChatMemberBuilder<'a> {
    /// Sets the `until_date` parameter.
    pub fn until_date(mut self, val: i64) -> Self {
        self.until_date = Some(val);
        self
    }
    /// Sets the `revoke_messages` parameter.
    pub fn revoke_messages(mut self, val: bool) -> Self {
        self.revoke_messages = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("banChatMember", &payload).await
    }
}

impl_into_future!(BanChatMemberBuilder, bool);

// =========================================================================
// UnbanChatMemberBuilder
// =========================================================================

/// Builder for the [`unbanChatMember`] API method.
#[derive(Serialize)]
pub struct UnbanChatMemberBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_if_banned: Option<bool>,
}

impl<'a> UnbanChatMemberBuilder<'a> {
    /// Sets the `only_if_banned` parameter.
    pub fn only_if_banned(mut self, val: bool) -> Self {
        self.only_if_banned = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("unbanChatMember", &payload).await
    }
}

impl_into_future!(UnbanChatMemberBuilder, bool);

// =========================================================================
// BanChatSenderChatBuilder
// =========================================================================

/// Builder for the [`banChatSenderChat`] API method.
#[derive(Serialize)]
pub struct BanChatSenderChatBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    sender_chat_id: i64,
}

impl<'a> BanChatSenderChatBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("banChatSenderChat", &payload).await
    }
}

impl_into_future!(BanChatSenderChatBuilder, bool);

// =========================================================================
// UnbanChatSenderChatBuilder
// =========================================================================

/// Builder for the [`unbanChatSenderChat`] API method.
#[derive(Serialize)]
pub struct UnbanChatSenderChatBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    sender_chat_id: i64,
}

impl<'a> UnbanChatSenderChatBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("unbanChatSenderChat", &payload).await
    }
}

impl_into_future!(UnbanChatSenderChatBuilder, bool);

// =========================================================================
// RestrictChatMemberBuilder
// =========================================================================

/// Builder for the [`restrictChatMember`] API method.
#[derive(Serialize)]
pub struct RestrictChatMemberBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    user_id: i64,
    permissions: chat_permissions::ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_independent_chat_permissions: Option<bool>,
}

impl<'a> RestrictChatMemberBuilder<'a> {
    /// Sets the `until_date` parameter.
    pub fn until_date(mut self, val: i64) -> Self {
        self.until_date = Some(val);
        self
    }
    /// Sets the `use_independent_chat_permissions` parameter.
    pub fn use_independent_chat_permissions(mut self, val: bool) -> Self {
        self.use_independent_chat_permissions = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("restrictChatMember", &payload).await
    }
}

impl_into_future!(RestrictChatMemberBuilder, bool);

// =========================================================================
// PromoteChatMemberBuilder
// =========================================================================

/// Builder for the [`promoteChatMember`] API method.
#[derive(Serialize)]
pub struct PromoteChatMemberBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_chat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_post_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_delete_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_video_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_restrict_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_promote_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_topics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_post_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_edit_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_delete_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_direct_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_tags: Option<bool>,
}

impl<'a> PromoteChatMemberBuilder<'a> {
    /// Sets the `is_anonymous` parameter.
    pub fn is_anonymous(mut self, val: bool) -> Self {
        self.is_anonymous = Some(val);
        self
    }
    /// Sets the `can_manage_chat` parameter.
    pub fn can_manage_chat(mut self, val: bool) -> Self {
        self.can_manage_chat = Some(val);
        self
    }
    /// Sets the `can_post_messages` parameter.
    pub fn can_post_messages(mut self, val: bool) -> Self {
        self.can_post_messages = Some(val);
        self
    }
    /// Sets the `can_edit_messages` parameter.
    pub fn can_edit_messages(mut self, val: bool) -> Self {
        self.can_edit_messages = Some(val);
        self
    }
    /// Sets the `can_delete_messages` parameter.
    pub fn can_delete_messages(mut self, val: bool) -> Self {
        self.can_delete_messages = Some(val);
        self
    }
    /// Sets the `can_manage_video_chats` parameter.
    pub fn can_manage_video_chats(mut self, val: bool) -> Self {
        self.can_manage_video_chats = Some(val);
        self
    }
    /// Sets the `can_restrict_members` parameter.
    pub fn can_restrict_members(mut self, val: bool) -> Self {
        self.can_restrict_members = Some(val);
        self
    }
    /// Sets the `can_promote_members` parameter.
    pub fn can_promote_members(mut self, val: bool) -> Self {
        self.can_promote_members = Some(val);
        self
    }
    /// Sets the `can_change_info` parameter.
    pub fn can_change_info(mut self, val: bool) -> Self {
        self.can_change_info = Some(val);
        self
    }
    /// Sets the `can_invite_users` parameter.
    pub fn can_invite_users(mut self, val: bool) -> Self {
        self.can_invite_users = Some(val);
        self
    }
    /// Sets the `can_pin_messages` parameter.
    pub fn can_pin_messages(mut self, val: bool) -> Self {
        self.can_pin_messages = Some(val);
        self
    }
    /// Sets the `can_manage_topics` parameter.
    pub fn can_manage_topics(mut self, val: bool) -> Self {
        self.can_manage_topics = Some(val);
        self
    }
    /// Sets the `can_post_stories` parameter.
    pub fn can_post_stories(mut self, val: bool) -> Self {
        self.can_post_stories = Some(val);
        self
    }
    /// Sets the `can_edit_stories` parameter.
    pub fn can_edit_stories(mut self, val: bool) -> Self {
        self.can_edit_stories = Some(val);
        self
    }
    /// Sets the `can_delete_stories` parameter.
    pub fn can_delete_stories(mut self, val: bool) -> Self {
        self.can_delete_stories = Some(val);
        self
    }
    /// Sets the `can_manage_direct_messages` parameter.
    pub fn can_manage_direct_messages(mut self, val: bool) -> Self {
        self.can_manage_direct_messages = Some(val);
        self
    }
    /// Sets the `can_manage_tags` parameter.
    pub fn can_manage_tags(mut self, val: bool) -> Self {
        self.can_manage_tags = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("promoteChatMember", &payload).await
    }
}

impl_into_future!(PromoteChatMemberBuilder, bool);

// =========================================================================
// SetChatAdministratorCustomTitleBuilder
// =========================================================================

/// Builder for the [`setChatAdministratorCustomTitle`] API method.
#[derive(Serialize)]
pub struct SetChatAdministratorCustomTitleBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    user_id: i64,
    custom_title: String,
}

impl<'a> SetChatAdministratorCustomTitleBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("setChatAdministratorCustomTitle", &payload)
            .await
    }
}

impl_into_future!(SetChatAdministratorCustomTitleBuilder, bool);

// =========================================================================
// SetChatPermissionsBuilder
// =========================================================================

/// Builder for the [`setChatPermissions`] API method.
#[derive(Serialize)]
pub struct SetChatPermissionsBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    permissions: chat_permissions::ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_independent_chat_permissions: Option<bool>,
}

impl<'a> SetChatPermissionsBuilder<'a> {
    /// Sets the `use_independent_chat_permissions` parameter.
    pub fn use_independent_chat_permissions(mut self, val: bool) -> Self {
        self.use_independent_chat_permissions = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("setChatPermissions", &payload).await
    }
}

impl_into_future!(SetChatPermissionsBuilder, bool);

// =========================================================================
// SetChatPhotoBuilder
// =========================================================================

/// Builder for the [`setChatPhoto`] API method.
///
/// This builder uses multipart form data since it uploads a file.
pub struct SetChatPhotoBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    photo: files::input_file::InputFile,
}

impl<'a> SetChatPhotoBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            input_file_param("photo", self.photo),
        ];
        self.bot.do_api_request("setChatPhoto", params).await
    }
}

impl_into_future!(SetChatPhotoBuilder, bool);

// =========================================================================
// DeleteChatPhotoBuilder
// =========================================================================

/// Builder for the [`deleteChatPhoto`] API method.
#[derive(Serialize)]
pub struct DeleteChatPhotoBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
}

impl<'a> DeleteChatPhotoBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("deleteChatPhoto", &payload).await
    }
}

impl_into_future!(DeleteChatPhotoBuilder, bool);

// =========================================================================
// SetChatTitleBuilder
// =========================================================================

/// Builder for the [`setChatTitle`] API method.
#[derive(Serialize)]
pub struct SetChatTitleBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    title: String,
}

impl<'a> SetChatTitleBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("setChatTitle", &payload).await
    }
}

impl_into_future!(SetChatTitleBuilder, bool);

// =========================================================================
// SetChatDescriptionBuilder
// =========================================================================

/// Builder for the [`setChatDescription`] API method.
#[derive(Serialize)]
pub struct SetChatDescriptionBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl<'a> SetChatDescriptionBuilder<'a> {
    /// Sets the `description` parameter.
    pub fn description(mut self, val: impl Into<String>) -> Self {
        self.description = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("setChatDescription", &payload).await
    }
}

impl_into_future!(SetChatDescriptionBuilder, bool);

// =========================================================================
// SetChatStickerSetBuilder
// =========================================================================

/// Builder for the [`setChatStickerSet`] API method.
#[derive(Serialize)]
pub struct SetChatStickerSetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    sticker_set_name: String,
}

impl<'a> SetChatStickerSetBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("setChatStickerSet", &payload).await
    }
}

impl_into_future!(SetChatStickerSetBuilder, bool);

// =========================================================================
// DeleteChatStickerSetBuilder
// =========================================================================

/// Builder for the [`deleteChatStickerSet`] API method.
#[derive(Serialize)]
pub struct DeleteChatStickerSetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
}

impl<'a> DeleteChatStickerSetBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("deleteChatStickerSet", &payload)
            .await
    }
}

impl_into_future!(DeleteChatStickerSetBuilder, bool);

// =========================================================================
// SetChatMemberTagBuilder
// =========================================================================

/// Builder for the [`setChatMemberTag`] API method.
#[derive(Serialize)]
pub struct SetChatMemberTagBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
}

impl<'a> SetChatMemberTagBuilder<'a> {
    /// Sets the `tag` parameter.
    pub fn tag(mut self, val: impl Into<String>) -> Self {
        self.tag = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("setChatMemberTag", &payload).await
    }
}

impl_into_future!(SetChatMemberTagBuilder, bool);

// =========================================================================
// PinChatMessageBuilder
// =========================================================================

/// Builder for the [`pinChatMessage`] API method.
#[derive(Serialize)]
pub struct PinChatMessageBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
}

impl<'a> PinChatMessageBuilder<'a> {
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("pinChatMessage", &payload).await
    }
}

impl_into_future!(PinChatMessageBuilder, bool);

// =========================================================================
// UnpinChatMessageBuilder
// =========================================================================

/// Builder for the [`unpinChatMessage`] API method.
#[derive(Serialize)]
pub struct UnpinChatMessageBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
}

impl<'a> UnpinChatMessageBuilder<'a> {
    /// Sets the `message_id` parameter.
    pub fn message_id(mut self, val: i64) -> Self {
        self.message_id = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("unpinChatMessage", &payload).await
    }
}

impl_into_future!(UnpinChatMessageBuilder, bool);

// =========================================================================
// UnpinAllChatMessagesBuilder
// =========================================================================

/// Builder for the [`unpinAllChatMessages`] API method.
#[derive(Serialize)]
pub struct UnpinAllChatMessagesBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
}

impl<'a> UnpinAllChatMessagesBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("unpinAllChatMessages", &payload)
            .await
    }
}

impl_into_future!(UnpinAllChatMessagesBuilder, bool);

// =========================================================================
// ExportChatInviteLinkBuilder
// =========================================================================

/// Builder for the [`exportChatInviteLink`] API method.
#[derive(Serialize)]
pub struct ExportChatInviteLinkBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
}

impl<'a> ExportChatInviteLinkBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<String> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("exportChatInviteLink", &payload)
            .await
    }
}

impl_into_future!(ExportChatInviteLinkBuilder, String);

// =========================================================================
// CreateChatInviteLinkBuilder
// =========================================================================

/// Builder for the [`createChatInviteLink`] API method.
#[derive(Serialize)]
pub struct CreateChatInviteLinkBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creates_join_request: Option<bool>,
}

impl<'a> CreateChatInviteLinkBuilder<'a> {
    /// Sets the `expire_date` parameter.
    pub fn expire_date(mut self, val: i64) -> Self {
        self.expire_date = Some(val);
        self
    }
    /// Sets the `member_limit` parameter.
    pub fn member_limit(mut self, val: i64) -> Self {
        self.member_limit = Some(val);
        self
    }
    /// Sets the `name` parameter.
    pub fn name(mut self, val: impl Into<String>) -> Self {
        self.name = Some(val.into());
        self
    }
    /// Sets the `creates_join_request` parameter.
    pub fn creates_join_request(mut self, val: bool) -> Self {
        self.creates_join_request = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<chat_invite_link::ChatInviteLink> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("createChatInviteLink", &payload)
            .await
    }
}

impl_into_future!(
    CreateChatInviteLinkBuilder,
    chat_invite_link::ChatInviteLink
);

// =========================================================================
// EditChatInviteLinkBuilder
// =========================================================================

/// Builder for the [`editChatInviteLink`] API method.
#[derive(Serialize)]
pub struct EditChatInviteLinkBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creates_join_request: Option<bool>,
}

impl<'a> EditChatInviteLinkBuilder<'a> {
    /// Sets the `expire_date` parameter.
    pub fn expire_date(mut self, val: i64) -> Self {
        self.expire_date = Some(val);
        self
    }
    /// Sets the `member_limit` parameter.
    pub fn member_limit(mut self, val: i64) -> Self {
        self.member_limit = Some(val);
        self
    }
    /// Sets the `name` parameter.
    pub fn name(mut self, val: impl Into<String>) -> Self {
        self.name = Some(val.into());
        self
    }
    /// Sets the `creates_join_request` parameter.
    pub fn creates_join_request(mut self, val: bool) -> Self {
        self.creates_join_request = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<chat_invite_link::ChatInviteLink> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("editChatInviteLink", &payload).await
    }
}

impl_into_future!(EditChatInviteLinkBuilder, chat_invite_link::ChatInviteLink);

// =========================================================================
// RevokeChatInviteLinkBuilder
// =========================================================================

/// Builder for the [`revokeChatInviteLink`] API method.
#[derive(Serialize)]
pub struct RevokeChatInviteLinkBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    invite_link: String,
}

impl<'a> RevokeChatInviteLinkBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<chat_invite_link::ChatInviteLink> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("revokeChatInviteLink", &payload)
            .await
    }
}

impl_into_future!(
    RevokeChatInviteLinkBuilder,
    chat_invite_link::ChatInviteLink
);

// =========================================================================
// CreateChatSubscriptionInviteLinkBuilder
// =========================================================================

/// Builder for the [`createChatSubscriptionInviteLink`] API method.
#[derive(Serialize)]
pub struct CreateChatSubscriptionInviteLinkBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    subscription_period: i64,
    subscription_price: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl<'a> CreateChatSubscriptionInviteLinkBuilder<'a> {
    /// Sets the `name` parameter.
    pub fn name(mut self, val: impl Into<String>) -> Self {
        self.name = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<chat_invite_link::ChatInviteLink> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("createChatSubscriptionInviteLink", &payload)
            .await
    }
}

impl_into_future!(
    CreateChatSubscriptionInviteLinkBuilder,
    chat_invite_link::ChatInviteLink
);

// =========================================================================
// EditChatSubscriptionInviteLinkBuilder
// =========================================================================

/// Builder for the [`editChatSubscriptionInviteLink`] API method.
#[derive(Serialize)]
pub struct EditChatSubscriptionInviteLinkBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl<'a> EditChatSubscriptionInviteLinkBuilder<'a> {
    /// Sets the `name` parameter.
    pub fn name(mut self, val: impl Into<String>) -> Self {
        self.name = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<chat_invite_link::ChatInviteLink> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("editChatSubscriptionInviteLink", &payload)
            .await
    }
}

impl_into_future!(
    EditChatSubscriptionInviteLinkBuilder,
    chat_invite_link::ChatInviteLink
);

// =========================================================================
// ApproveChatJoinRequestBuilder
// =========================================================================

/// Builder for the [`approveChatJoinRequest`] API method.
#[derive(Serialize)]
pub struct ApproveChatJoinRequestBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    user_id: i64,
}

impl<'a> ApproveChatJoinRequestBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("approveChatJoinRequest", &payload)
            .await
    }
}

impl_into_future!(ApproveChatJoinRequestBuilder, bool);

// =========================================================================
// DeclineChatJoinRequestBuilder
// =========================================================================

/// Builder for the [`declineChatJoinRequest`] API method.
#[derive(Serialize)]
pub struct DeclineChatJoinRequestBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    user_id: i64,
}

impl<'a> DeclineChatJoinRequestBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("declineChatJoinRequest", &payload)
            .await
    }
}

impl_into_future!(DeclineChatJoinRequestBuilder, bool);

// =========================================================================
// Builder factory methods on Bot for messages and chat methods
// =========================================================================

impl Bot {
    // -- Message methods ---------------------------------------------------

    /// Build a `copyMessage` request.
    pub fn copy_message(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> CopyMessageBuilder<'_> {
        CopyMessageBuilder {
            bot: self,
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            show_caption_above_media: None,
            allow_paid_broadcast: None,
            video_start_timestamp: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
            message_effect_id: None,
        }
    }

    /// Build a `copyMessages` request.
    pub fn copy_messages(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_ids: Vec<i64>,
    ) -> CopyMessagesBuilder<'_> {
        CopyMessagesBuilder {
            bot: self,
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_ids,
            disable_notification: None,
            protect_content: None,
            message_thread_id: None,
            remove_caption: None,
            direct_messages_topic_id: None,
        }
    }

    /// Build a `deleteMessage` request.
    pub fn delete_message(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> DeleteMessageBuilder<'_> {
        DeleteMessageBuilder {
            bot: self,
            chat_id: chat_id.into(),
            message_id,
        }
    }

    /// Build a `deleteMessages` request.
    pub fn delete_messages(
        &self,
        chat_id: impl Into<ChatId>,
        message_ids: Vec<i64>,
    ) -> DeleteMessagesBuilder<'_> {
        DeleteMessagesBuilder {
            bot: self,
            chat_id: chat_id.into(),
            message_ids,
        }
    }

    /// Build a `forwardMessage` request.
    pub fn forward_message(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> ForwardMessageBuilder<'_> {
        ForwardMessageBuilder {
            bot: self,
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            disable_notification: None,
            protect_content: None,
            message_thread_id: None,
            video_start_timestamp: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
            message_effect_id: None,
        }
    }

    /// Build a `forwardMessages` request.
    pub fn forward_messages(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_ids: Vec<i64>,
    ) -> ForwardMessagesBuilder<'_> {
        ForwardMessagesBuilder {
            bot: self,
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_ids,
            disable_notification: None,
            protect_content: None,
            message_thread_id: None,
            direct_messages_topic_id: None,
        }
    }

    /// Build a `sendMessageDraft` request.
    pub fn send_message_draft(
        &self,
        chat_id: i64,
        draft_id: i64,
        text: impl Into<String>,
    ) -> SendMessageDraftBuilder<'_> {
        SendMessageDraftBuilder {
            bot: self,
            chat_id,
            draft_id,
            text: text.into(),
            message_thread_id: None,
            parse_mode: None,
            entities: None,
        }
    }

    // -- Chat management methods -------------------------------------------

    /// Build a `leaveChat` request.
    pub fn leave_chat(&self, chat_id: impl Into<ChatId>) -> LeaveChatBuilder<'_> {
        LeaveChatBuilder {
            bot: self,
            chat_id: chat_id.into(),
        }
    }

    /// Build a `getChat` request.
    pub fn get_chat(&self, chat_id: impl Into<ChatId>) -> GetChatBuilder<'_> {
        GetChatBuilder {
            bot: self,
            chat_id: chat_id.into(),
        }
    }

    /// Build a `getChatAdministrators` request.
    pub fn get_chat_administrators(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> GetChatAdministratorsBuilder<'_> {
        GetChatAdministratorsBuilder {
            bot: self,
            chat_id: chat_id.into(),
        }
    }

    /// Build a `getChatMemberCount` request.
    pub fn get_chat_member_count(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> GetChatMemberCountBuilder<'_> {
        GetChatMemberCountBuilder {
            bot: self,
            chat_id: chat_id.into(),
        }
    }

    /// Build a `getChatMember` request.
    pub fn get_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> GetChatMemberBuilder<'_> {
        GetChatMemberBuilder {
            bot: self,
            chat_id: chat_id.into(),
            user_id,
        }
    }

    /// Build a `banChatMember` request.
    pub fn ban_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> BanChatMemberBuilder<'_> {
        BanChatMemberBuilder {
            bot: self,
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
            revoke_messages: None,
        }
    }

    /// Build an `unbanChatMember` request.
    pub fn unban_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> UnbanChatMemberBuilder<'_> {
        UnbanChatMemberBuilder {
            bot: self,
            chat_id: chat_id.into(),
            user_id,
            only_if_banned: None,
        }
    }

    /// Build a `banChatSenderChat` request.
    pub fn ban_chat_sender_chat(
        &self,
        chat_id: impl Into<ChatId>,
        sender_chat_id: i64,
    ) -> BanChatSenderChatBuilder<'_> {
        BanChatSenderChatBuilder {
            bot: self,
            chat_id: chat_id.into(),
            sender_chat_id,
        }
    }

    /// Build an `unbanChatSenderChat` request.
    pub fn unban_chat_sender_chat(
        &self,
        chat_id: impl Into<ChatId>,
        sender_chat_id: i64,
    ) -> UnbanChatSenderChatBuilder<'_> {
        UnbanChatSenderChatBuilder {
            bot: self,
            chat_id: chat_id.into(),
            sender_chat_id,
        }
    }

    /// Build a `restrictChatMember` request.
    pub fn restrict_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
        permissions: chat_permissions::ChatPermissions,
    ) -> RestrictChatMemberBuilder<'_> {
        RestrictChatMemberBuilder {
            bot: self,
            chat_id: chat_id.into(),
            user_id,
            permissions,
            until_date: None,
            use_independent_chat_permissions: None,
        }
    }

    /// Build a `promoteChatMember` request.
    pub fn promote_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> PromoteChatMemberBuilder<'_> {
        PromoteChatMemberBuilder {
            bot: self,
            chat_id: chat_id.into(),
            user_id,
            is_anonymous: None,
            can_manage_chat: None,
            can_post_messages: None,
            can_edit_messages: None,
            can_delete_messages: None,
            can_manage_video_chats: None,
            can_restrict_members: None,
            can_promote_members: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
            can_manage_topics: None,
            can_post_stories: None,
            can_edit_stories: None,
            can_delete_stories: None,
            can_manage_direct_messages: None,
            can_manage_tags: None,
        }
    }

    /// Build a `setChatAdministratorCustomTitle` request.
    pub fn set_chat_administrator_custom_title(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
        custom_title: impl Into<String>,
    ) -> SetChatAdministratorCustomTitleBuilder<'_> {
        SetChatAdministratorCustomTitleBuilder {
            bot: self,
            chat_id: chat_id.into(),
            user_id,
            custom_title: custom_title.into(),
        }
    }

    /// Build a `setChatPermissions` request.
    pub fn set_chat_permissions(
        &self,
        chat_id: impl Into<ChatId>,
        permissions: chat_permissions::ChatPermissions,
    ) -> SetChatPermissionsBuilder<'_> {
        SetChatPermissionsBuilder {
            bot: self,
            chat_id: chat_id.into(),
            permissions,
            use_independent_chat_permissions: None,
        }
    }

    /// Build a `setChatPhoto` request.
    pub fn set_chat_photo(
        &self,
        chat_id: impl Into<ChatId>,
        photo: files::input_file::InputFile,
    ) -> SetChatPhotoBuilder<'_> {
        SetChatPhotoBuilder {
            bot: self,
            chat_id: chat_id.into(),
            photo,
        }
    }

    /// Build a `deleteChatPhoto` request.
    pub fn delete_chat_photo(&self, chat_id: impl Into<ChatId>) -> DeleteChatPhotoBuilder<'_> {
        DeleteChatPhotoBuilder {
            bot: self,
            chat_id: chat_id.into(),
        }
    }

    /// Build a `setChatTitle` request.
    pub fn set_chat_title(
        &self,
        chat_id: impl Into<ChatId>,
        title: impl Into<String>,
    ) -> SetChatTitleBuilder<'_> {
        SetChatTitleBuilder {
            bot: self,
            chat_id: chat_id.into(),
            title: title.into(),
        }
    }

    /// Build a `setChatDescription` request.
    pub fn set_chat_description(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> SetChatDescriptionBuilder<'_> {
        SetChatDescriptionBuilder {
            bot: self,
            chat_id: chat_id.into(),
            description: None,
        }
    }

    /// Build a `setChatStickerSet` request.
    pub fn set_chat_sticker_set(
        &self,
        chat_id: impl Into<ChatId>,
        sticker_set_name: impl Into<String>,
    ) -> SetChatStickerSetBuilder<'_> {
        SetChatStickerSetBuilder {
            bot: self,
            chat_id: chat_id.into(),
            sticker_set_name: sticker_set_name.into(),
        }
    }

    /// Build a `deleteChatStickerSet` request.
    pub fn delete_chat_sticker_set(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> DeleteChatStickerSetBuilder<'_> {
        DeleteChatStickerSetBuilder {
            bot: self,
            chat_id: chat_id.into(),
        }
    }

    /// Build a `setChatMemberTag` request.
    pub fn set_chat_member_tag(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> SetChatMemberTagBuilder<'_> {
        SetChatMemberTagBuilder {
            bot: self,
            chat_id: chat_id.into(),
            user_id,
            tag: None,
        }
    }

    // -- Chat pinning methods ----------------------------------------------

    /// Build a `pinChatMessage` request.
    pub fn pin_chat_message(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> PinChatMessageBuilder<'_> {
        PinChatMessageBuilder {
            bot: self,
            chat_id: chat_id.into(),
            message_id,
            disable_notification: None,
            business_connection_id: None,
        }
    }

    /// Build an `unpinChatMessage` request.
    pub fn unpin_chat_message(&self, chat_id: impl Into<ChatId>) -> UnpinChatMessageBuilder<'_> {
        UnpinChatMessageBuilder {
            bot: self,
            chat_id: chat_id.into(),
            message_id: None,
            business_connection_id: None,
        }
    }

    /// Build an `unpinAllChatMessages` request.
    pub fn unpin_all_chat_messages(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> UnpinAllChatMessagesBuilder<'_> {
        UnpinAllChatMessagesBuilder {
            bot: self,
            chat_id: chat_id.into(),
        }
    }

    // -- Chat invite link methods ------------------------------------------

    /// Build an `exportChatInviteLink` request.
    pub fn export_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> ExportChatInviteLinkBuilder<'_> {
        ExportChatInviteLinkBuilder {
            bot: self,
            chat_id: chat_id.into(),
        }
    }

    /// Build a `createChatInviteLink` request.
    pub fn create_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> CreateChatInviteLinkBuilder<'_> {
        CreateChatInviteLinkBuilder {
            bot: self,
            chat_id: chat_id.into(),
            expire_date: None,
            member_limit: None,
            name: None,
            creates_join_request: None,
        }
    }

    /// Build an `editChatInviteLink` request.
    pub fn edit_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> EditChatInviteLinkBuilder<'_> {
        EditChatInviteLinkBuilder {
            bot: self,
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
            expire_date: None,
            member_limit: None,
            name: None,
            creates_join_request: None,
        }
    }

    /// Build a `revokeChatInviteLink` request.
    pub fn revoke_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> RevokeChatInviteLinkBuilder<'_> {
        RevokeChatInviteLinkBuilder {
            bot: self,
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
        }
    }

    /// Build a `createChatSubscriptionInviteLink` request.
    pub fn create_chat_subscription_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        subscription_period: i64,
        subscription_price: i64,
    ) -> CreateChatSubscriptionInviteLinkBuilder<'_> {
        CreateChatSubscriptionInviteLinkBuilder {
            bot: self,
            chat_id: chat_id.into(),
            subscription_period,
            subscription_price,
            name: None,
        }
    }

    /// Build an `editChatSubscriptionInviteLink` request.
    pub fn edit_chat_subscription_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> EditChatSubscriptionInviteLinkBuilder<'_> {
        EditChatSubscriptionInviteLinkBuilder {
            bot: self,
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
            name: None,
        }
    }

    /// Build an `approveChatJoinRequest` request.
    pub fn approve_chat_join_request(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> ApproveChatJoinRequestBuilder<'_> {
        ApproveChatJoinRequestBuilder {
            bot: self,
            chat_id: chat_id.into(),
            user_id,
        }
    }

    /// Build a `declineChatJoinRequest` request.
    pub fn decline_chat_join_request(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> DeclineChatJoinRequestBuilder<'_> {
        DeclineChatJoinRequestBuilder {
            bot: self,
            chat_id: chat_id.into(),
            user_id,
        }
    }
}
