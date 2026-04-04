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
use crate::types::*;
use serde::Serialize;

macro_rules! impl_into_future {
    ($builder:ident, $output:ty) => {
        impl<'a> std::future::IntoFuture for $builder<'a> {
            type Output = Result<$output>;
            type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'a>>;
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
                value: Some(serde_json::Value::String(format!("__filepath__:{path_str}"))),
                input_files: Some(vec![file_ref]),
            }
        }
    }
}

// =========================================================================
// SendMessageBuilder
// =========================================================================

pub struct SendMessageBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    text: String,
    parse_mode: Option<String>,
    entities: Option<Vec<message_entity::MessageEntity>>,
    link_preview_options: Option<link_preview_options::LinkPreviewOptions>,
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

impl<'a> SendMessageBuilder<'a> {
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self { self.parse_mode = Some(val.into()); self }
    pub fn entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self { self.entities = Some(val); self }
    pub fn link_preview_options(mut self, val: link_preview_options::LinkPreviewOptions) -> Self { self.link_preview_options = Some(val); self }
    pub fn disable_notification(mut self, val: bool) -> Self { self.disable_notification = Some(val); self }
    pub fn protect_content(mut self, val: bool) -> Self { self.protect_content = Some(val); self }
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self { self.reply_parameters = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn message_thread_id(mut self, val: i64) -> Self { self.message_thread_id = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self { self.message_effect_id = Some(val.into()); self }
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self { self.allow_paid_broadcast = Some(val); self }
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self { self.direct_messages_topic_id = Some(val); self }
    pub fn suggested_post_parameters(mut self, val: suggested_post::SuggestedPostParameters) -> Self { self.suggested_post_parameters = Some(val); self }

    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            RequestParameter::new("text", serde_json::Value::String(self.text)),
        ];
        push_opt_str(&mut params, "parse_mode", &self.parse_mode);
        push_opt(&mut params, "entities", &self.entities)?;
        push_opt(&mut params, "link_preview_options", &self.link_preview_options)?;
        push_opt(&mut params, "disable_notification", &self.disable_notification)?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &self.allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &self.direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &self.suggested_post_parameters)?;
        self.bot.do_api_request("sendMessage", params).await
    }
}

impl_into_future!(SendMessageBuilder, message::Message);

// =========================================================================
// SendPhotoBuilder
// =========================================================================

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
    pub fn caption(mut self, val: impl Into<String>) -> Self { self.caption = Some(val.into()); self }
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self { self.parse_mode = Some(val.into()); self }
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self { self.caption_entities = Some(val); self }
    pub fn disable_notification(mut self, val: bool) -> Self { self.disable_notification = Some(val); self }
    pub fn protect_content(mut self, val: bool) -> Self { self.protect_content = Some(val); self }
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self { self.reply_parameters = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn message_thread_id(mut self, val: i64) -> Self { self.message_thread_id = Some(val); self }
    pub fn has_spoiler(mut self, val: bool) -> Self { self.has_spoiler = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self { self.message_effect_id = Some(val.into()); self }
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self { self.allow_paid_broadcast = Some(val); self }
    pub fn show_caption_above_media(mut self, val: bool) -> Self { self.show_caption_above_media = Some(val); self }
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self { self.direct_messages_topic_id = Some(val); self }
    pub fn suggested_post_parameters(mut self, val: suggested_post::SuggestedPostParameters) -> Self { self.suggested_post_parameters = Some(val); self }

    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            input_file_param("photo", self.photo),
        ];
        push_opt_str(&mut params, "caption", &self.caption);
        push_opt_str(&mut params, "parse_mode", &self.parse_mode);
        push_opt(&mut params, "caption_entities", &self.caption_entities)?;
        push_opt(&mut params, "disable_notification", &self.disable_notification)?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt(&mut params, "has_spoiler", &self.has_spoiler)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &self.allow_paid_broadcast)?;
        push_opt(&mut params, "show_caption_above_media", &self.show_caption_above_media)?;
        push_opt(&mut params, "direct_messages_topic_id", &self.direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &self.suggested_post_parameters)?;
        self.bot.do_api_request("sendPhoto", params).await
    }
}

impl_into_future!(SendPhotoBuilder, message::Message);

// =========================================================================
// SendDocumentBuilder
// =========================================================================

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
    pub fn caption(mut self, val: impl Into<String>) -> Self { self.caption = Some(val.into()); self }
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self { self.parse_mode = Some(val.into()); self }
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self { self.caption_entities = Some(val); self }
    pub fn disable_content_type_detection(mut self, val: bool) -> Self { self.disable_content_type_detection = Some(val); self }
    pub fn thumbnail(mut self, val: files::input_file::InputFile) -> Self { self.thumbnail = Some(val); self }
    pub fn disable_notification(mut self, val: bool) -> Self { self.disable_notification = Some(val); self }
    pub fn protect_content(mut self, val: bool) -> Self { self.protect_content = Some(val); self }
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self { self.reply_parameters = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn message_thread_id(mut self, val: i64) -> Self { self.message_thread_id = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self { self.message_effect_id = Some(val.into()); self }
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self { self.allow_paid_broadcast = Some(val); self }
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self { self.direct_messages_topic_id = Some(val); self }
    pub fn suggested_post_parameters(mut self, val: suggested_post::SuggestedPostParameters) -> Self { self.suggested_post_parameters = Some(val); self }

    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            input_file_param("document", self.document),
        ];
        push_opt_str(&mut params, "caption", &self.caption);
        push_opt_str(&mut params, "parse_mode", &self.parse_mode);
        push_opt(&mut params, "caption_entities", &self.caption_entities)?;
        push_opt(&mut params, "disable_content_type_detection", &self.disable_content_type_detection)?;
        push_opt_file(&mut params, "thumbnail", self.thumbnail);
        push_opt(&mut params, "disable_notification", &self.disable_notification)?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &self.allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &self.direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &self.suggested_post_parameters)?;
        self.bot.do_api_request("sendDocument", params).await
    }
}

impl_into_future!(SendDocumentBuilder, message::Message);

