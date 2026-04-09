//! Abstract interface for making HTTP requests to the Telegram Bot API.
//!
//! This module mirrors `telegram.request.BaseRequest` from python-telegram-bot.
//! The central piece is the [`BaseRequest`] trait; everything else in this
//! module is shared infrastructure used by all implementations.

use std::time::Duration;

use serde_json::Value;
use tracing::debug;

use crate::error::{Result, TelegramError};

use super::request_data::RequestData;

// ---------------------------------------------------------------------------
// Supporting types
// ---------------------------------------------------------------------------

/// HTTP methods used when issuing requests to the Bot API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    /// `POST` — used for all Bot API method calls.
    Post,
    /// `GET` — used when downloading files from Telegram CDN URLs.
    Get,
}

impl HttpMethod {
    /// The method name as an uppercase string slice, ready to pass to reqwest.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Post => "POST",
            Self::Get => "GET",
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Per-request timeout configuration.
///
/// Each field is `Option<Option<Duration>>`:
/// - `None` — caller did not specify; use the implementation's default.
/// - `Some(None)` — caller explicitly requested "no timeout".
/// - `Some(Some(d))` — caller explicitly set a specific duration.
///
/// This two-tier encoding mirrors the Python `DEFAULT_NONE` / `DefaultValue`
/// sentinel mechanism.
#[derive(Debug, Clone, Copy, Default)]
pub struct TimeoutOverride {
    /// Maximum time to wait for a TCP connection to be established.
    pub connect: Option<Option<Duration>>,
    /// Maximum time to wait for the full response to arrive.
    pub read: Option<Option<Duration>>,
    /// Maximum time to wait while sending the request body.
    pub write: Option<Option<Duration>>,
    /// Maximum time to wait for a free connection from the pool.
    pub pool: Option<Option<Duration>>,
}

impl TimeoutOverride {
    /// All fields left as `None` (use the implementation's defaults for every
    /// timeout dimension).
    pub const fn default_none() -> Self {
        Self {
            connect: None,
            read: None,
            write: None,
            pool: None,
        }
    }
}

/// Concrete timeout values resolved by an implementation after applying
/// caller overrides on top of its own defaults.
#[derive(Debug, Clone, Copy)]
pub struct ResolvedTimeouts {
    /// Effective connect timeout (`None` = wait forever).
    pub connect: Option<Duration>,
    /// Effective read timeout (`None` = wait forever).
    pub read: Option<Duration>,
    /// Effective write timeout (`None` = wait forever).
    pub write: Option<Duration>,
    /// Effective pool timeout (`None` = wait forever).
    pub pool: Option<Duration>,
}

// ---------------------------------------------------------------------------
// The trait
// ---------------------------------------------------------------------------

/// Abstract interface for sending HTTP requests to the Telegram Bot API.
///
/// Implementors must provide:
/// - [`BaseRequest::initialize`] — open connections / warm up resources.
/// - [`BaseRequest::shutdown`] — close connections / release resources.
/// - [`BaseRequest::do_request`] — the raw HTTP round-trip.
/// - [`BaseRequest::do_request_json_bytes`] — POST pre-serialized JSON bytes.
/// - [`BaseRequest::default_read_timeout`] — the default read timeout so that
///   provided methods can forward it downstream.
///
/// All other methods are provided as default implementations that implementors
/// may override.
///
/// # Context manager equivalent
///
/// The Python `async with request_object:` pattern maps to:
///
/// ```ignore
/// request.initialize().await?;
/// // ... work ...
/// request.shutdown().await;
/// ```
#[async_trait::async_trait]
pub trait BaseRequest: Send + Sync {
    // ------------------------------------------------------------------
    // Abstract methods
    // ------------------------------------------------------------------

    /// Open connections and allocate resources required by this implementation.
    async fn initialize(&self) -> Result<()>;

    /// Close connections and release resources held by this implementation.
    ///
    /// Must not return an error even if the implementation is already shut
    /// down — log a debug message and return `Ok(())` instead.
    async fn shutdown(&self) -> Result<()>;

    /// The default read timeout used when the caller does not supply an
    /// override.
    fn default_read_timeout(&self) -> Option<Duration>;

