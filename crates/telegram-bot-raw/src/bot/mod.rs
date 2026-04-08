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
use tokio::sync::OnceCell;

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
    /// Default parse mode for text formatting (e.g. `"HTML"`, `"MarkdownV2"`).
    pub parse_mode: Option<String>,
    /// Whether to send messages silently by default.
    pub disable_notification: Option<bool>,
    /// Whether to protect forwarded messages from being saved by default.
    pub protect_content: Option<bool>,
    /// Whether to allow sending without a reply by default.
    pub allow_sending_without_reply: Option<bool>,
    /// Default link preview options.
    pub link_preview_options: Option<link_preview_options::LinkPreviewOptions>,
    /// Whether to quote the original message by default when replying.
    pub quote: Option<bool>,
}

// ---------------------------------------------------------------------------
// Bot struct
// ---------------------------------------------------------------------------
/// The core Telegram Bot API client.
///
/// `Bot` holds the API token, HTTP request backend, and optional defaults.
/// It provides async methods for every Telegram Bot API endpoint (sending
/// messages, managing chats, uploading files, etc.).
///
/// # Construction
///
/// Use [`Bot::new`] for the simplest case or [`Bot::with_options`] for full
/// control over request backends and defaults.
///
/// # Thread safety
///
/// `Bot` is `Send + Sync` and can be shared across tasks via `Arc<Bot>`.
pub struct Bot {
    token: Arc<str>,
    base_url: Arc<str>,
    base_file_url: Arc<str>,
    request: Arc<dyn BaseRequest>,
    /// Separate request object for `getUpdates` long-polling (M3).
    request_for_updates: Arc<dyn BaseRequest>,
    /// User-configured defaults merged into outgoing API calls (C10).
    defaults: Option<Defaults>,
    /// Cached result of `get_me()` after `initialize()` (M5).
    cached_bot_data: Arc<OnceCell<user::User>>,
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
                value: Some(serde_json::Value::String(format!(
                    "__filepath__:{path_str}"
                ))),
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

// ---------------------------------------------------------------------------
// Core impl: constructors, infrastructure, get_updates, basic methods
// ---------------------------------------------------------------------------

#[allow(dead_code)]
impl Bot {
    /// Creates a new `Bot` with the given token and HTTP request backend.
    ///
    /// Uses the Telegram production API endpoint (`https://api.telegram.org`).
    /// For custom endpoints (e.g. a local Bot API server), use [`Bot::with_options`].
    pub fn new(token: impl Into<String>, request: Arc<dyn BaseRequest>) -> Self {
        let token = token.into();
        let base_url: Arc<str> = format!("https://api.telegram.org/bot{token}").into();
        let base_file_url: Arc<str> = format!("https://api.telegram.org/file/bot{token}").into();
        let token: Arc<str> = token.into();
        Self {
            token,
            base_url,
            base_file_url,
            request_for_updates: Arc::clone(&request),
            request,
            defaults: None,
            cached_bot_data: Arc::new(OnceCell::new()),
            local_mode: false,
        }
    }

    /// Creates a `Bot` with full configuration options.
    ///
    /// Allows a separate HTTP backend for `getUpdates` long-polling and
    /// optional [`Defaults`] to merge into every API call.
    pub fn with_options(
        token: impl Into<String>,
        request: Arc<dyn BaseRequest>,
        request_for_updates: Option<Arc<dyn BaseRequest>>,
        defaults: Option<Defaults>,
    ) -> Self {
        let token = token.into();
        let base_url: Arc<str> = format!("https://api.telegram.org/bot{token}").into();
        let base_file_url: Arc<str> = format!("https://api.telegram.org/file/bot{token}").into();
        let token: Arc<str> = token.into();
        let request_for_updates = request_for_updates.unwrap_or_else(|| Arc::clone(&request));
        Self {
            token,
            base_url,
            base_file_url,
            request_for_updates,
            request,
            defaults,
            cached_bot_data: Arc::new(OnceCell::new()),
            local_mode: false,
        }
    }

    /// Returns the bot token.
    pub fn token(&self) -> &str {
        &self.token
    }
    /// Returns the base API URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
    /// Returns the base file-download URL.
    pub fn base_file_url(&self) -> &str {
        &self.base_file_url
    }
    /// Returns the user-configured defaults, if any.
    pub fn defaults(&self) -> Option<&Defaults> {
        self.defaults.as_ref()
    }
    /// Returns the cached bot user data from `get_me()`, if initialized.
    pub fn bot_data(&self) -> Option<&user::User> {
        self.cached_bot_data.get()
    }

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

    fn api_url(&self, method: &str) -> String {
        format!("{}/{method}", self.base_url)
    }