// =========================================================================
// SendVideoBuilder
// =========================================================================

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
    pub fn duration(mut self, val: i64) -> Self { self.duration = Some(val); self }
    pub fn width(mut self, val: i64) -> Self { self.width = Some(val); self }
    pub fn height(mut self, val: i64) -> Self { self.height = Some(val); self }
    pub fn caption(mut self, val: impl Into<String>) -> Self { self.caption = Some(val.into()); self }
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self { self.parse_mode = Some(val.into()); self }
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self { self.caption_entities = Some(val); self }
    pub fn supports_streaming(mut self, val: bool) -> Self { self.supports_streaming = Some(val); self }
    pub fn thumbnail(mut self, val: files::input_file::InputFile) -> Self { self.thumbnail = Some(val); self }
    pub fn has_spoiler(mut self, val: bool) -> Self { self.has_spoiler = Some(val); self }
    pub fn show_caption_above_media(mut self, val: bool) -> Self { self.show_caption_above_media = Some(val); self }
    pub fn cover(mut self, val: files::input_file::InputFile) -> Self { self.cover = Some(val); self }
    pub fn start_timestamp(mut self, val: i64) -> Self { self.start_timestamp = Some(val); self }
    pub fn disable_notification(mut self, val: bool) -> Self { self.disable_notification = Some(val); self }
    pub fn protect_content(mut self, val: bool) -> Self { self.protect_content = Some(val); self }
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self { self.reply_parameters = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn message_thread_id(mut self, val: i64) -> Self { self.message_thread_id = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self { self.message_effect_id = Some(val.into()); self }
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self { self.allow_paid_broadcast = Some(val); self }
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self { self.direct_messages_topic_id = Some(val); self }
    pub fn suggested_post_parameters(mut self, val: suggested_post::SuggestedPostParameters) -> Self { self.suggested_post_parameters = Some(val); self }

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
        push_opt(&mut params, "show_caption_above_media", &self.show_caption_above_media)?;
        push_opt_file(&mut params, "cover", self.cover);
        push_opt(&mut params, "start_timestamp", &self.start_timestamp)?;
        push_opt(&mut params, "disable_notification", &self.disable_notification)?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &self.allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &self.direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &self.suggested_post_parameters)?;
        self.bot.do_api_request("sendVideo", params).await
    }
}

impl_into_future!(SendVideoBuilder, message::Message);

// =========================================================================
// SendAudioBuilder
// =========================================================================

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
    pub fn caption(mut self, val: impl Into<String>) -> Self { self.caption = Some(val.into()); self }
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self { self.parse_mode = Some(val.into()); self }
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self { self.caption_entities = Some(val); self }
    pub fn duration(mut self, val: i64) -> Self { self.duration = Some(val); self }
    pub fn performer(mut self, val: impl Into<String>) -> Self { self.performer = Some(val.into()); self }
    pub fn title(mut self, val: impl Into<String>) -> Self { self.title = Some(val.into()); self }
    pub fn thumbnail(mut self, val: files::input_file::InputFile) -> Self { self.thumbnail = Some(val); self }
    pub fn disable_notification(mut self, val: bool) -> Self { self.disable_notification = Some(val); self }
    pub fn protect_content(mut self, val: bool) -> Self { self.protect_content = Some(val); self }
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self { self.reply_parameters = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn message_thread_id(mut self, val: i64) -> Self { self.message_thread_id = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self { self.message_effect_id = Some(val.into()); self }
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self { self.allow_paid_broadcast = Some(val); self }
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self { self.direct_messages_topic_id = Some(val); self }
    pub fn suggested_post_parameters(mut self, val: suggested_post::SuggestedPostParameters) -> Self { self.suggested_post_parameters = Some(val); self }

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
        push_opt(&mut params, "disable_notification", &self.disable_notification)?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &self.allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &self.direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &self.suggested_post_parameters)?;
        self.bot.do_api_request("sendAudio", params).await
    }
}

impl_into_future!(SendAudioBuilder, message::Message);

// =========================================================================
// SendAnimationBuilder
// =========================================================================

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
    pub fn duration(mut self, val: i64) -> Self { self.duration = Some(val); self }
    pub fn width(mut self, val: i64) -> Self { self.width = Some(val); self }
    pub fn height(mut self, val: i64) -> Self { self.height = Some(val); self }
    pub fn caption(mut self, val: impl Into<String>) -> Self { self.caption = Some(val.into()); self }
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self { self.parse_mode = Some(val.into()); self }
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self { self.caption_entities = Some(val); self }
    pub fn thumbnail(mut self, val: files::input_file::InputFile) -> Self { self.thumbnail = Some(val); self }
    pub fn has_spoiler(mut self, val: bool) -> Self { self.has_spoiler = Some(val); self }
    pub fn show_caption_above_media(mut self, val: bool) -> Self { self.show_caption_above_media = Some(val); self }
    pub fn disable_notification(mut self, val: bool) -> Self { self.disable_notification = Some(val); self }
    pub fn protect_content(mut self, val: bool) -> Self { self.protect_content = Some(val); self }
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self { self.reply_parameters = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn message_thread_id(mut self, val: i64) -> Self { self.message_thread_id = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self { self.message_effect_id = Some(val.into()); self }
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self { self.allow_paid_broadcast = Some(val); self }
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self { self.direct_messages_topic_id = Some(val); self }
    pub fn suggested_post_parameters(mut self, val: suggested_post::SuggestedPostParameters) -> Self { self.suggested_post_parameters = Some(val); self }

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
        push_opt(&mut params, "show_caption_above_media", &self.show_caption_above_media)?;
        push_opt(&mut params, "disable_notification", &self.disable_notification)?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &self.allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &self.direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &self.suggested_post_parameters)?;
        self.bot.do_api_request("sendAnimation", params).await
    }
}

impl_into_future!(SendAnimationBuilder, message::Message);

// =========================================================================
// SendVoiceBuilder
// =========================================================================

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
    pub fn caption(mut self, val: impl Into<String>) -> Self { self.caption = Some(val.into()); self }
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self { self.parse_mode = Some(val.into()); self }
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self { self.caption_entities = Some(val); self }
    pub fn duration(mut self, val: i64) -> Self { self.duration = Some(val); self }
    pub fn disable_notification(mut self, val: bool) -> Self { self.disable_notification = Some(val); self }
    pub fn protect_content(mut self, val: bool) -> Self { self.protect_content = Some(val); self }
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self { self.reply_parameters = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn message_thread_id(mut self, val: i64) -> Self { self.message_thread_id = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self { self.message_effect_id = Some(val.into()); self }
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self { self.allow_paid_broadcast = Some(val); self }
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self { self.direct_messages_topic_id = Some(val); self }
    pub fn suggested_post_parameters(mut self, val: suggested_post::SuggestedPostParameters) -> Self { self.suggested_post_parameters = Some(val); self }

    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            input_file_param("voice", self.voice),
        ];
        push_opt_str(&mut params, "caption", &self.caption);
        push_opt_str(&mut params, "parse_mode", &self.parse_mode);
        push_opt(&mut params, "caption_entities", &self.caption_entities)?;
        push_opt(&mut params, "duration", &self.duration)?;
        push_opt(&mut params, "disable_notification", &self.disable_notification)?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &self.allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &self.direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &self.suggested_post_parameters)?;
        self.bot.do_api_request("sendVoice", params).await
    }
}