    /// Perform the actual HTTP round-trip.
    ///
    /// Returns `(status_code, response_body)`.
    ///
    /// Implementations MUST convert transport-level errors into
    /// [`TelegramError::Network`] or [`TelegramError::TimedOut`] before
    /// returning — they must never let `reqwest::Error` or similar leak out.
    async fn do_request(
        &self,
        url: &str,
        method: HttpMethod,
        request_data: Option<&RequestData>,
        timeouts: TimeoutOverride,
    ) -> Result<(u16, bytes::Bytes)>;

    /// POST pre-serialized JSON bytes directly, bypassing [`RequestData`]
    /// construction.
    ///
    /// This eliminates the double-serialization overhead for text-only API
    /// methods: the caller serializes a typed struct to `Vec<u8>` once via
    /// `serde_json::to_vec`, and this method sends those bytes with
    /// `Content-Type: application/json`.
    ///
    /// Returns `(status_code, response_body)`.
    async fn do_request_json_bytes(
        &self,
        url: &str,
        body: &[u8],
        timeouts: TimeoutOverride,
    ) -> Result<(u16, bytes::Bytes)>;

    // ------------------------------------------------------------------
    // Provided methods
    // ------------------------------------------------------------------

    /// High-level POST call used by `Bot` methods.
    ///
    /// Calls [`Self::request_wrapper`] and then extracts `result` from the
    /// Telegram JSON envelope.
    ///
    /// Mirrors `BaseRequest.post` in Python.
    async fn post(
        &self,
        url: &str,
        request_data: Option<&RequestData>,
        timeouts: TimeoutOverride,
    ) -> Result<Value> {
        let raw = self
            .request_wrapper(url, HttpMethod::Post, request_data, timeouts)
            .await?;
        // Use the free-function variant so we are not constrained by Self: Sized.
        let json_data = parse_json_payload_impl(&raw)?;
        // https://core.telegram.org/bots/api#making-requests — successful
        // responses always carry a "result" key.
        json_data
            .get("result")
            .cloned()
            .ok_or_else(|| TelegramError::Network("Missing 'result' field in API response".into()))
    }

    /// High-level POST call that sends pre-serialized JSON bytes.
    ///
    /// Eliminates double serialization for text-only API methods by sending
    /// raw bytes directly with `Content-Type: application/json`, then
    /// extracting `result` from the Telegram JSON envelope.
    async fn post_json(&self, url: &str, body: &[u8], timeouts: TimeoutOverride) -> Result<Value> {
        let (code, payload) = self.do_request_json_bytes(url, body, timeouts).await?;

        if (200..=299).contains(&code) {
            let json_data = parse_json_payload_impl(&payload)?;
            return json_data.get("result").cloned().ok_or_else(|| {
                TelegramError::Network("Missing 'result' field in API response".into())
            });
        }

        // Reuse the same error-handling logic as request_wrapper.
        let (message, migrate_chat_id, retry_after, extra_params) =
            parse_error_body(&payload, code);

        if let Some(new_chat_id) = migrate_chat_id {
            return Err(TelegramError::ChatMigrated { new_chat_id });
        }
        if let Some(secs) = retry_after {
            return Err(TelegramError::RetryAfter {
                retry_after: Duration::from_secs(secs),
            });
        }

        let full_message = if let Some(params) = extra_params {
            format!("{message}. The server response contained unknown parameters: {params}")
        } else {
            message
        };

        let err = match code {
            403 => TelegramError::Forbidden(full_message),
            401 | 404 => TelegramError::InvalidToken(full_message),
            400 => TelegramError::BadRequest(full_message),
            409 => TelegramError::Conflict(full_message),
            _ => TelegramError::Network(full_message),
        };

        Err(err)
    }

    /// File download helper — issues a GET request and returns raw bytes.
    ///
    /// Mirrors `BaseRequest.retrieve` in Python.
    async fn retrieve(&self, url: &str, timeouts: TimeoutOverride) -> Result<bytes::Bytes> {
        self.request_wrapper(url, HttpMethod::Get, None, timeouts)
            .await
    }

