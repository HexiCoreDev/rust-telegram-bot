#![allow(clippy::too_many_arguments)]

use crate::error::Result;
use crate::request::base::{BaseRequest, TimeoutOverride};
use crate::request::request_data::RequestData;
use crate::request::request_parameter::{InputFileRef, RequestParameter};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

// ---------------------------------------------------------------------------
// Shared enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatId {
    Id(i64),
    Username(String),
}

impl std::fmt::Display for ChatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatId::Id(id) => write!(f, "{id}"),
            ChatId::Username(u) => write!(f, "{u}"),
        }
    }
}

impl From<i64> for ChatId {
    fn from(id: i64) -> Self {
        ChatId::Id(id)
    }
}

impl From<String> for ChatId {
    fn from(username: String) -> Self {
        ChatId::Username(username)
    }
}

impl From<&str> for ChatId {
    fn from(username: &str) -> Self {
        ChatId::Username(username.to_owned())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageOrBool {
    Message(Box<message::Message>),
    Bool(bool),
}

// ---------------------------------------------------------------------------
// Defaults -- user-configurable default parameters (C10)
// ---------------------------------------------------------------------------

/// Default parameter values merged into every API call when the
/// caller has not provided an explicit value.
#[derive(Debug, Clone, Default)]
pub struct Defaults {
    pub parse_mode: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_sending_without_reply: Option<bool>,
    pub link_preview_options: Option<link_preview_options::LinkPreviewOptions>,
    pub quote: Option<bool>,
}


// ---------------------------------------------------------------------------
// Bot struct
// ---------------------------------------------------------------------------
pub struct Bot {
    token: String,
    base_url: String,
    base_file_url: String,
    request: Arc<dyn BaseRequest>,
    /// Separate request object for `getUpdates` long-polling (M3).
    request_for_updates: Arc<dyn BaseRequest>,
    /// User-configured defaults merged into outgoing API calls (C10).
    defaults: Option<Defaults>,
    /// Cached result of `get_me()` after `initialize()` (M5).
    cached_bot_data: Arc<RwLock<Option<user::User>>>,
    /// When `true`, [`files::input_file::InputFile::Path`] is sent as a
    /// `file://` URI instead of uploading the file bytes.  Required when
    /// connecting to a locally-hosted Bot API server.
    local_mode: bool,
}

impl std::fmt::Debug for Bot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bot")
            .field("token", &"[REDACTED]")
            .field("base_url", &self.base_url)
            .field("base_file_url", &self.base_file_url)
            .field("defaults", &self.defaults)
            .field("local_mode", &self.local_mode)
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Convert an `InputFile` into a `RequestParameter`, handling file uploads.
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
/// Push an optional value as a `RequestParameter` if present.
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

/// Push an optional `&str` parameter.
fn push_opt_str(params: &mut Vec<RequestParameter>, name: &'static str, val: Option<&str>) {
    if let Some(v) = val {
        params.push(RequestParameter::new(
            name,
            serde_json::Value::String(v.to_owned()),
        ));
    }
}

/// Push an optional `InputFile` parameter.
fn push_opt_file(
    params: &mut Vec<RequestParameter>,
    name: &'static str,
    val: Option<files::input_file::InputFile>,
) {
    if let Some(f) = val {
        params.push(input_file_param(name, f));
    }
}

impl Bot {
    pub fn new(token: impl Into<String>, request: Arc<dyn BaseRequest>) -> Self {
        let token = token.into();
        let base_url = format!("https://api.telegram.org/bot{token}");
        let base_file_url = format!("https://api.telegram.org/file/bot{token}");
        Self { token, base_url, base_file_url, request_for_updates: Arc::clone(&request), request, defaults: None, cached_bot_data: Arc::new(RwLock::new(None)), local_mode: false }
    }

    pub fn with_options(token: impl Into<String>, request: Arc<dyn BaseRequest>, request_for_updates: Option<Arc<dyn BaseRequest>>, defaults: Option<Defaults>) -> Self {
        let token = token.into();
        let base_url = format!("https://api.telegram.org/bot{token}");
        let base_file_url = format!("https://api.telegram.org/file/bot{token}");
        let request_for_updates = request_for_updates.unwrap_or_else(|| Arc::clone(&request));
        Self { token, base_url, base_file_url, request_for_updates, request, defaults, cached_bot_data: Arc::new(RwLock::new(None)), local_mode: false }
    }

    pub fn token(&self) -> &str { &self.token }
    pub fn base_url(&self) -> &str { &self.base_url }
    pub fn base_file_url(&self) -> &str { &self.base_file_url }
    pub fn defaults(&self) -> Option<&Defaults> { self.defaults.as_ref() }
    pub async fn bot_data(&self) -> Option<user::User> { self.cached_bot_data.read().await.clone() }

    /// Returns `true` if the bot is operating in local mode.
    pub fn local_mode(&self) -> bool {
        self.local_mode
    }

    /// Enable local mode.  When enabled, [`files::input_file::InputFile::Path`]
    /// values are sent as `file://` URIs instead of being uploaded as bytes.
    ///
    /// Use this when connecting to a locally-hosted Bot API server instance.
    #[must_use]
    pub fn with_local_mode(mut self) -> Self {
        self.local_mode = true;
        self
    }

    fn api_url(&self, method: &str) -> String { format!("{}/{method}", self.base_url) }

    async fn resolve_file_paths(&self, params: &mut Vec<RequestParameter>) -> Result<()> {
        for param in params.iter_mut() {
            let path_str = param
                .value
                .as_ref()
                .and_then(|v| v.as_str())
                .and_then(|s| s.strip_prefix("__filepath__:"))
                .map(str::to_owned);
            if let Some(path_str) = path_str {
                if self.local_mode {
                    // In local mode, send the path as a file:// URI rather than uploading bytes.
                    param.value = Some(serde_json::Value::String(format!("file://{path_str}")));
                    param.input_files = None;
                } else {
                    let data = tokio::fs::read(&path_str).await?;
                    param.value = None;
                    if let Some(ref mut files) = param.input_files {
                        for f in files.iter_mut() { if f.bytes.is_empty() { f.bytes = data.clone(); } }
                    }
                }
            }
        }
        Ok(())
    }

    fn apply_defaults(&self, params: &mut Vec<RequestParameter>) {
        let defaults = match &self.defaults { Some(d) => d, None => return };
        let existing: std::collections::HashSet<String> = params.iter().map(|p| p.name.as_ref().to_owned()).collect();
        if let Some(ref pm) = defaults.parse_mode {
            if !existing.contains("parse_mode") { params.push(RequestParameter::new("parse_mode", serde_json::Value::String(pm.clone()))); }
        }
        if let Some(dn) = defaults.disable_notification {
            if !existing.contains("disable_notification") { params.push(RequestParameter::new("disable_notification", serde_json::Value::Bool(dn))); }
        }
        if let Some(pc) = defaults.protect_content {
            if !existing.contains("protect_content") { params.push(RequestParameter::new("protect_content", serde_json::Value::Bool(pc))); }
        }
        if let Some(aswr) = defaults.allow_sending_without_reply {
            if !existing.contains("allow_sending_without_reply") { params.push(RequestParameter::new("allow_sending_without_reply", serde_json::Value::Bool(aswr))); }
        }
        if let Some(ref lpo) = defaults.link_preview_options {
            if !existing.contains("link_preview_options") {
                if let Ok(v) = serde_json::to_value(lpo) { params.push(RequestParameter::new("link_preview_options", v)); }
            }
        }
    }

    async fn do_post<T: serde::de::DeserializeOwned>(&self, method: &str, params: Vec<RequestParameter>) -> Result<T> {
        self.do_post_inner(method, params, TimeoutOverride::default_none(), None).await
    }

    #[allow(dead_code)]
    async fn do_post_with_timeouts<T: serde::de::DeserializeOwned>(&self, method: &str, params: Vec<RequestParameter>, timeouts: TimeoutOverride) -> Result<T> {
        self.do_post_inner(method, params, timeouts, None).await
    }

    /// Central dispatch — heap-allocates the future via Box::pin to prevent
    /// stack overflow from deeply nested async state machines.
    fn do_post_inner<'a, T: serde::de::DeserializeOwned + 'a>(
        &'a self,
        method: &'a str,
        mut params: Vec<RequestParameter>,
        timeouts: TimeoutOverride,
        api_kwargs: Option<HashMap<String, serde_json::Value>>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send + 'a>> {
        Box::pin(async move {
            self.apply_defaults(&mut params);
            if let Some(kwargs) = api_kwargs {
                let existing: std::collections::HashSet<String> = params.iter().map(|p| p.name.as_ref().to_owned()).collect();
                for (key, value) in kwargs { if !existing.contains(key.as_str()) { params.push(RequestParameter::new(key, value)); } }
            }
            self.resolve_file_paths(&mut params).await?;
            let url = self.api_url(method);
            let data = RequestData::from_parameters(params);
            let result = self.request.post(&url, Some(&data), timeouts).await?;
            serde_json::from_value(result).map_err(Into::into)
        })
    }

    pub async fn download_file(&self, file_path: &str) -> Result<Vec<u8>> {
        let url = format!("{}/{file_path}", self.base_file_url);
        let bytes = self.request.retrieve(&url, TimeoutOverride::default_none()).await?;
        Ok(bytes.to_vec())
    }

    pub async fn initialize(&mut self) -> Result<()> {
        self.request.initialize().await?;
        if !Arc::ptr_eq(&self.request, &self.request_for_updates) { self.request_for_updates.initialize().await?; }
        let me = self.get_me().await?;
        *self.cached_bot_data.write().await = Some(me);
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<()> {
        self.request.shutdown().await?;
        if !Arc::ptr_eq(&self.request, &self.request_for_updates) { self.request_for_updates.shutdown().await?; }
        Ok(())
    }

    pub async fn do_api_request<T: serde::de::DeserializeOwned>(&self, method: &str, params: Vec<RequestParameter>) -> Result<T> {
        self.do_post(method, params).await
    }

    pub async fn do_api_request_with_kwargs<T: serde::de::DeserializeOwned>(&self, method: &str, params: Vec<RequestParameter>, api_kwargs: Option<HashMap<String, serde_json::Value>>) -> Result<T> {
        self.do_post_inner(method, params, TimeoutOverride::default_none(), api_kwargs).await
    }

    // ======================================================================
    // Getting updates
    // ======================================================================

    pub async fn get_updates(&self, offset: Option<i64>, limit: Option<i32>, timeout: Option<i32>, allowed_updates: Option<Vec<String>>) -> Result<Vec<update::Update>> {
        let mut params = Vec::new();
        push_opt(&mut params, "offset", &offset)?;
        push_opt(&mut params, "limit", &limit)?;
        push_opt(&mut params, "timeout", &timeout)?;
        push_opt(&mut params, "allowed_updates", &allowed_updates)?;
        self.apply_defaults(&mut params);
        let timeouts = if let Some(t) = timeout {
            let effective = Duration::from_secs(t as u64 + 2);
            TimeoutOverride { read: Some(Some(effective)), ..TimeoutOverride::default_none() }
        } else { TimeoutOverride::default_none() };
        let url = self.api_url("getUpdates");
        let data = RequestData::from_parameters(params);
        let result = self.request_for_updates.post(&url, Some(&data), timeouts).await?;
        serde_json::from_value(result).map_err(Into::into)
    }


    pub(crate) async fn set_webhook_raw(
        &self,
        url: &str,
        certificate: Option<files::input_file::InputFile>,
        ip_address: Option<&str>,
        max_connections: Option<i32>,
        allowed_updates: Option<Vec<String>>,
        drop_pending_updates: Option<bool>,
        secret_token: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "url",
            serde_json::Value::String(url.to_owned()),
        )];
        push_opt_file(&mut params, "certificate", certificate);
        push_opt_str(&mut params, "ip_address", ip_address);
        push_opt(&mut params, "max_connections", &max_connections)?;
        push_opt(&mut params, "allowed_updates", &allowed_updates)?;
        push_opt(&mut params, "drop_pending_updates", &drop_pending_updates)?;
        push_opt_str(&mut params, "secret_token", secret_token);
        self.do_post("setWebhook", params).await
    }

    pub(crate) async fn delete_webhook_raw(
        &self,
        drop_pending_updates: Option<bool>,
    ) -> Result<bool> {
        let mut params = Vec::new();
        push_opt(&mut params, "drop_pending_updates", &drop_pending_updates)?;
        self.do_post("deleteWebhook", params).await
    }

    pub async fn get_webhook_info(&self) -> Result<webhook_info::WebhookInfo> {
        self.do_post("getWebhookInfo", Vec::new()).await
    }

    // ======================================================================
    // Basic methods
    // ======================================================================

    pub async fn get_me(&self) -> Result<user::User> {
        self.do_post("getMe", Vec::new()).await
    }

    pub async fn log_out(&self) -> Result<bool> {
        self.do_post("logOut", Vec::new()).await
    }

    pub async fn close(&self) -> Result<bool> {
        self.do_post("close", Vec::new()).await
    }

    // ======================================================================
    // Sending messages
    // ======================================================================

    pub(crate) async fn send_message_raw(
        &self,
        chat_id: ChatId,
        text: &str,
        parse_mode: Option<&str>,
        entities: Option<Vec<message_entity::MessageEntity>>,
        link_preview_options: Option<link_preview_options::LinkPreviewOptions>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("text", serde_json::Value::String(text.to_owned())),
        ];
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "entities", &entities)?;
        push_opt(&mut params, "link_preview_options", &link_preview_options)?;
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendMessage", params).await
    }

    pub async fn send_message_draft(
        &self,
        chat_id: i64,
        draft_id: i64,
        text: &str,
        message_thread_id: Option<i64>,
        parse_mode: Option<&str>,
        entities: Option<Vec<message_entity::MessageEntity>>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(chat_id)?),
            RequestParameter::new("draft_id", serde_json::to_value(draft_id)?),
            RequestParameter::new("text", serde_json::Value::String(text.to_owned())),
        ];
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "entities", &entities)?;
        self.do_post("sendMessageDraft", params).await
    }

    pub async fn forward_message(
        &self,
        chat_id: ChatId,
        from_chat_id: ChatId,
        message_id: i64,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        message_thread_id: Option<i64>,
        video_start_timestamp: Option<i64>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
        message_effect_id: Option<&str>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("from_chat_id", serde_json::to_value(&from_chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt(&mut params, "video_start_timestamp", &video_start_timestamp)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        self.do_post("forwardMessage", params).await
    }

    pub async fn forward_messages(
        &self,
        chat_id: ChatId,
        from_chat_id: ChatId,
        message_ids: Vec<i64>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        message_thread_id: Option<i64>,
        direct_messages_topic_id: Option<i64>,
    ) -> Result<Vec<message_id::MessageId>> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("from_chat_id", serde_json::to_value(&from_chat_id)?),
            RequestParameter::new("message_ids", serde_json::to_value(&message_ids)?),
        ];
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        self.do_post("forwardMessages", params).await
    }

    pub async fn copy_message(
        &self,
        chat_id: ChatId,
        from_chat_id: ChatId,
        message_id: i64,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        show_caption_above_media: Option<bool>,
        allow_paid_broadcast: Option<bool>,
        video_start_timestamp: Option<i64>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
        message_effect_id: Option<&str>,
    ) -> Result<message_id::MessageId> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("from_chat_id", serde_json::to_value(&from_chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt(&mut params, "show_caption_above_media", &show_caption_above_media)?;
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "video_start_timestamp", &video_start_timestamp)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        self.do_post("copyMessage", params).await
    }

    pub async fn copy_messages(
        &self,
        chat_id: ChatId,
        from_chat_id: ChatId,
        message_ids: Vec<i64>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        message_thread_id: Option<i64>,
        remove_caption: Option<bool>,
        direct_messages_topic_id: Option<i64>,
    ) -> Result<Vec<message_id::MessageId>> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("from_chat_id", serde_json::to_value(&from_chat_id)?),
            RequestParameter::new("message_ids", serde_json::to_value(&message_ids)?),
        ];
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt(&mut params, "remove_caption", &remove_caption)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        self.do_post("copyMessages", params).await
    }

    pub async fn delete_message(
        &self,
        chat_id: ChatId,
        message_id: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        self.do_post("deleteMessage", params).await
    }

    pub async fn delete_messages(
        &self,
        chat_id: ChatId,
        message_ids: Vec<i64>,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("message_ids", serde_json::to_value(&message_ids)?),
        ];
        self.do_post("deleteMessages", params).await
    }

    // ======================================================================
    // Sending media
    // ======================================================================

    pub(crate) async fn send_photo_raw(
        &self,
        chat_id: ChatId,
        photo: files::input_file::InputFile,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        has_spoiler: Option<bool>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        show_caption_above_media: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            input_file_param("photo", photo),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt(&mut params, "has_spoiler", &has_spoiler)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "show_caption_above_media", &show_caption_above_media)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendPhoto", params).await
    }

    pub(crate) async fn send_audio_raw(
        &self,
        chat_id: ChatId,
        audio: files::input_file::InputFile,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        duration: Option<i64>,
        performer: Option<&str>,
        title: Option<&str>,
        thumbnail: Option<files::input_file::InputFile>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            input_file_param("audio", audio),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "duration", &duration)?;
        push_opt_str(&mut params, "performer", performer);
        push_opt_str(&mut params, "title", title);
        push_opt_file(&mut params, "thumbnail", thumbnail);
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendAudio", params).await
    }

    pub(crate) async fn send_document_raw(
        &self,
        chat_id: ChatId,
        document: files::input_file::InputFile,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        disable_content_type_detection: Option<bool>,
        thumbnail: Option<files::input_file::InputFile>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            input_file_param("document", document),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(
            &mut params,
            "disable_content_type_detection",
            &disable_content_type_detection,
        )?;
        push_opt_file(&mut params, "thumbnail", thumbnail);
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendDocument", params).await
    }

    pub(crate) async fn send_video_raw(
        &self,
        chat_id: ChatId,
        video: files::input_file::InputFile,
        duration: Option<i64>,
        width: Option<i64>,
        height: Option<i64>,
        caption: Option<&str>,
        parse_mode: Option<&str>,
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
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            input_file_param("video", video),
        ];
        push_opt(&mut params, "duration", &duration)?;
        push_opt(&mut params, "width", &width)?;
        push_opt(&mut params, "height", &height)?;
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "supports_streaming", &supports_streaming)?;
        push_opt_file(&mut params, "thumbnail", thumbnail);
        push_opt(&mut params, "has_spoiler", &has_spoiler)?;
        push_opt(&mut params, "show_caption_above_media", &show_caption_above_media)?;
        push_opt_file(&mut params, "cover", cover);
        push_opt(&mut params, "start_timestamp", &start_timestamp)?;
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendVideo", params).await
    }

    pub(crate) async fn send_animation_raw(
        &self,
        chat_id: ChatId,
        animation: files::input_file::InputFile,
        duration: Option<i64>,
        width: Option<i64>,
        height: Option<i64>,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        thumbnail: Option<files::input_file::InputFile>,
        has_spoiler: Option<bool>,
        show_caption_above_media: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            input_file_param("animation", animation),
        ];
        push_opt(&mut params, "duration", &duration)?;
        push_opt(&mut params, "width", &width)?;
        push_opt(&mut params, "height", &height)?;
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt_file(&mut params, "thumbnail", thumbnail);
        push_opt(&mut params, "has_spoiler", &has_spoiler)?;
        push_opt(&mut params, "show_caption_above_media", &show_caption_above_media)?;
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendAnimation", params).await
    }

    pub(crate) async fn send_voice_raw(
        &self,
        chat_id: ChatId,
        voice: files::input_file::InputFile,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        duration: Option<i64>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            input_file_param("voice", voice),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "duration", &duration)?;
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendVoice", params).await
    }

    pub(crate) async fn send_video_note_raw(
        &self,
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
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            input_file_param("video_note", video_note),
        ];
        push_opt(&mut params, "duration", &duration)?;
        push_opt(&mut params, "length", &length)?;
        push_opt_file(&mut params, "thumbnail", thumbnail);
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendVideoNote", params).await
    }

    pub async fn send_media_group(&self, chat_id: ChatId, media: Vec<serde_json::Value>, disable_notification: Option<bool>, protect_content: Option<bool>, message_thread_id: Option<i64>, reply_parameters: Option<reply::ReplyParameters>, business_connection_id: Option<&str>, message_effect_id: Option<&str>, allow_paid_broadcast: Option<bool>, direct_messages_topic_id: Option<i64>, suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>) -> Result<Vec<message::Message>> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("media", serde_json::to_value(&media)?),
        ];
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendMediaGroup", params).await
    }


    pub async fn send_paid_media(
        &self,
        chat_id: ChatId,
        star_count: i64,
        media: Vec<serde_json::Value>,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        show_caption_above_media: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        business_connection_id: Option<&str>,
        payload: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
        message_thread_id: Option<i64>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("star_count", serde_json::to_value(star_count)?),
            RequestParameter::new("media", serde_json::to_value(&media)?),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "show_caption_above_media", &show_caption_above_media)?;
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "payload", payload);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        self.do_post("sendPaidMedia", params).await
    }

    // ======================================================================
    // Sending other content
    // ======================================================================

    pub(crate) async fn send_location_raw(
        &self,
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
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("latitude", serde_json::to_value(latitude)?),
            RequestParameter::new("longitude", serde_json::to_value(longitude)?),
        ];
        push_opt(&mut params, "horizontal_accuracy", &horizontal_accuracy)?;
        push_opt(&mut params, "live_period", &live_period)?;
        push_opt(&mut params, "heading", &heading)?;
        push_opt(&mut params, "proximity_alert_radius", &proximity_alert_radius)?;
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendLocation", params).await
    }

    pub(crate) async fn send_venue_raw(
        &self,
        chat_id: ChatId,
        latitude: f64,
        longitude: f64,
        title: &str,
        address: &str,
        foursquare_id: Option<&str>,
        foursquare_type: Option<&str>,
        google_place_id: Option<&str>,
        google_place_type: Option<&str>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("latitude", serde_json::to_value(latitude)?),
            RequestParameter::new("longitude", serde_json::to_value(longitude)?),
            RequestParameter::new("title", serde_json::Value::String(title.to_owned())),
            RequestParameter::new("address", serde_json::Value::String(address.to_owned())),
        ];
        push_opt_str(&mut params, "foursquare_id", foursquare_id);
        push_opt_str(&mut params, "foursquare_type", foursquare_type);
        push_opt_str(&mut params, "google_place_id", google_place_id);
        push_opt_str(&mut params, "google_place_type", google_place_type);
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendVenue", params).await
    }

    pub(crate) async fn send_contact_raw(
        &self,
        chat_id: ChatId,
        phone_number: &str,
        first_name: &str,
        last_name: Option<&str>,
        vcard: Option<&str>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "phone_number",
                serde_json::Value::String(phone_number.to_owned()),
            ),
            RequestParameter::new(
                "first_name",
                serde_json::Value::String(first_name.to_owned()),
            ),
        ];
        push_opt_str(&mut params, "last_name", last_name);
        push_opt_str(&mut params, "vcard", vcard);
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendContact", params).await
    }

    pub(crate) async fn send_poll_raw(&self, chat_id: ChatId, question: &str, options: Vec<serde_json::Value>, is_anonymous: Option<bool>, poll_type: Option<&str>, allows_multiple_answers: Option<bool>, correct_option_id: Option<i64>, explanation: Option<&str>, explanation_parse_mode: Option<&str>, explanation_entities: Option<Vec<message_entity::MessageEntity>>, open_period: Option<i64>, close_date: Option<i64>, is_closed: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, reply_parameters: Option<reply::ReplyParameters>, reply_markup: Option<serde_json::Value>, message_thread_id: Option<i64>, business_connection_id: Option<&str>, question_parse_mode: Option<&str>, question_entities: Option<Vec<message_entity::MessageEntity>>, message_effect_id: Option<&str>, allow_paid_broadcast: Option<bool>, direct_messages_topic_id: Option<i64>, suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("question", serde_json::Value::String(question.to_owned())),
            RequestParameter::new("options", serde_json::to_value(&options)?),
        ];
        push_opt(&mut params, "is_anonymous", &is_anonymous)?;
        push_opt_str(&mut params, "type", poll_type);
        push_opt(&mut params, "allows_multiple_answers", &allows_multiple_answers)?;
        push_opt(&mut params, "correct_option_id", &correct_option_id)?;
        push_opt_str(&mut params, "explanation", explanation);
        push_opt_str(&mut params, "explanation_parse_mode", explanation_parse_mode);
        push_opt(&mut params, "explanation_entities", &explanation_entities)?;
        push_opt(&mut params, "open_period", &open_period)?;
        push_opt(&mut params, "close_date", &close_date)?;
        push_opt(&mut params, "is_closed", &is_closed)?;
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "question_parse_mode", question_parse_mode);
        push_opt(&mut params, "question_entities", &question_entities)?;
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendPoll", params).await
    }


    pub(crate) async fn send_dice_raw(
        &self,
        chat_id: ChatId,
        emoji: Option<&str>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        push_opt_str(&mut params, "emoji", emoji);
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendDice", params).await
    }

    pub async fn send_chat_action(
        &self,
        chat_id: ChatId,
        action: &str,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("action", serde_json::Value::String(action.to_owned())),
        ];
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        self.do_post("sendChatAction", params).await
    }

    pub async fn send_checklist(
        &self,
        business_connection_id: &str,
        chat_id: i64,
        checklist: input_checklist::InputChecklist,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        message_effect_id: Option<&str>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("chat_id", serde_json::to_value(chat_id)?),
            RequestParameter::new("checklist", serde_json::to_value(&checklist)?),
        ];
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        self.do_post("sendChecklist", params).await
    }

    // ======================================================================
    // Stickers
    // ======================================================================

    pub(crate) async fn send_sticker_raw(
        &self,
        chat_id: ChatId,
        sticker: files::input_file::InputFile,
        emoji: Option<&str>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            input_file_param("sticker", sticker),
        ];
        push_opt_str(&mut params, "emoji", emoji);
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendSticker", params).await
    }

    pub async fn get_sticker_set(&self, name: &str) -> Result<files::sticker::StickerSet> {
        let params = vec![RequestParameter::new(
            "name",
            serde_json::Value::String(name.to_owned()),
        )];
        self.do_post("getStickerSet", params).await
    }

    pub async fn get_custom_emoji_stickers(
        &self,
        custom_emoji_ids: Vec<String>,
    ) -> Result<Vec<files::sticker::Sticker>> {
        let params = vec![RequestParameter::new(
            "custom_emoji_ids",
            serde_json::to_value(&custom_emoji_ids)?,
        )];
        self.do_post("getCustomEmojiStickers", params).await
    }

    pub async fn upload_sticker_file(
        &self,
        user_id: i64,
        sticker: files::input_file::InputFile,
        sticker_format: &str,
    ) -> Result<files::file::File> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            input_file_param("sticker", sticker),
            RequestParameter::new(
                "sticker_format",
                serde_json::Value::String(sticker_format.to_owned()),
            ),
        ];
        self.do_post("uploadStickerFile", params).await
    }

    pub async fn create_new_sticker_set(
        &self,
        user_id: i64,
        name: &str,
        title: &str,
        stickers: Vec<serde_json::Value>,
        sticker_type: Option<&str>,
        needs_repainting: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
            RequestParameter::new("title", serde_json::Value::String(title.to_owned())),
            RequestParameter::new("stickers", serde_json::to_value(&stickers)?),
        ];
        push_opt_str(&mut params, "sticker_type", sticker_type);
        push_opt(&mut params, "needs_repainting", &needs_repainting)?;
        self.do_post("createNewStickerSet", params).await
    }

    pub async fn add_sticker_to_set(
        &self,
        user_id: i64,
        name: &str,
        sticker: serde_json::Value,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
            RequestParameter::new("sticker", sticker),
        ];
        self.do_post("addStickerToSet", params).await
    }

    pub async fn set_sticker_position_in_set(
        &self,
        sticker: &str,
        position: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "sticker",
                serde_json::Value::String(sticker.to_owned()),
            ),
            RequestParameter::new("position", serde_json::to_value(position)?),
        ];
        self.do_post("setStickerPositionInSet", params).await
    }

    pub async fn delete_sticker_from_set(&self, sticker: &str) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "sticker",
            serde_json::Value::String(sticker.to_owned()),
        )];
        self.do_post("deleteStickerFromSet", params).await
    }

    pub async fn replace_sticker_in_set(
        &self,
        user_id: i64,
        name: &str,
        old_sticker: &str,
        sticker: serde_json::Value,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
            RequestParameter::new(
                "old_sticker",
                serde_json::Value::String(old_sticker.to_owned()),
            ),
            RequestParameter::new("sticker", sticker),
        ];
        self.do_post("replaceStickerInSet", params).await
    }

    pub async fn set_sticker_emoji_list(
        &self,
        sticker: &str,
        emoji_list: Vec<String>,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "sticker",
                serde_json::Value::String(sticker.to_owned()),
            ),
            RequestParameter::new("emoji_list", serde_json::to_value(&emoji_list)?),
        ];
        self.do_post("setStickerEmojiList", params).await
    }

    pub async fn set_sticker_keywords(
        &self,
        sticker: &str,
        keywords: Option<Vec<String>>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "sticker",
            serde_json::Value::String(sticker.to_owned()),
        )];
        push_opt(&mut params, "keywords", &keywords)?;
        self.do_post("setStickerKeywords", params).await
    }

    pub async fn set_sticker_mask_position(
        &self,
        sticker: &str,
        mask_position: Option<files::sticker::MaskPosition>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "sticker",
            serde_json::Value::String(sticker.to_owned()),
        )];
        push_opt(&mut params, "mask_position", &mask_position)?;
        self.do_post("setStickerMaskPosition", params).await
    }

    pub async fn set_sticker_set_thumbnail(
        &self,
        name: &str,
        user_id: i64,
        format: &str,
        thumbnail: Option<files::input_file::InputFile>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("format", serde_json::Value::String(format.to_owned())),
        ];
        push_opt_file(&mut params, "thumbnail", thumbnail);
        self.do_post("setStickerSetThumbnail", params).await
    }

    pub async fn set_sticker_set_title(&self, name: &str, title: &str) -> Result<bool> {
        let params = vec![
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
            RequestParameter::new("title", serde_json::Value::String(title.to_owned())),
        ];
        self.do_post("setStickerSetTitle", params).await
    }

    pub async fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        name: &str,
        custom_emoji_id: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "name",
            serde_json::Value::String(name.to_owned()),
        )];
        push_opt_str(&mut params, "custom_emoji_id", custom_emoji_id);
        self.do_post("setCustomEmojiStickerSetThumbnail", params)
            .await
    }

    pub async fn delete_sticker_set(&self, name: &str) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "name",
            serde_json::Value::String(name.to_owned()),
        )];
        self.do_post("deleteStickerSet", params).await
    }

    pub async fn get_forum_topic_icon_stickers(&self) -> Result<Vec<files::sticker::Sticker>> {
        self.do_post("getForumTopicIconStickers", Vec::new()).await
    }

    // ======================================================================
    // Editing messages
    // ======================================================================

    pub(crate) async fn edit_message_text_raw(
        &self,
        text: &str,
        chat_id: Option<ChatId>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
        parse_mode: Option<&str>,
        entities: Option<Vec<message_entity::MessageEntity>>,
        link_preview_options: Option<link_preview_options::LinkPreviewOptions>,
        reply_markup: Option<serde_json::Value>,
        business_connection_id: Option<&str>,
    ) -> Result<MessageOrBool> {
        let mut params = vec![RequestParameter::new(
            "text",
            serde_json::Value::String(text.to_owned()),
        )];
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "entities", &entities)?;
        push_opt(&mut params, "link_preview_options", &link_preview_options)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        self.do_post("editMessageText", params).await
    }

    pub(crate) async fn edit_message_caption_raw(
        &self,
        chat_id: Option<ChatId>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        reply_markup: Option<serde_json::Value>,
        show_caption_above_media: Option<bool>,
        business_connection_id: Option<&str>,
    ) -> Result<MessageOrBool> {
        let mut params = Vec::new();
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "show_caption_above_media", &show_caption_above_media)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        self.do_post("editMessageCaption", params).await
    }

    pub(crate) async fn edit_message_media_raw(
        &self,
        media: serde_json::Value,
        chat_id: Option<ChatId>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
        reply_markup: Option<serde_json::Value>,
        business_connection_id: Option<&str>,
    ) -> Result<MessageOrBool> {
        let mut params = vec![RequestParameter::new("media", media)];
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        self.do_post("editMessageMedia", params).await
    }

    pub(crate) async fn edit_message_reply_markup_raw(
        &self,
        chat_id: Option<ChatId>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
        reply_markup: Option<serde_json::Value>,
        business_connection_id: Option<&str>,
    ) -> Result<MessageOrBool> {
        let mut params = Vec::new();
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        self.do_post("editMessageReplyMarkup", params).await
    }

    pub async fn edit_message_live_location(
        &self,
        latitude: f64,
        longitude: f64,
        chat_id: Option<ChatId>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
        horizontal_accuracy: Option<f64>,
        heading: Option<i64>,
        proximity_alert_radius: Option<i64>,
        reply_markup: Option<serde_json::Value>,
        live_period: Option<i64>,
        business_connection_id: Option<&str>,
    ) -> Result<MessageOrBool> {
        let mut params = vec![
            RequestParameter::new("latitude", serde_json::to_value(latitude)?),
            RequestParameter::new("longitude", serde_json::to_value(longitude)?),
        ];
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        push_opt(&mut params, "horizontal_accuracy", &horizontal_accuracy)?;
        push_opt(&mut params, "heading", &heading)?;
        push_opt(&mut params, "proximity_alert_radius", &proximity_alert_radius)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "live_period", &live_period)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        self.do_post("editMessageLiveLocation", params).await
    }

    pub async fn stop_message_live_location(
        &self,
        chat_id: Option<ChatId>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
        reply_markup: Option<serde_json::Value>,
        business_connection_id: Option<&str>,
    ) -> Result<MessageOrBool> {
        let mut params = Vec::new();
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        self.do_post("stopMessageLiveLocation", params).await
    }

    pub async fn edit_message_checklist(
        &self,
        business_connection_id: &str,
        chat_id: i64,
        message_id: i64,
        checklist: input_checklist::InputChecklist,
        reply_markup: Option<serde_json::Value>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("chat_id", serde_json::to_value(chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
            RequestParameter::new("checklist", serde_json::to_value(&checklist)?),
        ];
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        self.do_post("editMessageChecklist", params).await
    }

    pub async fn stop_poll(
        &self,
        chat_id: ChatId,
        message_id: i64,
        reply_markup: Option<serde_json::Value>,
        business_connection_id: Option<&str>,
    ) -> Result<poll::Poll> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        self.do_post("stopPoll", params).await
    }

    // ======================================================================
    // User and profile
    // ======================================================================

    pub async fn get_user_profile_photos(
        &self,
        user_id: i64,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<user_profile_photos::UserProfilePhotos> {
        let mut params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(user_id)?,
        )];
        push_opt(&mut params, "offset", &offset)?;
        push_opt(&mut params, "limit", &limit)?;
        self.do_post("getUserProfilePhotos", params).await
    }

    pub async fn get_user_profile_audios(
        &self,
        user_id: i64,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<user_profile_audios::UserProfileAudios> {
        let mut params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(user_id)?,
        )];
        push_opt(&mut params, "offset", &offset)?;
        push_opt(&mut params, "limit", &limit)?;
        self.do_post("getUserProfileAudios", params).await
    }

    pub async fn set_user_emoji_status(
        &self,
        user_id: i64,
        emoji_status_custom_emoji_id: Option<&str>,
        emoji_status_expiration_date: Option<i64>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(user_id)?,
        )];
        push_opt_str(
            &mut params,
            "emoji_status_custom_emoji_id",
            emoji_status_custom_emoji_id,
        );
        push_opt(
            &mut params,
            "emoji_status_expiration_date",
            &emoji_status_expiration_date,
        )?;
        self.do_post("setUserEmojiStatus", params).await
    }

    pub async fn set_my_profile_photo(&self, photo: serde_json::Value) -> Result<bool> {
        let params = vec![RequestParameter::new("photo", photo)];
        self.do_post("setMyProfilePhoto", params).await
    }

    pub async fn remove_my_profile_photo(&self) -> Result<bool> {
        self.do_post("removeMyProfilePhoto", Vec::new()).await
    }

    // ======================================================================
    // Files
    // ======================================================================

    pub(crate) async fn get_file_raw(&self, file_id: &str) -> Result<files::file::File> {
        let params = vec![RequestParameter::new(
            "file_id",
            serde_json::Value::String(file_id.to_owned()),
        )];
        self.do_post("getFile", params).await
    }

    // ======================================================================
    // Chat management
    // ======================================================================

    pub async fn leave_chat(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("leaveChat", params).await
    }

    pub async fn get_chat(&self, chat_id: ChatId) -> Result<chat_full_info::ChatFullInfo> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("getChat", params).await
    }

    pub async fn get_chat_administrators(
        &self,
        chat_id: ChatId,
    ) -> Result<Vec<chat_member::ChatMember>> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("getChatAdministrators", params).await
    }

    pub async fn get_chat_member_count(&self, chat_id: ChatId) -> Result<i64> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("getChatMemberCount", params).await
    }

    pub async fn get_chat_member(
        &self,
        chat_id: ChatId,
        user_id: i64,
    ) -> Result<chat_member::ChatMember> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        self.do_post("getChatMember", params).await
    }

    pub async fn ban_chat_member(
        &self,
        chat_id: ChatId,
        user_id: i64,
        until_date: Option<i64>,
        revoke_messages: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        push_opt(&mut params, "until_date", &until_date)?;
        push_opt(&mut params, "revoke_messages", &revoke_messages)?;
        self.do_post("banChatMember", params).await
    }

    pub async fn unban_chat_member(
        &self,
        chat_id: ChatId,
        user_id: i64,
        only_if_banned: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        push_opt(&mut params, "only_if_banned", &only_if_banned)?;
        self.do_post("unbanChatMember", params).await
    }

    pub async fn ban_chat_sender_chat(
        &self,
        chat_id: ChatId,
        sender_chat_id: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("sender_chat_id", serde_json::to_value(sender_chat_id)?),
        ];
        self.do_post("banChatSenderChat", params).await
    }

    pub async fn unban_chat_sender_chat(
        &self,
        chat_id: ChatId,
        sender_chat_id: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("sender_chat_id", serde_json::to_value(sender_chat_id)?),
        ];
        self.do_post("unbanChatSenderChat", params).await
    }

    pub async fn restrict_chat_member(
        &self,
        chat_id: ChatId,
        user_id: i64,
        permissions: chat_permissions::ChatPermissions,
        until_date: Option<i64>,
        use_independent_chat_permissions: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("permissions", serde_json::to_value(&permissions)?),
        ];
        push_opt(&mut params, "until_date", &until_date)?;
        push_opt(
            &mut params,
            "use_independent_chat_permissions",
            &use_independent_chat_permissions,
        )?;
        self.do_post("restrictChatMember", params).await
    }

    pub async fn promote_chat_member(
        &self,
        chat_id: ChatId,
        user_id: i64,
        is_anonymous: Option<bool>,
        can_manage_chat: Option<bool>,
        can_post_messages: Option<bool>,
        can_edit_messages: Option<bool>,
        can_delete_messages: Option<bool>,
        can_manage_video_chats: Option<bool>,
        can_restrict_members: Option<bool>,
        can_promote_members: Option<bool>,
        can_change_info: Option<bool>,
        can_invite_users: Option<bool>,
        can_pin_messages: Option<bool>,
        can_manage_topics: Option<bool>,
        can_post_stories: Option<bool>,
        can_edit_stories: Option<bool>,
        can_delete_stories: Option<bool>,
        can_manage_direct_messages: Option<bool>,
        can_manage_tags: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        push_opt(&mut params, "is_anonymous", &is_anonymous)?;
        push_opt(&mut params, "can_manage_chat", &can_manage_chat)?;
        push_opt(&mut params, "can_post_messages", &can_post_messages)?;
        push_opt(&mut params, "can_edit_messages", &can_edit_messages)?;
        push_opt(&mut params, "can_delete_messages", &can_delete_messages)?;
        push_opt(&mut params, "can_manage_video_chats", &can_manage_video_chats)?;
        push_opt(&mut params, "can_restrict_members", &can_restrict_members)?;
        push_opt(&mut params, "can_promote_members", &can_promote_members)?;
        push_opt(&mut params, "can_change_info", &can_change_info)?;
        push_opt(&mut params, "can_invite_users", &can_invite_users)?;
        push_opt(&mut params, "can_pin_messages", &can_pin_messages)?;
        push_opt(&mut params, "can_manage_topics", &can_manage_topics)?;
        push_opt(&mut params, "can_post_stories", &can_post_stories)?;
        push_opt(&mut params, "can_edit_stories", &can_edit_stories)?;
        push_opt(&mut params, "can_delete_stories", &can_delete_stories)?;
        push_opt(&mut params, "can_manage_direct_messages", &can_manage_direct_messages)?;
        push_opt(&mut params, "can_manage_tags", &can_manage_tags)?;
        self.do_post("promoteChatMember", params).await
    }

    pub async fn set_chat_administrator_custom_title(
        &self,
        chat_id: ChatId,
        user_id: i64,
        custom_title: &str,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new(
                "custom_title",
                serde_json::Value::String(custom_title.to_owned()),
            ),
        ];
        self.do_post("setChatAdministratorCustomTitle", params)
            .await
    }

    pub async fn set_chat_permissions(
        &self,
        chat_id: ChatId,
        permissions: chat_permissions::ChatPermissions,
        use_independent_chat_permissions: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("permissions", serde_json::to_value(&permissions)?),
        ];
        push_opt(
            &mut params,
            "use_independent_chat_permissions",
            &use_independent_chat_permissions,
        )?;
        self.do_post("setChatPermissions", params).await
    }

    pub async fn set_chat_photo(
        &self,
        chat_id: ChatId,
        photo: files::input_file::InputFile,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            input_file_param("photo", photo),
        ];
        self.do_post("setChatPhoto", params).await
    }

    pub async fn delete_chat_photo(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("deleteChatPhoto", params).await
    }

    pub async fn set_chat_title(&self, chat_id: ChatId, title: &str) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("title", serde_json::Value::String(title.to_owned())),
        ];
        self.do_post("setChatTitle", params).await
    }

    pub async fn set_chat_description(
        &self,
        chat_id: ChatId,
        description: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        push_opt_str(&mut params, "description", description);
        self.do_post("setChatDescription", params).await
    }

    pub async fn set_chat_sticker_set(
        &self,
        chat_id: ChatId,
        sticker_set_name: &str,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "sticker_set_name",
                serde_json::Value::String(sticker_set_name.to_owned()),
            ),
        ];
        self.do_post("setChatStickerSet", params).await
    }

    pub async fn delete_chat_sticker_set(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("deleteChatStickerSet", params).await
    }

    pub async fn set_chat_member_tag(
        &self,
        chat_id: ChatId,
        user_id: i64,
        tag: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        push_opt_str(&mut params, "tag", tag);
        self.do_post("setChatMemberTag", params).await
    }

    // ======================================================================
    // Chat pinning
    // ======================================================================

    pub async fn pin_chat_message(
        &self,
        chat_id: ChatId,
        message_id: i64,
        disable_notification: Option<bool>,
        business_connection_id: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        self.do_post("pinChatMessage", params).await
    }

    pub async fn unpin_chat_message(
        &self,
        chat_id: ChatId,
        message_id: Option<i64>,
        business_connection_id: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        self.do_post("unpinChatMessage", params).await
    }

    pub async fn unpin_all_chat_messages(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("unpinAllChatMessages", params).await
    }

    // ======================================================================
    // Chat invite links
    // ======================================================================

    pub async fn export_chat_invite_link(&self, chat_id: ChatId) -> Result<String> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("exportChatInviteLink", params).await
    }

    pub async fn create_chat_invite_link(
        &self,
        chat_id: ChatId,
        expire_date: Option<i64>,
        member_limit: Option<i64>,
        name: Option<&str>,
        creates_join_request: Option<bool>,
    ) -> Result<chat_invite_link::ChatInviteLink> {
        let mut params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        push_opt(&mut params, "expire_date", &expire_date)?;
        push_opt(&mut params, "member_limit", &member_limit)?;
        push_opt_str(&mut params, "name", name);
        push_opt(&mut params, "creates_join_request", &creates_join_request)?;
        self.do_post("createChatInviteLink", params).await
    }

    pub async fn edit_chat_invite_link(
        &self,
        chat_id: ChatId,
        invite_link: &str,
        expire_date: Option<i64>,
        member_limit: Option<i64>,
        name: Option<&str>,
        creates_join_request: Option<bool>,
    ) -> Result<chat_invite_link::ChatInviteLink> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "invite_link",
                serde_json::Value::String(invite_link.to_owned()),
            ),
        ];
        push_opt(&mut params, "expire_date", &expire_date)?;
        push_opt(&mut params, "member_limit", &member_limit)?;
        push_opt_str(&mut params, "name", name);
        push_opt(&mut params, "creates_join_request", &creates_join_request)?;
        self.do_post("editChatInviteLink", params).await
    }

    pub async fn revoke_chat_invite_link(
        &self,
        chat_id: ChatId,
        invite_link: &str,
    ) -> Result<chat_invite_link::ChatInviteLink> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "invite_link",
                serde_json::Value::String(invite_link.to_owned()),
            ),
        ];
        self.do_post("revokeChatInviteLink", params).await
    }

    pub async fn create_chat_subscription_invite_link(
        &self,
        chat_id: ChatId,
        subscription_period: i64,
        subscription_price: i64,
        name: Option<&str>,
    ) -> Result<chat_invite_link::ChatInviteLink> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "subscription_period",
                serde_json::to_value(subscription_period)?,
            ),
            RequestParameter::new(
                "subscription_price",
                serde_json::to_value(subscription_price)?,
            ),
        ];
        push_opt_str(&mut params, "name", name);
        self.do_post("createChatSubscriptionInviteLink", params)
            .await
    }

    pub async fn edit_chat_subscription_invite_link(
        &self,
        chat_id: ChatId,
        invite_link: &str,
        name: Option<&str>,
    ) -> Result<chat_invite_link::ChatInviteLink> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "invite_link",
                serde_json::Value::String(invite_link.to_owned()),
            ),
        ];
        push_opt_str(&mut params, "name", name);
        self.do_post("editChatSubscriptionInviteLink", params)
            .await
    }

    pub async fn approve_chat_join_request(
        &self,
        chat_id: ChatId,
        user_id: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        self.do_post("approveChatJoinRequest", params).await
    }

    pub async fn decline_chat_join_request(
        &self,
        chat_id: ChatId,
        user_id: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        self.do_post("declineChatJoinRequest", params).await
    }

    // ======================================================================
    // Callback & inline queries
    // ======================================================================

    pub(crate) async fn answer_callback_query_raw(
        &self,
        callback_query_id: &str,
        text: Option<&str>,
        show_alert: Option<bool>,
        url: Option<&str>,
        cache_time: Option<i64>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "callback_query_id",
            serde_json::Value::String(callback_query_id.to_owned()),
        )];
        push_opt_str(&mut params, "text", text);
        push_opt(&mut params, "show_alert", &show_alert)?;
        push_opt_str(&mut params, "url", url);
        push_opt(&mut params, "cache_time", &cache_time)?;
        self.do_post("answerCallbackQuery", params).await
    }

    pub(crate) async fn answer_inline_query_raw(
        &self,
        inline_query_id: &str,
        results: Vec<serde_json::Value>,
        cache_time: Option<i64>,
        is_personal: Option<bool>,
        next_offset: Option<&str>,
        button: Option<serde_json::Value>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "inline_query_id",
                serde_json::Value::String(inline_query_id.to_owned()),
            ),
            RequestParameter::new("results", serde_json::to_value(&results)?),
        ];
        push_opt(&mut params, "cache_time", &cache_time)?;
        push_opt(&mut params, "is_personal", &is_personal)?;
        push_opt_str(&mut params, "next_offset", next_offset);
        push_opt(&mut params, "button", &button)?;
        self.do_post("answerInlineQuery", params).await
    }

    pub async fn save_prepared_inline_message(
        &self,
        user_id: i64,
        result: serde_json::Value,
        allow_user_chats: Option<bool>,
        allow_bot_chats: Option<bool>,
        allow_group_chats: Option<bool>,
        allow_channel_chats: Option<bool>,
    ) -> Result<serde_json::Value> {
        let mut params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("result", result),
        ];
        push_opt(&mut params, "allow_user_chats", &allow_user_chats)?;
        push_opt(&mut params, "allow_bot_chats", &allow_bot_chats)?;
        push_opt(&mut params, "allow_group_chats", &allow_group_chats)?;
        push_opt(&mut params, "allow_channel_chats", &allow_channel_chats)?;
        self.do_post("savePreparedInlineMessage", params).await
    }

    pub async fn answer_web_app_query(
        &self,
        web_app_query_id: &str,
        result: serde_json::Value,
    ) -> Result<sent_web_app_message::SentWebAppMessage> {
        let params = vec![
            RequestParameter::new(
                "web_app_query_id",
                serde_json::Value::String(web_app_query_id.to_owned()),
            ),
            RequestParameter::new("result", result),
        ];
        self.do_post("answerWebAppQuery", params).await
    }

    // ======================================================================
    // Chat menu & commands
    // ======================================================================

    pub async fn set_chat_menu_button(
        &self,
        chat_id: Option<i64>,
        menu_button: Option<menu_button::MenuButton>,
    ) -> Result<bool> {
        let mut params = Vec::new();
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "menu_button", &menu_button)?;
        self.do_post("setChatMenuButton", params).await
    }

    pub async fn get_chat_menu_button(
        &self,
        chat_id: Option<i64>,
    ) -> Result<menu_button::MenuButton> {
        let mut params = Vec::new();
        push_opt(&mut params, "chat_id", &chat_id)?;
        self.do_post("getChatMenuButton", params).await
    }

    pub async fn set_my_commands(
        &self,
        commands: Vec<bot_command::BotCommand>,
        scope: Option<bot_command_scope::BotCommandScope>,
        language_code: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "commands",
            serde_json::to_value(&commands)?,
        )];
        push_opt(&mut params, "scope", &scope)?;
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("setMyCommands", params).await
    }

    pub async fn get_my_commands(
        &self,
        scope: Option<bot_command_scope::BotCommandScope>,
        language_code: Option<&str>,
    ) -> Result<Vec<bot_command::BotCommand>> {
        let mut params = Vec::new();
        push_opt(&mut params, "scope", &scope)?;
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("getMyCommands", params).await
    }

    pub async fn delete_my_commands(
        &self,
        scope: Option<bot_command_scope::BotCommandScope>,
        language_code: Option<&str>,
    ) -> Result<bool> {
        let mut params = Vec::new();
        push_opt(&mut params, "scope", &scope)?;
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("deleteMyCommands", params).await
    }

    pub async fn set_my_default_administrator_rights(
        &self,
        rights: Option<chat_administrator_rights::ChatAdministratorRights>,
        for_channels: Option<bool>,
    ) -> Result<bool> {
        let mut params = Vec::new();
        push_opt(&mut params, "rights", &rights)?;
        push_opt(&mut params, "for_channels", &for_channels)?;
        self.do_post("setMyDefaultAdministratorRights", params)
            .await
    }

    pub async fn get_my_default_administrator_rights(
        &self,
        for_channels: Option<bool>,
    ) -> Result<chat_administrator_rights::ChatAdministratorRights> {
        let mut params = Vec::new();
        push_opt(&mut params, "for_channels", &for_channels)?;
        self.do_post("getMyDefaultAdministratorRights", params)
            .await
    }

    // ======================================================================
    // Bot description and name
    // ======================================================================

    pub async fn set_my_description(
        &self,
        description: Option<&str>,
        language_code: Option<&str>,
    ) -> Result<bool> {
        let mut params = Vec::new();
        push_opt_str(&mut params, "description", description);
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("setMyDescription", params).await
    }

    pub async fn get_my_description(
        &self,
        language_code: Option<&str>,
    ) -> Result<bot_description::BotDescription> {
        let mut params = Vec::new();
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("getMyDescription", params).await
    }

    pub async fn set_my_short_description(
        &self,
        short_description: Option<&str>,
        language_code: Option<&str>,
    ) -> Result<bool> {
        let mut params = Vec::new();
        push_opt_str(&mut params, "short_description", short_description);
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("setMyShortDescription", params).await
    }

    pub async fn get_my_short_description(
        &self,
        language_code: Option<&str>,
    ) -> Result<bot_description::BotShortDescription> {
        let mut params = Vec::new();
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("getMyShortDescription", params).await
    }

    pub async fn set_my_name(
        &self,
        name: Option<&str>,
        language_code: Option<&str>,
    ) -> Result<bool> {
        let mut params = Vec::new();
        push_opt_str(&mut params, "name", name);
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("setMyName", params).await
    }

    pub async fn get_my_name(
        &self,
        language_code: Option<&str>,
    ) -> Result<bot_name::BotName> {
        let mut params = Vec::new();
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("getMyName", params).await
    }

    // ======================================================================
    // Reactions & boosts
    // ======================================================================

    pub async fn set_message_reaction(
        &self,
        chat_id: ChatId,
        message_id: i64,
        reaction: Option<Vec<serde_json::Value>>,
        is_big: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        push_opt(&mut params, "reaction", &reaction)?;
        push_opt(&mut params, "is_big", &is_big)?;
        self.do_post("setMessageReaction", params).await
    }

    pub async fn get_user_chat_boosts(
        &self,
        chat_id: ChatId,
        user_id: i64,
    ) -> Result<chat_boost::UserChatBoosts> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        self.do_post("getUserChatBoosts", params).await
    }

    // ======================================================================
    // Games
    // ======================================================================

    pub async fn send_game(
        &self,
        chat_id: i64,
        game_short_name: &str,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(chat_id)?),
            RequestParameter::new(
                "game_short_name",
                serde_json::Value::String(game_short_name.to_owned()),
            ),
        ];
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        self.do_post("sendGame", params).await
    }

    pub async fn set_game_score(
        &self,
        user_id: i64,
        score: i64,
        force: Option<bool>,
        disable_edit_message: Option<bool>,
        chat_id: Option<i64>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
    ) -> Result<MessageOrBool> {
        let mut params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("score", serde_json::to_value(score)?),
        ];
        push_opt(&mut params, "force", &force)?;
        push_opt(&mut params, "disable_edit_message", &disable_edit_message)?;
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        self.do_post("setGameScore", params).await
    }

    pub async fn get_game_high_scores(
        &self,
        user_id: i64,
        chat_id: Option<i64>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
    ) -> Result<Vec<games::game_high_score::GameHighScore>> {
        let mut params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(user_id)?,
        )];
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        self.do_post("getGameHighScores", params).await
    }

    // ======================================================================
    // Payments
    // ======================================================================

    pub(crate) async fn send_invoice_raw(
        &self,
        chat_id: ChatId,
        title: &str,
        description: &str,
        payload: &str,
        currency: &str,
        prices: Vec<serde_json::Value>,
        provider_token: Option<&str>,
        max_tip_amount: Option<i64>,
        suggested_tip_amounts: Option<Vec<i64>>,
        start_parameter: Option<&str>,
        provider_data: Option<&str>,
        photo_url: Option<&str>,
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
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("title", serde_json::Value::String(title.to_owned())),
            RequestParameter::new(
                "description",
                serde_json::Value::String(description.to_owned()),
            ),
            RequestParameter::new("payload", serde_json::Value::String(payload.to_owned())),
            RequestParameter::new("currency", serde_json::Value::String(currency.to_owned())),
            RequestParameter::new("prices", serde_json::to_value(&prices)?),
        ];
        push_opt_str(&mut params, "provider_token", provider_token);
        push_opt(&mut params, "max_tip_amount", &max_tip_amount)?;
        push_opt(&mut params, "suggested_tip_amounts", &suggested_tip_amounts)?;
        push_opt_str(&mut params, "start_parameter", start_parameter);
        push_opt_str(&mut params, "provider_data", provider_data);
        push_opt_str(&mut params, "photo_url", photo_url);
        push_opt(&mut params, "photo_size", &photo_size)?;
        push_opt(&mut params, "photo_width", &photo_width)?;
        push_opt(&mut params, "photo_height", &photo_height)?;
        push_opt(&mut params, "need_name", &need_name)?;
        push_opt(&mut params, "need_phone_number", &need_phone_number)?;
        push_opt(&mut params, "need_email", &need_email)?;
        push_opt(&mut params, "need_shipping_address", &need_shipping_address)?;
        push_opt(
            &mut params,
            "send_phone_number_to_provider",
            &send_phone_number_to_provider,
        )?;
        push_opt(&mut params, "send_email_to_provider", &send_email_to_provider)?;
        push_opt(&mut params, "is_flexible", &is_flexible)?;
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "direct_messages_topic_id", &direct_messages_topic_id)?;
        push_opt(&mut params, "suggested_post_parameters", &suggested_post_parameters)?;
        self.do_post("sendInvoice", params).await
    }

    pub async fn create_invoice_link(
        &self,
        title: &str,
        description: &str,
        payload: &str,
        currency: &str,
        prices: Vec<serde_json::Value>,
        provider_token: Option<&str>,
        max_tip_amount: Option<i64>,
        suggested_tip_amounts: Option<Vec<i64>>,
        provider_data: Option<&str>,
        photo_url: Option<&str>,
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
        subscription_period: Option<i64>,
        business_connection_id: Option<&str>,
    ) -> Result<String> {
        let mut params = vec![
            RequestParameter::new("title", serde_json::Value::String(title.to_owned())),
            RequestParameter::new(
                "description",
                serde_json::Value::String(description.to_owned()),
            ),
            RequestParameter::new("payload", serde_json::Value::String(payload.to_owned())),
            RequestParameter::new("currency", serde_json::Value::String(currency.to_owned())),
            RequestParameter::new("prices", serde_json::to_value(&prices)?),
        ];
        push_opt_str(&mut params, "provider_token", provider_token);
        push_opt(&mut params, "max_tip_amount", &max_tip_amount)?;
        push_opt(&mut params, "suggested_tip_amounts", &suggested_tip_amounts)?;
        push_opt_str(&mut params, "provider_data", provider_data);
        push_opt_str(&mut params, "photo_url", photo_url);
        push_opt(&mut params, "photo_size", &photo_size)?;
        push_opt(&mut params, "photo_width", &photo_width)?;
        push_opt(&mut params, "photo_height", &photo_height)?;
        push_opt(&mut params, "need_name", &need_name)?;
        push_opt(&mut params, "need_phone_number", &need_phone_number)?;
        push_opt(&mut params, "need_email", &need_email)?;
        push_opt(&mut params, "need_shipping_address", &need_shipping_address)?;
        push_opt(
            &mut params,
            "send_phone_number_to_provider",
            &send_phone_number_to_provider,
        )?;
        push_opt(&mut params, "send_email_to_provider", &send_email_to_provider)?;
        push_opt(&mut params, "is_flexible", &is_flexible)?;
        push_opt(&mut params, "subscription_period", &subscription_period)?;
        push_opt_str(&mut params, "business_connection_id", business_connection_id);
        self.do_post("createInvoiceLink", params).await
    }

    pub(crate) async fn answer_shipping_query_raw(
        &self,
        shipping_query_id: &str,
        ok: bool,
        shipping_options: Option<Vec<serde_json::Value>>,
        error_message: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "shipping_query_id",
                serde_json::Value::String(shipping_query_id.to_owned()),
            ),
            RequestParameter::new("ok", serde_json::to_value(ok)?),
        ];
        push_opt(&mut params, "shipping_options", &shipping_options)?;
        push_opt_str(&mut params, "error_message", error_message);
        self.do_post("answerShippingQuery", params).await
    }

    pub(crate) async fn answer_pre_checkout_query_raw(
        &self,
        pre_checkout_query_id: &str,
        ok: bool,
        error_message: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "pre_checkout_query_id",
                serde_json::Value::String(pre_checkout_query_id.to_owned()),
            ),
            RequestParameter::new("ok", serde_json::to_value(ok)?),
        ];
        push_opt_str(&mut params, "error_message", error_message);
        self.do_post("answerPreCheckoutQuery", params).await
    }

    pub async fn refund_star_payment(
        &self,
        user_id: i64,
        telegram_payment_charge_id: &str,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new(
                "telegram_payment_charge_id",
                serde_json::Value::String(telegram_payment_charge_id.to_owned()),
            ),
        ];
        self.do_post("refundStarPayment", params).await
    }

    pub async fn get_star_transactions(
        &self,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<payment::stars::star_transactions::StarTransactions> {
        let mut params = Vec::new();
        push_opt(&mut params, "offset", &offset)?;
        push_opt(&mut params, "limit", &limit)?;
        self.do_post("getStarTransactions", params).await
    }

    pub async fn edit_user_star_subscription(
        &self,
        user_id: i64,
        telegram_payment_charge_id: &str,
        is_canceled: bool,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new(
                "telegram_payment_charge_id",
                serde_json::Value::String(telegram_payment_charge_id.to_owned()),
            ),
            RequestParameter::new("is_canceled", serde_json::to_value(is_canceled)?),
        ];
        self.do_post("editUserStarSubscription", params).await
    }

    pub async fn get_my_star_balance(&self) -> Result<payment::stars::star_amount::StarAmount> {
        self.do_post("getMyStarBalance", Vec::new()).await
    }

    // ======================================================================
    // Forum topics
    // ======================================================================

    pub async fn create_forum_topic(
        &self,
        chat_id: ChatId,
        name: &str,
        icon_color: Option<i64>,
        icon_custom_emoji_id: Option<&str>,
    ) -> Result<forum_topic::ForumTopic> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
        ];
        push_opt(&mut params, "icon_color", &icon_color)?;
        push_opt_str(&mut params, "icon_custom_emoji_id", icon_custom_emoji_id);
        self.do_post("createForumTopic", params).await
    }

    pub async fn edit_forum_topic(
        &self,
        chat_id: ChatId,
        message_thread_id: i64,
        name: Option<&str>,
        icon_custom_emoji_id: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("message_thread_id", serde_json::to_value(message_thread_id)?),
        ];
        push_opt_str(&mut params, "name", name);
        push_opt_str(&mut params, "icon_custom_emoji_id", icon_custom_emoji_id);
        self.do_post("editForumTopic", params).await
    }

    pub async fn close_forum_topic(&self, chat_id: ChatId, message_thread_id: i64) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("message_thread_id", serde_json::to_value(message_thread_id)?),
        ];
        self.do_post("closeForumTopic", params).await
    }

    pub async fn reopen_forum_topic(&self, chat_id: ChatId, message_thread_id: i64) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("message_thread_id", serde_json::to_value(message_thread_id)?),
        ];
        self.do_post("reopenForumTopic", params).await
    }

    pub async fn delete_forum_topic(&self, chat_id: ChatId, message_thread_id: i64) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("message_thread_id", serde_json::to_value(message_thread_id)?),
        ];
        self.do_post("deleteForumTopic", params).await
    }

    pub async fn unpin_all_forum_topic_messages(
        &self,
        chat_id: ChatId,
        message_thread_id: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("message_thread_id", serde_json::to_value(message_thread_id)?),
        ];
        self.do_post("unpinAllForumTopicMessages", params).await
    }

    pub async fn unpin_all_general_forum_topic_messages(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("unpinAllGeneralForumTopicMessages", params)
            .await
    }

    pub async fn edit_general_forum_topic(&self, chat_id: ChatId, name: &str) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
        ];
        self.do_post("editGeneralForumTopic", params).await
    }

    pub async fn close_general_forum_topic(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("closeGeneralForumTopic", params).await
    }

    pub async fn reopen_general_forum_topic(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("reopenGeneralForumTopic", params).await
    }

    pub async fn hide_general_forum_topic(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("hideGeneralForumTopic", params).await
    }

    pub async fn unhide_general_forum_topic(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("unhideGeneralForumTopic", params).await
    }

    // ======================================================================
    // Passport
    // ======================================================================

    pub async fn set_passport_data_errors(
        &self,
        user_id: i64,
        errors: Vec<serde_json::Value>,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("errors", serde_json::to_value(&errors)?),
        ];
        self.do_post("setPassportDataErrors", params).await
    }

    // ======================================================================
    // Gifts
    // ======================================================================

    pub async fn get_available_gifts(&self) -> Result<gifts::Gifts> {
        self.do_post("getAvailableGifts", Vec::new()).await
    }

    pub async fn send_gift(
        &self,
        gift_id: &str,
        user_id: Option<i64>,
        chat_id: Option<ChatId>,
        text: Option<&str>,
        text_parse_mode: Option<&str>,
        text_entities: Option<Vec<message_entity::MessageEntity>>,
        pay_for_upgrade: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "gift_id",
            serde_json::Value::String(gift_id.to_owned()),
        )];
        push_opt(&mut params, "user_id", &user_id)?;
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt_str(&mut params, "text", text);
        push_opt_str(&mut params, "text_parse_mode", text_parse_mode);
        push_opt(&mut params, "text_entities", &text_entities)?;
        push_opt(&mut params, "pay_for_upgrade", &pay_for_upgrade)?;
        self.do_post("sendGift", params).await
    }

    pub async fn gift_premium_subscription(
        &self,
        user_id: i64,
        month_count: i64,
        star_count: i64,
        text: Option<&str>,
        text_parse_mode: Option<&str>,
        text_entities: Option<Vec<message_entity::MessageEntity>>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("month_count", serde_json::to_value(month_count)?),
            RequestParameter::new("star_count", serde_json::to_value(star_count)?),
        ];
        push_opt_str(&mut params, "text", text);
        push_opt_str(&mut params, "text_parse_mode", text_parse_mode);
        push_opt(&mut params, "text_entities", &text_entities)?;
        self.do_post("giftPremiumSubscription", params).await
    }

    pub async fn get_user_gifts(
        &self,
        user_id: i64,
        exclude_unlimited: Option<bool>,
        exclude_limited_upgradable: Option<bool>,
        exclude_limited_non_upgradable: Option<bool>,
        exclude_from_blockchain: Option<bool>,
        exclude_unique: Option<bool>,
        sort_by_price: Option<bool>,
        offset: Option<&str>,
        limit: Option<i64>,
    ) -> Result<owned_gift::OwnedGifts> {
        let mut params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(user_id)?,
        )];
        push_opt(&mut params, "exclude_unlimited", &exclude_unlimited)?;
        push_opt(&mut params, "exclude_limited_upgradable", &exclude_limited_upgradable)?;
        push_opt(
            &mut params,
            "exclude_limited_non_upgradable",
            &exclude_limited_non_upgradable,
        )?;
        push_opt(&mut params, "exclude_from_blockchain", &exclude_from_blockchain)?;
        push_opt(&mut params, "exclude_unique", &exclude_unique)?;
        push_opt(&mut params, "sort_by_price", &sort_by_price)?;
        push_opt_str(&mut params, "offset", offset);
        push_opt(&mut params, "limit", &limit)?;
        self.do_post("getUserGifts", params).await
    }

    pub async fn get_chat_gifts(
        &self,
        chat_id: ChatId,
        exclude_unsaved: Option<bool>,
        exclude_saved: Option<bool>,
        exclude_unlimited: Option<bool>,
        exclude_limited_upgradable: Option<bool>,
        exclude_limited_non_upgradable: Option<bool>,
        exclude_from_blockchain: Option<bool>,
        exclude_unique: Option<bool>,
        sort_by_price: Option<bool>,
        offset: Option<&str>,
        limit: Option<i64>,
    ) -> Result<owned_gift::OwnedGifts> {
        let mut params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        push_opt(&mut params, "exclude_unsaved", &exclude_unsaved)?;
        push_opt(&mut params, "exclude_saved", &exclude_saved)?;
        push_opt(&mut params, "exclude_unlimited", &exclude_unlimited)?;
        push_opt(&mut params, "exclude_limited_upgradable", &exclude_limited_upgradable)?;
        push_opt(
            &mut params,
            "exclude_limited_non_upgradable",
            &exclude_limited_non_upgradable,
        )?;
        push_opt(&mut params, "exclude_from_blockchain", &exclude_from_blockchain)?;
        push_opt(&mut params, "exclude_unique", &exclude_unique)?;
        push_opt(&mut params, "sort_by_price", &sort_by_price)?;
        push_opt_str(&mut params, "offset", offset);
        push_opt(&mut params, "limit", &limit)?;
        self.do_post("getChatGifts", params).await
    }

    // ======================================================================
    // Business account management
    // ======================================================================

    pub async fn get_business_connection(
        &self,
        business_connection_id: &str,
    ) -> Result<business::BusinessConnection> {
        let params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(business_connection_id.to_owned()),
        )];
        self.do_post("getBusinessConnection", params).await
    }

    pub async fn get_business_account_gifts(
        &self,
        business_connection_id: &str,
        exclude_unsaved: Option<bool>,
        exclude_saved: Option<bool>,
        exclude_unlimited: Option<bool>,
        exclude_unique: Option<bool>,
        sort_by_price: Option<bool>,
        offset: Option<&str>,
        limit: Option<i64>,
        exclude_limited_upgradable: Option<bool>,
        exclude_limited_non_upgradable: Option<bool>,
        exclude_from_blockchain: Option<bool>,
    ) -> Result<owned_gift::OwnedGifts> {
        let mut params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(business_connection_id.to_owned()),
        )];
        push_opt(&mut params, "exclude_unsaved", &exclude_unsaved)?;
        push_opt(&mut params, "exclude_saved", &exclude_saved)?;
        push_opt(&mut params, "exclude_unlimited", &exclude_unlimited)?;
        push_opt(&mut params, "exclude_unique", &exclude_unique)?;
        push_opt(&mut params, "sort_by_price", &sort_by_price)?;
        push_opt_str(&mut params, "offset", offset);
        push_opt(&mut params, "limit", &limit)?;
        push_opt(&mut params, "exclude_limited_upgradable", &exclude_limited_upgradable)?;
        push_opt(
            &mut params,
            "exclude_limited_non_upgradable",
            &exclude_limited_non_upgradable,
        )?;
        push_opt(&mut params, "exclude_from_blockchain", &exclude_from_blockchain)?;
        self.do_post("getBusinessAccountGifts", params).await
    }

    pub async fn get_business_account_star_balance(
        &self,
        business_connection_id: &str,
    ) -> Result<payment::stars::star_amount::StarAmount> {
        let params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(business_connection_id.to_owned()),
        )];
        self.do_post("getBusinessAccountStarBalance", params).await
    }

    pub async fn read_business_message(
        &self,
        business_connection_id: &str,
        chat_id: i64,
        message_id: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("chat_id", serde_json::to_value(chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        self.do_post("readBusinessMessage", params).await
    }

    pub async fn delete_business_messages(
        &self,
        business_connection_id: &str,
        message_ids: Vec<i64>,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("message_ids", serde_json::to_value(&message_ids)?),
        ];
        self.do_post("deleteBusinessMessages", params).await
    }

    pub async fn set_business_account_name(
        &self,
        business_connection_id: &str,
        first_name: &str,
        last_name: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new(
                "first_name",
                serde_json::Value::String(first_name.to_owned()),
            ),
        ];
        push_opt_str(&mut params, "last_name", last_name);
        self.do_post("setBusinessAccountName", params).await
    }

    pub async fn set_business_account_username(
        &self,
        business_connection_id: &str,
        username: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(business_connection_id.to_owned()),
        )];
        push_opt_str(&mut params, "username", username);
        self.do_post("setBusinessAccountUsername", params).await
    }

    pub async fn set_business_account_bio(
        &self,
        business_connection_id: &str,
        bio: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(business_connection_id.to_owned()),
        )];
        push_opt_str(&mut params, "bio", bio);
        self.do_post("setBusinessAccountBio", params).await
    }

    pub async fn set_business_account_gift_settings(
        &self,
        business_connection_id: &str,
        show_gift_button: bool,
        accepted_gift_types: gifts::AcceptedGiftTypes,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("show_gift_button", serde_json::to_value(show_gift_button)?),
            RequestParameter::new(
                "accepted_gift_types",
                serde_json::to_value(&accepted_gift_types)?,
            ),
        ];
        self.do_post("setBusinessAccountGiftSettings", params)
            .await
    }

    pub async fn set_business_account_profile_photo(
        &self,
        business_connection_id: &str,
        photo: serde_json::Value,
        is_public: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("photo", photo),
        ];
        push_opt(&mut params, "is_public", &is_public)?;
        self.do_post("setBusinessAccountProfilePhoto", params)
            .await
    }

    pub async fn remove_business_account_profile_photo(
        &self,
        business_connection_id: &str,
        is_public: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(business_connection_id.to_owned()),
        )];
        push_opt(&mut params, "is_public", &is_public)?;
        self.do_post("removeBusinessAccountProfilePhoto", params)
            .await
    }

    pub async fn convert_gift_to_stars(
        &self,
        business_connection_id: &str,
        owned_gift_id: &str,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new(
                "owned_gift_id",
                serde_json::Value::String(owned_gift_id.to_owned()),
            ),
        ];
        self.do_post("convertGiftToStars", params).await
    }

    pub async fn upgrade_gift(
        &self,
        business_connection_id: &str,
        owned_gift_id: &str,
        keep_original_details: Option<bool>,
        star_count: Option<i64>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new(
                "owned_gift_id",
                serde_json::Value::String(owned_gift_id.to_owned()),
            ),
        ];
        push_opt(&mut params, "keep_original_details", &keep_original_details)?;
        push_opt(&mut params, "star_count", &star_count)?;
        self.do_post("upgradeGift", params).await
    }

    pub async fn transfer_gift(
        &self,
        business_connection_id: &str,
        owned_gift_id: &str,
        new_owner_chat_id: i64,
        star_count: Option<i64>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new(
                "owned_gift_id",
                serde_json::Value::String(owned_gift_id.to_owned()),
            ),
            RequestParameter::new("new_owner_chat_id", serde_json::to_value(new_owner_chat_id)?),
        ];
        push_opt(&mut params, "star_count", &star_count)?;
        self.do_post("transferGift", params).await
    }

    pub async fn transfer_business_account_stars(
        &self,
        business_connection_id: &str,
        star_count: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("star_count", serde_json::to_value(star_count)?),
        ];
        self.do_post("transferBusinessAccountStars", params).await
    }

    // ======================================================================
    // Stories
    // ======================================================================

    pub async fn post_story(
        &self,
        business_connection_id: &str,
        content: serde_json::Value,
        active_period: i64,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        areas: Option<Vec<serde_json::Value>>,
        post_to_chat_page: Option<bool>,
        protect_content: Option<bool>,
    ) -> Result<story::Story> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("content", content),
            RequestParameter::new("active_period", serde_json::to_value(active_period)?),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "areas", &areas)?;
        push_opt(&mut params, "post_to_chat_page", &post_to_chat_page)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        self.do_post("postStory", params).await
    }

    pub async fn edit_story(
        &self,
        business_connection_id: &str,
        story_id: i64,
        content: serde_json::Value,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        areas: Option<Vec<serde_json::Value>>,
    ) -> Result<story::Story> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("story_id", serde_json::to_value(story_id)?),
            RequestParameter::new("content", content),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "areas", &areas)?;
        self.do_post("editStory", params).await
    }

    pub async fn delete_story(
        &self,
        business_connection_id: &str,
        story_id: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("story_id", serde_json::to_value(story_id)?),
        ];
        self.do_post("deleteStory", params).await
    }

    pub async fn repost_story(
        &self,
        business_connection_id: &str,
        from_chat_id: i64,
        from_story_id: i64,
        active_period: i64,
        post_to_chat_page: Option<bool>,
        protect_content: Option<bool>,
    ) -> Result<story::Story> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("from_chat_id", serde_json::to_value(from_chat_id)?),
            RequestParameter::new("from_story_id", serde_json::to_value(from_story_id)?),
            RequestParameter::new("active_period", serde_json::to_value(active_period)?),
        ];
        push_opt(&mut params, "post_to_chat_page", &post_to_chat_page)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        self.do_post("repostStory", params).await
    }

    // ======================================================================
    // Verification
    // ======================================================================

    pub async fn verify_chat(
        &self,
        chat_id: ChatId,
        custom_description: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        push_opt_str(&mut params, "custom_description", custom_description);
        self.do_post("verifyChat", params).await
    }

    pub async fn verify_user(
        &self,
        user_id: i64,
        custom_description: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(user_id)?,
        )];
        push_opt_str(&mut params, "custom_description", custom_description);
        self.do_post("verifyUser", params).await
    }

    pub async fn remove_chat_verification(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("removeChatVerification", params).await
    }

    pub async fn remove_user_verification(&self, user_id: i64) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(user_id)?,
        )];
        self.do_post("removeUserVerification", params).await
    }

    // ======================================================================
    // Suggested posts
    // ======================================================================

    pub async fn approve_suggested_post(
        &self,
        chat_id: i64,
        message_id: i64,
        send_date: Option<i64>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        push_opt(&mut params, "send_date", &send_date)?;
        self.do_post("approveSuggestedPost", params).await
    }

    pub async fn decline_suggested_post(
        &self,
        chat_id: i64,
        message_id: i64,
        comment: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        push_opt_str(&mut params, "comment", comment);
        self.do_post("declineSuggestedPost", params).await
    }
}