impl_into_future!(SendVoiceBuilder, message::Message);

// =========================================================================
// SendVideoNoteBuilder
// =========================================================================

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
    pub fn duration(mut self, val: i64) -> Self { self.duration = Some(val); self }
    pub fn length(mut self, val: i64) -> Self { self.length = Some(val); self }
    pub fn thumbnail(mut self, val: files::input_file::InputFile) -> Self { self.thumbnail = Some(val); self }
    pub fn disable_notification(mut self, val: bool) -> Self { self.disable_notification = Some(val); self }
    pub fn protect_content(mut self, val: bool) -> Self { self.protect_content = Some(val); self }
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self { self.reply_parameters = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn message_thread_id(mut self, val: i64) -> Self { self.message_thread_id = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self { self.message_effect_id = Some(val.into()); self }
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self { self.allow_paid_broadcast = Some(val); self }
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self { self.direct_messages_topic_id = Some(val); self }
    pub fn suggested_post_parameters(mut self, val: suggested_post::SuggestedPostParameters) -> Self { self.suggested_post_parameters = Some(val); self }

    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            input_file_param("video_note", self.video_note),
        ];
        push_opt(&mut params, "duration", &self.duration)?;
        push_opt(&mut params, "length", &self.length)?;
        push_opt_file(&mut params, "thumbnail", self.thumbnail);
        push_opt(&mut params, "disable_notification", &self.disable_notification)?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &self.allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &self.direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &self.suggested_post_parameters)?;
        self.bot.do_api_request("sendVideoNote", params).await
    }
}

impl_into_future!(SendVideoNoteBuilder, message::Message);

// =========================================================================
// SendLocationBuilder
// =========================================================================

pub struct SendLocationBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    latitude: f64,
    longitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<i64>,
    heading: Option<i64>,
    proximity_alert_radius: Option<i64>,
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

impl<'a> SendLocationBuilder<'a> {
    pub fn horizontal_accuracy(mut self, val: f64) -> Self { self.horizontal_accuracy = Some(val); self }
    pub fn live_period(mut self, val: i64) -> Self { self.live_period = Some(val); self }
    pub fn heading(mut self, val: i64) -> Self { self.heading = Some(val); self }
    pub fn proximity_alert_radius(mut self, val: i64) -> Self { self.proximity_alert_radius = Some(val); self }
    pub fn disable_notification(mut self, val: bool) -> Self { self.disable_notification = Some(val); self }
    pub fn protect_content(mut self, val: bool) -> Self { self.protect_content = Some(val); self }
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self { self.reply_parameters = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn message_thread_id(mut self, val: i64) -> Self { self.message_thread_id = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self { self.message_effect_id = Some(val.into()); self }
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self { self.allow_paid_broadcast = Some(val); self }
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self { self.direct_messages_topic_id = Some(val); self }
    pub fn suggested_post_parameters(mut self, val: suggested_post::SuggestedPostParameters) -> Self { self.suggested_post_parameters = Some(val); self }

    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            RequestParameter::new("latitude", serde_json::to_value(self.latitude)?),
            RequestParameter::new("longitude", serde_json::to_value(self.longitude)?),
        ];
        push_opt(&mut params, "horizontal_accuracy", &self.horizontal_accuracy)?;
        push_opt(&mut params, "live_period", &self.live_period)?;
        push_opt(&mut params, "heading", &self.heading)?;
        push_opt(&mut params, "proximity_alert_radius", &self.proximity_alert_radius)?;
        push_opt(&mut params, "disable_notification", &self.disable_notification)?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &self.allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &self.direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &self.suggested_post_parameters)?;
        self.bot.do_api_request("sendLocation", params).await
    }
}

impl_into_future!(SendLocationBuilder, message::Message);

// =========================================================================
// SendVenueBuilder
// =========================================================================

pub struct SendVenueBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
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

impl<'a> SendVenueBuilder<'a> {
    pub fn foursquare_id(mut self, val: impl Into<String>) -> Self { self.foursquare_id = Some(val.into()); self }
    pub fn foursquare_type(mut self, val: impl Into<String>) -> Self { self.foursquare_type = Some(val.into()); self }
    pub fn google_place_id(mut self, val: impl Into<String>) -> Self { self.google_place_id = Some(val.into()); self }
    pub fn google_place_type(mut self, val: impl Into<String>) -> Self { self.google_place_type = Some(val.into()); self }
    pub fn disable_notification(mut self, val: bool) -> Self { self.disable_notification = Some(val); self }
    pub fn protect_content(mut self, val: bool) -> Self { self.protect_content = Some(val); self }
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self { self.reply_parameters = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn message_thread_id(mut self, val: i64) -> Self { self.message_thread_id = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self { self.message_effect_id = Some(val.into()); self }
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self { self.allow_paid_broadcast = Some(val); self }
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self { self.direct_messages_topic_id = Some(val); self }
    pub fn suggested_post_parameters(mut self, val: suggested_post::SuggestedPostParameters) -> Self { self.suggested_post_parameters = Some(val); self }

    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            RequestParameter::new("latitude", serde_json::to_value(self.latitude)?),
            RequestParameter::new("longitude", serde_json::to_value(self.longitude)?),
            RequestParameter::new("title", serde_json::Value::String(self.title)),
            RequestParameter::new("address", serde_json::Value::String(self.address)),
        ];
        push_opt_str(&mut params, "foursquare_id", &self.foursquare_id);
        push_opt_str(&mut params, "foursquare_type", &self.foursquare_type);
        push_opt_str(&mut params, "google_place_id", &self.google_place_id);
        push_opt_str(&mut params, "google_place_type", &self.google_place_type);
        push_opt(&mut params, "disable_notification", &self.disable_notification)?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &self.allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &self.direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &self.suggested_post_parameters)?;
        self.bot.do_api_request("sendVenue", params).await
    }
}