    /// Wraps [`Self::do_request`], translating HTTP status codes into the
    /// appropriate [`TelegramError`] variants.
    ///
    /// Mirrors `BaseRequest._request_wrapper` in Python.
    async fn request_wrapper(
        &self,
        url: &str,
        method: HttpMethod,
        request_data: Option<&RequestData>,
        timeouts: TimeoutOverride,
    ) -> Result<bytes::Bytes> {
        let (code, payload) = match self.do_request(url, method, request_data, timeouts).await {
            Ok(pair) => pair,
            // TelegramErrors that bubbled up from do_request are re-raised as-is.
            Err(e) => return Err(e),
        };

        if (200..=299).contains(&code) {
            return Ok(payload);
        }

        // Attempt to extract the Telegram error description from the JSON body.
        let (message, migrate_chat_id, retry_after, extra_params) =
            parse_error_body(&payload, code);

        // Special-case response parameters before dispatching on status code.
        if let Some(new_chat_id) = migrate_chat_id {
            return Err(TelegramError::ChatMigrated { new_chat_id });
        }
        if let Some(secs) = retry_after {
            return Err(TelegramError::RetryAfter {
                retry_after: Duration::from_secs(secs),
            });
        }

        let full_message = if let Some(params) = extra_params {
            format!("{message}. The server response contained unknown parameters: {params}")
        } else {
            message
        };

        let err = match code {
            403 => TelegramError::Forbidden(full_message),
            // 401 Unauthorized and 404 Not Found both map to InvalidToken.
            401 | 404 => TelegramError::InvalidToken(full_message),
            400 => TelegramError::BadRequest(full_message),
            409 => TelegramError::Conflict(full_message),
            // 502 Bad Gateway and anything else are network errors.
            _ => TelegramError::Network(full_message),
        };

        Err(err)
    }