    async fn resolve_file_paths(&self, params: &mut [RequestParameter]) -> Result<()> {
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
                        for f in files.iter_mut() {
                            if f.bytes.is_empty() {
                                f.bytes = data.clone();
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn apply_defaults(&self, params: &mut Vec<RequestParameter>) {
        let defaults = match &self.defaults {
            Some(d) => d,
            None => return,
        };
        let existing: std::collections::HashSet<String> =
            params.iter().map(|p| p.name.as_ref().to_owned()).collect();
        if let Some(ref pm) = defaults.parse_mode {
            if !existing.contains("parse_mode") {
                params.push(RequestParameter::new(
                    "parse_mode",
                    serde_json::Value::String(pm.clone()),
                ));
            }
        }
        if let Some(dn) = defaults.disable_notification {
            if !existing.contains("disable_notification") {
                params.push(RequestParameter::new(
                    "disable_notification",
                    serde_json::Value::Bool(dn),
                ));
            }
        }
        if let Some(pc) = defaults.protect_content {
            if !existing.contains("protect_content") {
                params.push(RequestParameter::new(
                    "protect_content",
                    serde_json::Value::Bool(pc),
                ));
            }
        }
        if let Some(aswr) = defaults.allow_sending_without_reply {
            if !existing.contains("allow_sending_without_reply") {
                params.push(RequestParameter::new(
                    "allow_sending_without_reply",
                    serde_json::Value::Bool(aswr),
                ));
            }
        }
        if let Some(ref lpo) = defaults.link_preview_options {
            if !existing.contains("link_preview_options") {
                if let Ok(v) = serde_json::to_value(lpo) {
                    params.push(RequestParameter::new("link_preview_options", v));
                }
            }
        }
    }

    async fn do_post<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Vec<RequestParameter>,
    ) -> Result<T> {
        self.do_post_inner(method, params, TimeoutOverride::default_none(), None)
            .await
    }

    #[allow(dead_code)]
    async fn do_post_with_timeouts<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Vec<RequestParameter>,
        timeouts: TimeoutOverride,
    ) -> Result<T> {
        self.do_post_inner(method, params, timeouts, None).await
    }

    /// Central dispatch -- heap-allocates the future via Box::pin to prevent
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
                let existing: std::collections::HashSet<String> =
                    params.iter().map(|p| p.name.as_ref().to_owned()).collect();
                for (key, value) in kwargs {
                    if !existing.contains(key.as_str()) {
                        params.push(RequestParameter::new(key, value));
                    }
                }
            }
            self.resolve_file_paths(&mut params).await?;
            let url = self.api_url(method);
            let data = RequestData::from_parameters(params);
            let result = self.request.post(&url, Some(&data), timeouts).await?;
            serde_json::from_value(result).map_err(Into::into)
        })
    }

    /// Downloads a file from the Telegram servers given its `file_path`.
    pub async fn download_file(&self, file_path: &str) -> Result<Vec<u8>> {
        let url = format!("{}/{file_path}", self.base_file_url);
        let bytes = self
            .request
            .retrieve(&url, TimeoutOverride::default_none())
            .await?;
        Ok(bytes.to_vec())
    }

    /// Initializes the bot by calling `get_me()` and caching the result.
    pub async fn initialize(&mut self) -> Result<()> {
        self.request.initialize().await?;
        if !Arc::ptr_eq(&self.request, &self.request_for_updates) {
            self.request_for_updates.initialize().await?;
        }
        let me = self.get_me().await?;
        let _ = self.cached_bot_data.set(me);
        Ok(())
    }

    /// Shuts down the bot and releases the HTTP request backend.
    /// Shuts down the bot and releases the HTTP request backend.
    pub async fn shutdown(&self) -> Result<()> {
        if !Arc::ptr_eq(&self.request, &self.request_for_updates) {
            self.request_for_updates.shutdown().await?;
        }
        Ok(())
    }

    pub async fn do_api_request<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Vec<RequestParameter>,
    ) -> Result<T> {
        self.do_post(method, params).await
    }

    pub async fn do_api_request_with_kwargs<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Vec<RequestParameter>,
        api_kwargs: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<T> {
        self.do_post_inner(method, params, TimeoutOverride::default_none(), api_kwargs)
            .await
    }

    // ======================================================================
    // Getting updates
    // ======================================================================

    /// Calls the Telegram `getUpdates` long-polling endpoint.
    pub async fn get_updates(
        &self,
        offset: Option<i64>,
        limit: Option<i32>,
        timeout: Option<i32>,
        allowed_updates: Option<Vec<String>>,
    ) -> Result<Vec<update::Update>> {
        let mut params = Vec::new();
        push_opt(&mut params, "offset", &offset)?;
        push_opt(&mut params, "limit", &limit)?;
        push_opt(&mut params, "timeout", &timeout)?;
        push_opt(&mut params, "allowed_updates", &allowed_updates)?;
        self.apply_defaults(&mut params);
        let timeouts = if let Some(t) = timeout {
            let effective = Duration::from_secs(t as u64 + 2);
            TimeoutOverride {
                read: Some(Some(effective)),
                ..TimeoutOverride::default_none()
            }
        } else {
            TimeoutOverride::default_none()
        };
        let url = self.api_url("getUpdates");
        let data = RequestData::from_parameters(params);
        let result = self
            .request_for_updates
            .post(&url, Some(&data), timeouts)
            .await?;
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

    /// Calls the `getMe` endpoint, returning the bot's own [`User`](user::User) object.
    pub async fn get_me(&self) -> Result<user::User> {
        self.do_post("getMe", Vec::new()).await
    }

    pub async fn log_out(&self) -> Result<bool> {
        self.do_post("logOut", Vec::new()).await
    }

    pub async fn close(&self) -> Result<bool> {
        self.do_post("close", Vec::new()).await
    }
}

// ---------------------------------------------------------------------------
// Per-method-group submodules
// ---------------------------------------------------------------------------

mod admin;
mod business_methods;
mod chat;
mod editing;
mod forum;
mod games_methods;
mod gifts_methods;
mod inline_methods;
mod keyboard_methods;
mod managed_bots;
mod media;
mod messages;
mod other_content;
mod passport;
mod payments;
mod reactions;
mod stickers;
mod stories;
mod suggested_posts;
mod user_profile;
mod verification;