impl_into_future!(SendVenueBuilder, message::Message);

// =========================================================================
// SendContactBuilder
// =========================================================================

pub struct SendContactBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    vcard: Option<String>,
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

impl<'a> SendContactBuilder<'a> {
    pub fn last_name(mut self, val: impl Into<String>) -> Self { self.last_name = Some(val.into()); self }
    pub fn vcard(mut self, val: impl Into<String>) -> Self { self.vcard = Some(val.into()); self }
    pub fn disable_notification(mut self, val: bool) -> Self { self.disable_notification = Some(val); self }
    pub fn protect_content(mut self, val: bool) -> Self { self.protect_content = Some(val); self }
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self { self.reply_parameters = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn message_thread_id(mut self, val: i64) -> Self { self.message_thread_id = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self { self.message_effect_id = Some(val.into()); self }
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self { self.allow_paid_broadcast = Some(val); self }
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self { self.direct_messages_topic_id = Some(val); self }
    pub fn suggested_post_parameters(mut self, val: suggested_post::SuggestedPostParameters) -> Self { self.suggested_post_parameters = Some(val); self }

    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            RequestParameter::new("phone_number", serde_json::Value::String(self.phone_number)),
            RequestParameter::new("first_name", serde_json::Value::String(self.first_name)),
        ];
        push_opt_str(&mut params, "last_name", &self.last_name);
        push_opt_str(&mut params, "vcard", &self.vcard);
        push_opt(&mut params, "disable_notification", &self.disable_notification)?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &self.allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &self.direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &self.suggested_post_parameters)?;
        self.bot.do_api_request("sendContact", params).await
    }
}

impl_into_future!(SendContactBuilder, message::Message);

// =========================================================================
// SendPollBuilder
// =========================================================================

pub struct SendPollBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    question: String,
    options: Vec<serde_json::Value>,
    is_anonymous: Option<bool>,
    poll_type: Option<String>,
    allows_multiple_answers: Option<bool>,
    correct_option_id: Option<i64>,
    explanation: Option<String>,
    explanation_parse_mode: Option<String>,
    explanation_entities: Option<Vec<message_entity::MessageEntity>>,
    open_period: Option<i64>,
    close_date: Option<i64>,
    is_closed: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<reply::ReplyParameters>,
    reply_markup: Option<serde_json::Value>,
    message_thread_id: Option<i64>,
    business_connection_id: Option<String>,
    question_parse_mode: Option<String>,
    question_entities: Option<Vec<message_entity::MessageEntity>>,
    message_effect_id: Option<String>,
    allow_paid_broadcast: Option<bool>,
    direct_messages_topic_id: Option<i64>,
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendPollBuilder<'a> {
    pub fn is_anonymous(mut self, val: bool) -> Self { self.is_anonymous = Some(val); self }
    pub fn poll_type(mut self, val: impl Into<String>) -> Self { self.poll_type = Some(val.into()); self }
    pub fn allows_multiple_answers(mut self, val: bool) -> Self { self.allows_multiple_answers = Some(val); self }
    pub fn correct_option_id(mut self, val: i64) -> Self { self.correct_option_id = Some(val); self }
    pub fn explanation(mut self, val: impl Into<String>) -> Self { self.explanation = Some(val.into()); self }
    pub fn explanation_parse_mode(mut self, val: impl Into<String>) -> Self { self.explanation_parse_mode = Some(val.into()); self }
    pub fn explanation_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self { self.explanation_entities = Some(val); self }
    pub fn open_period(mut self, val: i64) -> Self { self.open_period = Some(val); self }
    pub fn close_date(mut self, val: i64) -> Self { self.close_date = Some(val); self }
    pub fn is_closed(mut self, val: bool) -> Self { self.is_closed = Some(val); self }
    pub fn disable_notification(mut self, val: bool) -> Self { self.disable_notification = Some(val); self }
    pub fn protect_content(mut self, val: bool) -> Self { self.protect_content = Some(val); self }
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self { self.reply_parameters = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn message_thread_id(mut self, val: i64) -> Self { self.message_thread_id = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }
    pub fn question_parse_mode(mut self, val: impl Into<String>) -> Self { self.question_parse_mode = Some(val.into()); self }
    pub fn question_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self { self.question_entities = Some(val); self }
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self { self.message_effect_id = Some(val.into()); self }
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self { self.allow_paid_broadcast = Some(val); self }
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self { self.direct_messages_topic_id = Some(val); self }
    pub fn suggested_post_parameters(mut self, val: suggested_post::SuggestedPostParameters) -> Self { self.suggested_post_parameters = Some(val); self }

    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            RequestParameter::new("question", serde_json::Value::String(self.question)),
            RequestParameter::new("options", serde_json::to_value(&self.options)?),
        ];
        push_opt(&mut params, "is_anonymous", &self.is_anonymous)?;
        // `poll_type` maps to "type" in the API
        push_opt_str(&mut params, "type", &self.poll_type);
        push_opt(&mut params, "allows_multiple_answers", &self.allows_multiple_answers)?;
        push_opt(&mut params, "correct_option_id", &self.correct_option_id)?;
        push_opt_str(&mut params, "explanation", &self.explanation);
        push_opt_str(&mut params, "explanation_parse_mode", &self.explanation_parse_mode);
        push_opt(&mut params, "explanation_entities", &self.explanation_entities)?;
        push_opt(&mut params, "open_period", &self.open_period)?;
        push_opt(&mut params, "close_date", &self.close_date)?;
        push_opt(&mut params, "is_closed", &self.is_closed)?;
        push_opt(&mut params, "disable_notification", &self.disable_notification)?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        push_opt_str(&mut params, "question_parse_mode", &self.question_parse_mode);
        push_opt(&mut params, "question_entities", &self.question_entities)?;
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &self.allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &self.direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &self.suggested_post_parameters)?;
        self.bot.do_api_request("sendPoll", params).await
    }
}

impl_into_future!(SendPollBuilder, message::Message);

// =========================================================================
// SendDiceBuilder
// =========================================================================

pub struct SendDiceBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
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