    /// Parse a UTF-8 JSON payload returned by Telegram.
    ///
    /// Returns a [`TelegramError::Network`] when the bytes are not valid JSON,
    /// mirroring the Python `TelegramError("Invalid server response")`.
    ///
    /// Implementors may override this method to use a custom JSON library.
    ///
    /// The default implementation delegates to [`parse_json_payload_impl`].
    fn parse_json_payload(&self, payload: &[u8]) -> Result<Value> {
        parse_json_payload_impl(payload)
    }
}

// ---------------------------------------------------------------------------
// Free-function helpers (callable without a receiver or Self: Sized bound)
// ---------------------------------------------------------------------------

/// Parse a UTF-8 byte slice as JSON, producing a [`TelegramError::Network`]
/// error on failure.
///
/// ```
/// use rust_tg_bot_raw::request::base::parse_json_payload_impl;
///
/// let raw = br#"{"ok":true,"result":42}"#;
/// let v = parse_json_payload_impl(raw).unwrap();
/// assert_eq!(v["result"], 42);
/// ```
pub fn parse_json_payload_impl(payload: &[u8]) -> Result<Value> {
    // Decode with replacement characters on invalid UTF-8, matching the
    // Python `errors="replace"` strategy.
    let text = String::from_utf8_lossy(payload);
    serde_json::from_str(&text).map_err(|e| {
        debug!("Cannot parse server response as JSON: {e}  payload={text:?}");
        TelegramError::Network(format!("Invalid server response: {e}"))
    })
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Extract the human-readable message and any special parameters from an error
/// body.
///
/// Returns `(message, migrate_to_chat_id, retry_after_secs, unknown_params)`.
fn parse_error_body(
    payload: &[u8],
    code: u16,
) -> (String, Option<i64>, Option<u64>, Option<String>) {
    let fallback_message = http_status_phrase(code);

    match parse_json_payload_impl(payload) {
        Err(_) => {
            // Body is not valid JSON — return a descriptive fallback.
            let raw = String::from_utf8_lossy(payload);
            let msg = format!("{fallback_message}. Parsing the server response {raw:?} failed");
            (msg, None, None, None)
        }
        Ok(body) => {
            let description = body
                .get("description")
                .and_then(Value::as_str)
                .map(str::to_owned)
                .unwrap_or(fallback_message);

            let parameters = body.get("parameters");

            let migrate_to_chat_id = parameters
                .and_then(|p| p.get("migrate_to_chat_id"))
                .and_then(Value::as_i64);

            let retry_after = parameters
                .and_then(|p| p.get("retry_after"))
                .and_then(Value::as_u64);

            // Any parameters that are neither migrate_to_chat_id nor
            // retry_after are "unknown".
            let extra = parameters.and_then(|p| {
                if let Value::Object(map) = p {
                    let unknown: serde_json::Map<String, Value> = map
                        .iter()
                        .filter(|(k, _)| {
                            k.as_str() != "migrate_to_chat_id" && k.as_str() != "retry_after"
                        })
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();
                    if unknown.is_empty() {
                        None
                    } else {
                        Some(Value::Object(unknown).to_string())
                    }
                } else {
                    None
                }
            });

            (description, migrate_to_chat_id, retry_after, extra)
        }
    }
}

/// Best-effort HTTP status phrase lookup.
fn http_status_phrase(code: u16) -> String {
    let phrase = match code {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        409 => "Conflict",
        420 => "Enhance Your Calm",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        _ => "Unknown HTTP Error",
    };
    format!("{phrase} ({code})")
}

// ---------------------------------------------------------------------------
// Re-export async_trait so that callers don't need to depend on it directly.
// ---------------------------------------------------------------------------
pub use async_trait::async_trait;

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------
    // HttpMethod
    // ------------------------------------------------------------------

    #[test]
    fn http_method_as_str() {
        assert_eq!(HttpMethod::Post.as_str(), "POST");
        assert_eq!(HttpMethod::Get.as_str(), "GET");
    }

    #[test]
    fn http_method_display() {
        assert_eq!(HttpMethod::Post.to_string(), "POST");
    }

    // ------------------------------------------------------------------
    // parse_json_payload_impl
    // ------------------------------------------------------------------

    #[test]
    fn parse_valid_json() {
        let raw = br#"{"ok":true,"result":{"id":1}}"#;
        let v = parse_json_payload_impl(raw).unwrap();
        assert_eq!(v["ok"], true);
        assert_eq!(v["result"]["id"], 1);
    }

    #[test]
    fn parse_invalid_json_returns_network_error() {
        let raw = b"not json {{";
        let err = parse_json_payload_impl(raw).unwrap_err();
        assert!(
            matches!(err, TelegramError::Network(_)),
            "expected Network, got {err:?}"
        );
    }

    #[test]
    fn parse_invalid_utf8_with_replacement() {
        // 0xFF is not valid UTF-8 but we must not panic — the JSON parse will
        // fail gracefully with a Network error.
        let raw = b"\xff\xfe{\"ok\":true}";
        // Either a valid parse (if replacement chars still form valid JSON) or
        // a graceful Network error — no panics.
        let _ = parse_json_payload_impl(raw);
    }

    // ------------------------------------------------------------------
    // parse_error_body
    // ------------------------------------------------------------------

    #[test]
    fn parse_error_body_extracts_description() {
        let body = br#"{"ok":false,"error_code":400,"description":"Bad Request: chat not found"}"#;
        let (msg, migrate, retry, extra) = parse_error_body(body, 400);
        assert_eq!(msg, "Bad Request: chat not found");
        assert!(migrate.is_none());
        assert!(retry.is_none());
        assert!(extra.is_none());
    }

    #[test]
    fn parse_error_body_migrate_chat_id() {
        let body = br#"{"ok":false,"error_code":400,"description":"...","parameters":{"migrate_to_chat_id":-1001234567}}"#;
        let (_, migrate, _, _) = parse_error_body(body, 400);
        assert_eq!(migrate, Some(-1_001_234_567_i64));
    }

    #[test]
    fn parse_error_body_retry_after() {
        let body = br#"{"ok":false,"error_code":429,"description":"Too Many Requests","parameters":{"retry_after":30}}"#;
        let (_, _, retry, _) = parse_error_body(body, 429);
        assert_eq!(retry, Some(30));
    }

    #[test]
    fn parse_error_body_invalid_json() {
        let body = b"<html>502 Bad Gateway</html>";
        let (msg, _, _, _) = parse_error_body(body, 502);
        assert!(msg.contains("Parsing the server response"), "got: {msg}");
    }

    #[test]
    fn parse_error_body_unknown_parameters() {
        let body = br#"{"ok":false,"description":"err","parameters":{"some_future_field":1}}"#;
        let (msg, _, _, extra) = parse_error_body(body, 400);
        assert_eq!(msg, "err");
        assert!(extra.is_some(), "expected extra params, got none");
    }

    // ------------------------------------------------------------------
    // http_status_phrase
    // ------------------------------------------------------------------

    #[test]
    fn known_status_codes() {
        assert!(http_status_phrase(400).contains("Bad Request"));
        assert!(http_status_phrase(403).contains("Forbidden"));
        assert!(http_status_phrase(409).contains("Conflict"));
        assert!(http_status_phrase(502).contains("Bad Gateway"));
    }

    #[test]
    fn unknown_status_code() {
        assert!(http_status_phrase(418).contains("Unknown HTTP Error"));
    }
}