// Second impl block – additional methods separated from the main one to avoid
// merge conflicts in the large primary impl block.
impl Bot {
    // ======================================================================
    // Managed bot methods (Bot API 9.6)
    // ======================================================================

    /// Use this method to get the bot token of a business bot that is managed
    /// by the current bot.
    ///
    /// Requires the *can_manage_bots* business-bot right.
    /// Returns the bot token as a [`String`] on success.
    pub(crate) async fn get_managed_bot_token_raw(&self, bot_user_id: i64) -> Result<String> {
        let params = vec![RequestParameter::new(
            "bot_user_id",
            serde_json::to_value(bot_user_id)?,
        )];
        self.do_post("getManagedBotToken", params).await
    }

    /// Use this method to replace the bot token of a business bot that is
    /// managed by the current bot.  The old token stops working immediately.
    ///
    /// Requires the *can_manage_bots* business-bot right.
    /// Returns the new bot token as a [`String`] on success.
    pub(crate) async fn replace_managed_bot_token_raw(&self, bot_user_id: i64) -> Result<String> {
        let params = vec![RequestParameter::new(
            "bot_user_id",
            serde_json::to_value(bot_user_id)?,
        )];
        self.do_post("replaceManagedBotToken", params).await
    }

    // ======================================================================
    // Keyboard button methods (Bot API 9.6)
    // ======================================================================

    /// Use this method to save a keyboard button to be later shown to the
    /// user via a Mini App.
    ///
    /// Returns a [`PreparedKeyboardButton`](crate::types::prepared_keyboard_button::PreparedKeyboardButton)
    /// on success.
    pub(crate) async fn save_prepared_keyboard_button_raw(
        &self,
        user_id: i64,
        button: inline::inline_keyboard_button::InlineKeyboardButton,
    ) -> Result<prepared_keyboard_button::PreparedKeyboardButton> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("button", serde_json::to_value(&button)?),
        ];
        self.do_post("savePreparedKeyboardButton", params).await
    }
}