impl<'a> SendDiceBuilder<'a> {
    pub fn emoji(mut self, val: impl Into<String>) -> Self { self.emoji = Some(val.into()); self }
    pub fn disable_notification(mut self, val: bool) -> Self { self.disable_notification = Some(val); self }
    pub fn protect_content(mut self, val: bool) -> Self { self.protect_content = Some(val); self }
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self { self.reply_parameters = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn message_thread_id(mut self, val: i64) -> Self { self.message_thread_id = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self { self.message_effect_id = Some(val.into()); self }
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self { self.allow_paid_broadcast = Some(val); self }
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self { self.direct_messages_topic_id = Some(val); self }
    pub fn suggested_post_parameters(mut self, val: suggested_post::SuggestedPostParameters) -> Self { self.suggested_post_parameters = Some(val); self }

    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
        ];
        push_opt_str(&mut params, "emoji", &self.emoji);
        push_opt(&mut params, "disable_notification", &self.disable_notification)?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &self.allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &self.direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &self.suggested_post_parameters)?;
        self.bot.do_api_request("sendDice", params).await
    }
}

impl_into_future!(SendDiceBuilder, message::Message);

// =========================================================================
// SendStickerBuilder
// =========================================================================

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
    pub fn emoji(mut self, val: impl Into<String>) -> Self { self.emoji = Some(val.into()); self }
    pub fn disable_notification(mut self, val: bool) -> Self { self.disable_notification = Some(val); self }
    pub fn protect_content(mut self, val: bool) -> Self { self.protect_content = Some(val); self }
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self { self.reply_parameters = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn message_thread_id(mut self, val: i64) -> Self { self.message_thread_id = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self { self.message_effect_id = Some(val.into()); self }
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self { self.allow_paid_broadcast = Some(val); self }
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self { self.direct_messages_topic_id = Some(val); self }
    pub fn suggested_post_parameters(mut self, val: suggested_post::SuggestedPostParameters) -> Self { self.suggested_post_parameters = Some(val); self }

    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            input_file_param("sticker", self.sticker),
        ];
        push_opt_str(&mut params, "emoji", &self.emoji);
        push_opt(&mut params, "disable_notification", &self.disable_notification)?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &self.allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &self.direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &self.suggested_post_parameters)?;
        self.bot.do_api_request("sendSticker", params).await
    }
}

impl_into_future!(SendStickerBuilder, message::Message);

// =========================================================================
// EditMessageTextBuilder
// =========================================================================

pub struct EditMessageTextBuilder<'a> {
    bot: &'a Bot,
    text: String,
    chat_id: Option<ChatId>,
    message_id: Option<i64>,
    inline_message_id: Option<String>,
    parse_mode: Option<String>,
    entities: Option<Vec<message_entity::MessageEntity>>,
    link_preview_options: Option<link_preview_options::LinkPreviewOptions>,
    reply_markup: Option<serde_json::Value>,
    business_connection_id: Option<String>,
}

impl<'a> EditMessageTextBuilder<'a> {
    pub fn chat_id(mut self, val: impl Into<ChatId>) -> Self { self.chat_id = Some(val.into()); self }
    pub fn message_id(mut self, val: i64) -> Self { self.message_id = Some(val); self }
    pub fn inline_message_id(mut self, val: impl Into<String>) -> Self { self.inline_message_id = Some(val.into()); self }
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self { self.parse_mode = Some(val.into()); self }
    pub fn entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self { self.entities = Some(val); self }
    pub fn link_preview_options(mut self, val: link_preview_options::LinkPreviewOptions) -> Self { self.link_preview_options = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }

    pub async fn send(self) -> Result<MessageOrBool> {
        let mut params = vec![
            RequestParameter::new("text", serde_json::Value::String(self.text)),
        ];
        push_opt(&mut params, "chat_id", &self.chat_id)?;
        push_opt(&mut params, "message_id", &self.message_id)?;
        push_opt_str(&mut params, "inline_message_id", &self.inline_message_id);
        push_opt_str(&mut params, "parse_mode", &self.parse_mode);
        push_opt(&mut params, "entities", &self.entities)?;
        push_opt(&mut params, "link_preview_options", &self.link_preview_options)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        self.bot.do_api_request("editMessageText", params).await
    }
}

impl_into_future!(EditMessageTextBuilder, MessageOrBool);

// =========================================================================
// EditMessageCaptionBuilder
// =========================================================================

pub struct EditMessageCaptionBuilder<'a> {
    bot: &'a Bot,
    chat_id: Option<ChatId>,
    message_id: Option<i64>,
    inline_message_id: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<message_entity::MessageEntity>>,
    reply_markup: Option<serde_json::Value>,
    show_caption_above_media: Option<bool>,
    business_connection_id: Option<String>,
}

impl<'a> EditMessageCaptionBuilder<'a> {
    pub fn chat_id(mut self, val: impl Into<ChatId>) -> Self { self.chat_id = Some(val.into()); self }
    pub fn message_id(mut self, val: i64) -> Self { self.message_id = Some(val); self }
    pub fn inline_message_id(mut self, val: impl Into<String>) -> Self { self.inline_message_id = Some(val.into()); self }
    pub fn caption(mut self, val: impl Into<String>) -> Self { self.caption = Some(val.into()); self }
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self { self.parse_mode = Some(val.into()); self }
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self { self.caption_entities = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn show_caption_above_media(mut self, val: bool) -> Self { self.show_caption_above_media = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }

    pub async fn send(self) -> Result<MessageOrBool> {
        let mut params = Vec::new();
        push_opt(&mut params, "chat_id", &self.chat_id)?;
        push_opt(&mut params, "message_id", &self.message_id)?;
        push_opt_str(&mut params, "inline_message_id", &self.inline_message_id);
        push_opt_str(&mut params, "caption", &self.caption);
        push_opt_str(&mut params, "parse_mode", &self.parse_mode);
        push_opt(&mut params, "caption_entities", &self.caption_entities)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "show_caption_above_media", &self.show_caption_above_media)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        self.bot.do_api_request("editMessageCaption", params).await
    }
}

impl_into_future!(EditMessageCaptionBuilder, MessageOrBool);

// =========================================================================
// EditMessageMediaBuilder
// =========================================================================

pub struct EditMessageMediaBuilder<'a> {
    bot: &'a Bot,
    media: serde_json::Value,
    chat_id: Option<ChatId>,
    message_id: Option<i64>,
    inline_message_id: Option<String>,
    reply_markup: Option<serde_json::Value>,
    business_connection_id: Option<String>,
}

impl<'a> EditMessageMediaBuilder<'a> {
    pub fn chat_id(mut self, val: impl Into<ChatId>) -> Self { self.chat_id = Some(val.into()); self }
    pub fn message_id(mut self, val: i64) -> Self { self.message_id = Some(val); self }
    pub fn inline_message_id(mut self, val: impl Into<String>) -> Self { self.inline_message_id = Some(val.into()); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }

    pub async fn send(self) -> Result<MessageOrBool> {
        let mut params = vec![RequestParameter::new("media", self.media)];
        push_opt(&mut params, "chat_id", &self.chat_id)?;
        push_opt(&mut params, "message_id", &self.message_id)?;
        push_opt_str(&mut params, "inline_message_id", &self.inline_message_id);
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        self.bot.do_api_request("editMessageMedia", params).await
    }
}

impl_into_future!(EditMessageMediaBuilder, MessageOrBool);

// =========================================================================
// EditMessageReplyMarkupBuilder
// =========================================================================

pub struct EditMessageReplyMarkupBuilder<'a> {
    bot: &'a Bot,
    chat_id: Option<ChatId>,
    message_id: Option<i64>,
    inline_message_id: Option<String>,
    reply_markup: Option<serde_json::Value>,
    business_connection_id: Option<String>,
}

impl<'a> EditMessageReplyMarkupBuilder<'a> {
    pub fn chat_id(mut self, val: impl Into<ChatId>) -> Self { self.chat_id = Some(val.into()); self }
    pub fn message_id(mut self, val: i64) -> Self { self.message_id = Some(val); self }
    pub fn inline_message_id(mut self, val: impl Into<String>) -> Self { self.inline_message_id = Some(val.into()); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self { self.business_connection_id = Some(val.into()); self }

    pub async fn send(self) -> Result<MessageOrBool> {
        let mut params = Vec::new();
        push_opt(&mut params, "chat_id", &self.chat_id)?;
        push_opt(&mut params, "message_id", &self.message_id)?;
        push_opt_str(&mut params, "inline_message_id", &self.inline_message_id);
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt_str(&mut params, "business_connection_id", &self.business_connection_id);
        self.bot.do_api_request("editMessageReplyMarkup", params).await
    }
}

impl_into_future!(EditMessageReplyMarkupBuilder, MessageOrBool);

// =========================================================================
// AnswerCallbackQueryBuilder
// =========================================================================

pub struct AnswerCallbackQueryBuilder<'a> {
    bot: &'a Bot,
    callback_query_id: String,
    text: Option<String>,
    show_alert: Option<bool>,
    url: Option<String>,
    cache_time: Option<i64>,
}

impl<'a> AnswerCallbackQueryBuilder<'a> {
    pub fn text(mut self, val: impl Into<String>) -> Self { self.text = Some(val.into()); self }
    pub fn show_alert(mut self, val: bool) -> Self { self.show_alert = Some(val); self }
    pub fn url(mut self, val: impl Into<String>) -> Self { self.url = Some(val.into()); self }
    pub fn cache_time(mut self, val: i64) -> Self { self.cache_time = Some(val); self }

    pub async fn send(self) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("callback_query_id", serde_json::Value::String(self.callback_query_id)),
        ];
        push_opt_str(&mut params, "text", &self.text);
        push_opt(&mut params, "show_alert", &self.show_alert)?;
        push_opt_str(&mut params, "url", &self.url);
        push_opt(&mut params, "cache_time", &self.cache_time)?;
        self.bot.do_api_request("answerCallbackQuery", params).await
    }
}

impl_into_future!(AnswerCallbackQueryBuilder, bool);

// =========================================================================
// AnswerInlineQueryBuilder
// =========================================================================

pub struct AnswerInlineQueryBuilder<'a> {
    bot: &'a Bot,
    inline_query_id: String,
    results: Vec<serde_json::Value>,
    cache_time: Option<i64>,
    is_personal: Option<bool>,
    next_offset: Option<String>,
    button: Option<serde_json::Value>,
}

impl<'a> AnswerInlineQueryBuilder<'a> {
    pub fn cache_time(mut self, val: i64) -> Self { self.cache_time = Some(val); self }
    pub fn is_personal(mut self, val: bool) -> Self { self.is_personal = Some(val); self }
    pub fn next_offset(mut self, val: impl Into<String>) -> Self { self.next_offset = Some(val.into()); self }
    pub fn button(mut self, val: serde_json::Value) -> Self { self.button = Some(val); self }

    pub async fn send(self) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("inline_query_id", serde_json::Value::String(self.inline_query_id)),
            RequestParameter::new("results", serde_json::to_value(&self.results)?),
        ];
        push_opt(&mut params, "cache_time", &self.cache_time)?;
        push_opt(&mut params, "is_personal", &self.is_personal)?;
        push_opt_str(&mut params, "next_offset", &self.next_offset);
        push_opt(&mut params, "button", &self.button)?;
        self.bot.do_api_request("answerInlineQuery", params).await
    }
}

impl_into_future!(AnswerInlineQueryBuilder, bool);

// =========================================================================
// SetWebhookBuilder
// =========================================================================

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
    pub fn certificate(mut self, val: files::input_file::InputFile) -> Self { self.certificate = Some(val); self }
    pub fn ip_address(mut self, val: impl Into<String>) -> Self { self.ip_address = Some(val.into()); self }
    pub fn max_connections(mut self, val: i32) -> Self { self.max_connections = Some(val); self }
    pub fn allowed_updates(mut self, val: Vec<String>) -> Self { self.allowed_updates = Some(val); self }
    pub fn drop_pending_updates(mut self, val: bool) -> Self { self.drop_pending_updates = Some(val); self }
    pub fn secret_token(mut self, val: impl Into<String>) -> Self { self.secret_token = Some(val.into()); self }

    pub async fn send(self) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("url", serde_json::Value::String(self.url)),
        ];
        push_opt_file(&mut params, "certificate", self.certificate);
        push_opt_str(&mut params, "ip_address", &self.ip_address);
        push_opt(&mut params, "max_connections", &self.max_connections)?;
        push_opt(&mut params, "allowed_updates", &self.allowed_updates)?;
        push_opt(&mut params, "drop_pending_updates", &self.drop_pending_updates)?;
        push_opt_str(&mut params, "secret_token", &self.secret_token);
        self.bot.do_api_request("setWebhook", params).await
    }
}

impl_into_future!(SetWebhookBuilder, bool);

// =========================================================================
// DeleteWebhookBuilder
// =========================================================================

pub struct DeleteWebhookBuilder<'a> {
    bot: &'a Bot,
    drop_pending_updates: Option<bool>,
}

impl<'a> DeleteWebhookBuilder<'a> {
    pub fn drop_pending_updates(mut self, val: bool) -> Self { self.drop_pending_updates = Some(val); self }

    pub async fn send(self) -> Result<bool> {
        let mut params = Vec::new();
        push_opt(&mut params, "drop_pending_updates", &self.drop_pending_updates)?;
        self.bot.do_api_request("deleteWebhook", params).await
    }
}

impl_into_future!(DeleteWebhookBuilder, bool);

// =========================================================================
// GetFileBuilder
// =========================================================================

pub struct GetFileBuilder<'a> {
    bot: &'a Bot,
    file_id: String,
}

impl<'a> GetFileBuilder<'a> {
    pub async fn send(self) -> Result<files::file::File> {
        let params = vec![
            RequestParameter::new("file_id", serde_json::Value::String(self.file_id)),
        ];
        self.bot.do_api_request("getFile", params).await
    }
}

impl_into_future!(GetFileBuilder, files::file::File);

// =========================================================================
// SendInvoiceBuilder
// =========================================================================

pub struct SendInvoiceBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    title: String,
    description: String,
    payload: String,
    currency: String,
    prices: Vec<serde_json::Value>,
    provider_token: Option<String>,
    max_tip_amount: Option<i64>,
    suggested_tip_amounts: Option<Vec<i64>>,
    start_parameter: Option<String>,
    provider_data: Option<String>,
    photo_url: Option<String>,
    photo_size: Option<i64>,
    photo_width: Option<i64>,
    photo_height: Option<i64>,
    need_name: Option<bool>,
    need_phone_number: Option<bool>,
    need_email: Option<bool>,
    need_shipping_address: Option<bool>,
    send_phone_number_to_provider: Option<bool>,
    send_email_to_provider: Option<bool>,
    is_flexible: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<reply::ReplyParameters>,
    reply_markup: Option<serde_json::Value>,
    message_thread_id: Option<i64>,
    message_effect_id: Option<String>,
    allow_paid_broadcast: Option<bool>,
    direct_messages_topic_id: Option<i64>,
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendInvoiceBuilder<'a> {
    pub fn provider_token(mut self, val: impl Into<String>) -> Self { self.provider_token = Some(val.into()); self }
    pub fn max_tip_amount(mut self, val: i64) -> Self { self.max_tip_amount = Some(val); self }
    pub fn suggested_tip_amounts(mut self, val: Vec<i64>) -> Self { self.suggested_tip_amounts = Some(val); self }
    pub fn start_parameter(mut self, val: impl Into<String>) -> Self { self.start_parameter = Some(val.into()); self }
    pub fn provider_data(mut self, val: impl Into<String>) -> Self { self.provider_data = Some(val.into()); self }
    pub fn photo_url(mut self, val: impl Into<String>) -> Self { self.photo_url = Some(val.into()); self }
    pub fn photo_size(mut self, val: i64) -> Self { self.photo_size = Some(val); self }
    pub fn photo_width(mut self, val: i64) -> Self { self.photo_width = Some(val); self }
    pub fn photo_height(mut self, val: i64) -> Self { self.photo_height = Some(val); self }
    pub fn need_name(mut self, val: bool) -> Self { self.need_name = Some(val); self }
    pub fn need_phone_number(mut self, val: bool) -> Self { self.need_phone_number = Some(val); self }
    pub fn need_email(mut self, val: bool) -> Self { self.need_email = Some(val); self }
    pub fn need_shipping_address(mut self, val: bool) -> Self { self.need_shipping_address = Some(val); self }
    pub fn send_phone_number_to_provider(mut self, val: bool) -> Self { self.send_phone_number_to_provider = Some(val); self }
    pub fn send_email_to_provider(mut self, val: bool) -> Self { self.send_email_to_provider = Some(val); self }
    pub fn is_flexible(mut self, val: bool) -> Self { self.is_flexible = Some(val); self }
    pub fn disable_notification(mut self, val: bool) -> Self { self.disable_notification = Some(val); self }
    pub fn protect_content(mut self, val: bool) -> Self { self.protect_content = Some(val); self }
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self { self.reply_parameters = Some(val); self }
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self { self.reply_markup = Some(val); self }
    pub fn message_thread_id(mut self, val: i64) -> Self { self.message_thread_id = Some(val); self }
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self { self.message_effect_id = Some(val.into()); self }
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self { self.allow_paid_broadcast = Some(val); self }
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self { self.direct_messages_topic_id = Some(val); self }
    pub fn suggested_post_parameters(mut self, val: suggested_post::SuggestedPostParameters) -> Self { self.suggested_post_parameters = Some(val); self }

    pub async fn send(self) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&self.chat_id)?),
            RequestParameter::new("title", serde_json::Value::String(self.title)),
            RequestParameter::new("description", serde_json::Value::String(self.description)),
            RequestParameter::new("payload", serde_json::Value::String(self.payload)),
            RequestParameter::new("currency", serde_json::Value::String(self.currency)),
            RequestParameter::new("prices", serde_json::to_value(&self.prices)?),
        ];
        push_opt_str(&mut params, "provider_token", &self.provider_token);
        push_opt(&mut params, "max_tip_amount", &self.max_tip_amount)?;
        push_opt(&mut params, "suggested_tip_amounts", &self.suggested_tip_amounts)?;
        push_opt_str(&mut params, "start_parameter", &self.start_parameter);
        push_opt_str(&mut params, "provider_data", &self.provider_data);
        push_opt_str(&mut params, "photo_url", &self.photo_url);
        push_opt(&mut params, "photo_size", &self.photo_size)?;
        push_opt(&mut params, "photo_width", &self.photo_width)?;
        push_opt(&mut params, "photo_height", &self.photo_height)?;
        push_opt(&mut params, "need_name", &self.need_name)?;
        push_opt(&mut params, "need_phone_number", &self.need_phone_number)?;
        push_opt(&mut params, "need_email", &self.need_email)?;
        push_opt(&mut params, "need_shipping_address", &self.need_shipping_address)?;
        push_opt(&mut params, "send_phone_number_to_provider", &self.send_phone_number_to_provider)?;
        push_opt(&mut params, "send_email_to_provider", &self.send_email_to_provider)?;
        push_opt(&mut params, "is_flexible", &self.is_flexible)?;
        push_opt(&mut params, "disable_notification", &self.disable_notification)?;
        push_opt(&mut params, "protect_content", &self.protect_content)?;
        push_opt(&mut params, "reply_parameters", &self.reply_parameters)?;
        push_opt(&mut params, "reply_markup", &self.reply_markup)?;
        push_opt(&mut params, "message_thread_id", &self.message_thread_id)?;
        push_opt_str(&mut params, "message_effect_id", &self.message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &self.allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &self.direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &self.suggested_post_parameters)?;
        self.bot.do_api_request("sendInvoice", params).await
    }
}

impl_into_future!(SendInvoiceBuilder, message::Message);

// =========================================================================
// AnswerShippingQueryBuilder
// =========================================================================

pub struct AnswerShippingQueryBuilder<'a> {
    bot: &'a Bot,
    shipping_query_id: String,
    ok: bool,
    shipping_options: Option<Vec<serde_json::Value>>,
    error_message: Option<String>,
}

impl<'a> AnswerShippingQueryBuilder<'a> {
    pub fn shipping_options(mut self, val: Vec<serde_json::Value>) -> Self { self.shipping_options = Some(val); self }
    pub fn error_message(mut self, val: impl Into<String>) -> Self { self.error_message = Some(val.into()); self }

    pub async fn send(self) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("shipping_query_id", serde_json::Value::String(self.shipping_query_id)),
            RequestParameter::new("ok", serde_json::to_value(self.ok)?),
        ];
        push_opt(&mut params, "shipping_options", &self.shipping_options)?;
        push_opt_str(&mut params, "error_message", &self.error_message);
        self.bot.do_api_request("answerShippingQuery", params).await
    }
}

impl_into_future!(AnswerShippingQueryBuilder, bool);

// =========================================================================
// AnswerPreCheckoutQueryBuilder
// =========================================================================

pub struct AnswerPreCheckoutQueryBuilder<'a> {
    bot: &'a Bot,
    pre_checkout_query_id: String,
    ok: bool,
    error_message: Option<String>,
}

impl<'a> AnswerPreCheckoutQueryBuilder<'a> {
    pub fn error_message(mut self, val: impl Into<String>) -> Self { self.error_message = Some(val.into()); self }

    pub async fn send(self) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("pre_checkout_query_id", serde_json::Value::String(self.pre_checkout_query_id)),
            RequestParameter::new("ok", serde_json::to_value(self.ok)?),
        ];
        push_opt_str(&mut params, "error_message", &self.error_message);
        self.bot.do_api_request("answerPreCheckoutQuery", params).await
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
    pub fn edit_message_text(
        &self,
        text: impl Into<String>,
    ) -> EditMessageTextBuilder<'_> {
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
    pub fn edit_message_media(
        &self,
        media: serde_json::Value,
    ) -> EditMessageMediaBuilder<'_> {
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
    pub fn set_webhook(
        &self,
        url: impl Into<String>,
    ) -> SetWebhookBuilder<'_> {
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
    pub fn get_file(
        &self,
        file_id: impl Into<String>,
    ) -> GetFileBuilder<'_> {
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

pub struct GetManagedBotTokenBuilder<'a> {
    bot: &'a Bot,
    bot_user_id: i64,
}

impl<'a> GetManagedBotTokenBuilder<'a> {
    pub async fn send(self) -> Result<String> {
        let params = vec![
            RequestParameter::new("bot_user_id", serde_json::to_value(self.bot_user_id)?),
        ];
        self.bot.do_api_request("getManagedBotToken", params).await
    }
}

impl_into_future!(GetManagedBotTokenBuilder, String);

// =========================================================================
// ReplaceManagedBotTokenBuilder
// =========================================================================

pub struct ReplaceManagedBotTokenBuilder<'a> {
    bot: &'a Bot,
    bot_user_id: i64,
}

impl<'a> ReplaceManagedBotTokenBuilder<'a> {
    pub async fn send(self) -> Result<String> {
        let params = vec![
            RequestParameter::new("bot_user_id", serde_json::to_value(self.bot_user_id)?),
        ];
        self.bot.do_api_request("replaceManagedBotToken", params).await
    }
}

impl_into_future!(ReplaceManagedBotTokenBuilder, String);

// =========================================================================
// SavePreparedKeyboardButtonBuilder
// =========================================================================

pub struct SavePreparedKeyboardButtonBuilder<'a> {
    bot: &'a Bot,
    user_id: i64,
    button: inline::inline_keyboard_button::InlineKeyboardButton,
    allow_user_chats: Option<bool>,
    allow_bot_chats: Option<bool>,
    allow_group_chats: Option<bool>,
    allow_channel_chats: Option<bool>,
}

impl<'a> SavePreparedKeyboardButtonBuilder<'a> {
    pub fn allow_user_chats(mut self, val: bool) -> Self { self.allow_user_chats = Some(val); self }
    pub fn allow_bot_chats(mut self, val: bool) -> Self { self.allow_bot_chats = Some(val); self }
    pub fn allow_group_chats(mut self, val: bool) -> Self { self.allow_group_chats = Some(val); self }
    pub fn allow_channel_chats(mut self, val: bool) -> Self { self.allow_channel_chats = Some(val); self }

    pub async fn send(self) -> Result<prepared_keyboard_button::PreparedKeyboardButton> {
        let mut params = vec![
            RequestParameter::new("user_id", serde_json::to_value(self.user_id)?),
            RequestParameter::new("button", serde_json::to_value(&self.button)?),
        ];
        push_opt(&mut params, "allow_user_chats", &self.allow_user_chats)?;
        push_opt(&mut params, "allow_bot_chats", &self.allow_bot_chats)?;
        push_opt(&mut params, "allow_group_chats", &self.allow_group_chats)?;
        push_opt(&mut params, "allow_channel_chats", &self.allow_channel_chats)?;
        self.bot.do_api_request("savePreparedKeyboardButton", params).await
    }
}

impl_into_future!(SavePreparedKeyboardButtonBuilder, prepared_keyboard_button::PreparedKeyboardButton);

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
}
