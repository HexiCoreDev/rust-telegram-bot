//! [`reqwest`]-backed implementation of [`BaseRequest`].
//!
//! This mirrors `telegram.request.HTTPXRequest` from python-telegram-bot, which
//! uses `httpx` as its HTTP back-end.  Here we use `reqwest` instead but the
//! public contract and behaviour are identical.
//!
//! # Two-client design
//!
//! The Python implementation uses a single `httpx.AsyncClient` for both API
//! calls and file downloads.  We separate the two because file downloads may
//! be very large (hundreds of MB) and should never interfere with the
//! time-sensitive API call pool.  Each client has its own connection pool.
//!
//! # Timeouts
//!
//! `reqwest` supports per-request timeouts via
//! `reqwest::RequestBuilder::timeout`.  Since it does not expose separate
//! connect / read / write / pool timeouts the way `httpx` does, we use the
//! following mapping:
//!
//! | Python timeout | reqwest mapping |
//! |---|---|
//! | `connect_timeout` | `reqwest::ClientBuilder::connect_timeout` |
//! | `read_timeout` | per-request `timeout` (overall) |
//! | `write_timeout` | per-request `timeout` (overall) |
//! | `pool_timeout` | `reqwest::ClientBuilder::pool_idle_timeout` |
//!
//! The effective per-request timeout is `max(read_timeout, write_timeout)` so
//! that neither receiving nor sending is cut short prematurely.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::multipart::{Form, Part};
use reqwest::Client;
use tracing::{debug, warn};

use crate::error::{Result, TelegramError};

use super::base::{async_trait, BaseRequest, HttpMethod, TimeoutOverride};
use super::request_data::RequestData;

// ---------------------------------------------------------------------------
// Public constants
// ---------------------------------------------------------------------------

/// User-agent header value sent with every request.
pub const USER_AGENT_STRING: &str = concat!(
    "rust-telegram-bot/",
    env!("CARGO_PKG_VERSION"),
    " (https://github.com/nicegram/rust-telegram-bot)"
);

/// Default connection pool size (matches `HTTPXRequest` default of 256).
pub const DEFAULT_CONNECTION_POOL_SIZE: usize = 256;

/// Default read timeout — 5 seconds.
pub const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(5);
/// Default write timeout — 5 seconds.
pub const DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_secs(5);
/// Default connect timeout — 5 seconds.
pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
/// Default pool idle-connection timeout — 1 second.
pub const DEFAULT_POOL_TIMEOUT: Duration = Duration::from_secs(1);
/// Default media (large-file upload) write timeout — 20 seconds.
pub const DEFAULT_MEDIA_WRITE_TIMEOUT: Duration = Duration::from_secs(20);

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// Builder for [`ReqwestRequest`].
///
/// ```rust,no_run
/// # use telegram_bot_raw::request::reqwest_impl::ReqwestRequest;
/// # use std::time::Duration;
/// let req = ReqwestRequest::builder()
///     .connection_pool_size(128)
///     .read_timeout(Some(Duration::from_secs(10)))
///     .build()
///     .expect("valid configuration");
/// ```
#[derive(Debug, Clone)]
pub struct ReqwestRequestBuilder {
    connection_pool_size: usize,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    pool_timeout: Option<Duration>,
    media_write_timeout: Option<Duration>,
    /// Optional proxy URL applied to all requests (M6).
    proxy: Option<String>,
}

impl Default for ReqwestRequestBuilder {
    fn default() -> Self {
        Self {
            connection_pool_size: DEFAULT_CONNECTION_POOL_SIZE,
            read_timeout: Some(DEFAULT_READ_TIMEOUT),
            write_timeout: Some(DEFAULT_WRITE_TIMEOUT),
            connect_timeout: Some(DEFAULT_CONNECT_TIMEOUT),
            pool_timeout: Some(DEFAULT_POOL_TIMEOUT),
            media_write_timeout: Some(DEFAULT_MEDIA_WRITE_TIMEOUT),
            proxy: None,
        }
    }
}

impl ReqwestRequestBuilder {
    /// Maximum number of idle connections kept in the pool per host.
    pub fn connection_pool_size(mut self, size: usize) -> Self {
        self.connection_pool_size = size;
        self
    }

    /// Default read timeout (`None` = wait forever).
    pub fn read_timeout(mut self, t: Option<Duration>) -> Self {
        self.read_timeout = t;
        self
    }

    /// Default write timeout (`None` = wait forever).
    pub fn write_timeout(mut self, t: Option<Duration>) -> Self {
        self.write_timeout = t;
        self
    }

    /// Default connect timeout (`None` = wait forever).
    pub fn connect_timeout(mut self, t: Option<Duration>) -> Self {
        self.connect_timeout = t;
        self
    }

    /// Default pool (idle-connection) timeout (`None` = wait forever).
    pub fn pool_timeout(mut self, t: Option<Duration>) -> Self {
        self.pool_timeout = t;
        self
    }

    /// Write timeout used for large file uploads (`None` = wait forever).
    pub fn media_write_timeout(mut self, t: Option<Duration>) -> Self {
        self.media_write_timeout = t;
        self
    }

    /// Set a proxy URL (e.g. `socks5://127.0.0.1:1080` or
    /// `http://proxy.example.com:8080`).  The proxy is applied to both the API
    /// client and the file-download client.
    pub fn proxy(mut self, url: impl Into<String>) -> Self {
        self.proxy = Some(url.into());
        self
    }

    /// Consume the builder and produce a [`ReqwestRequest`].
    pub fn build(self) -> std::result::Result<ReqwestRequest, reqwest::Error> {
        let headers = {
            let mut h = HeaderMap::new();
            h.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_STRING));
            h
        };

        // API client — used for all Bot API JSON calls.
        let api_client = build_client(
            self.connection_pool_size,
            self.connect_timeout,
            self.pool_timeout,
            headers.clone(),
            self.proxy.as_deref(),
        )?;

        // File download client — separate pool, isolated from the API pool.
        let file_client = build_client(
            self.connection_pool_size,
            self.connect_timeout,
            self.pool_timeout,
            headers,
            self.proxy.as_deref(),
        )?;

        Ok(ReqwestRequest {
            api_client,
            file_client,
            defaults: Arc::new(DefaultTimeouts {
                read: self.read_timeout,
                write: self.write_timeout,
                connect: self.connect_timeout,
                pool: self.pool_timeout,
                media_write: self.media_write_timeout,
            }),
            initialized: Arc::new(AtomicBool::new(false)),
        })
    }
}

// ---------------------------------------------------------------------------
// Internal helper: build a single reqwest::Client
// ---------------------------------------------------------------------------

fn build_client(
    pool_size: usize,
    connect_timeout: Option<Duration>,
    pool_idle_timeout: Option<Duration>,
    default_headers: HeaderMap,
    proxy_url: Option<&str>,
) -> std::result::Result<Client, reqwest::Error> {
    let mut builder = Client::builder()
        .default_headers(default_headers)
        .pool_max_idle_per_host(pool_size);

    if let Some(ct) = connect_timeout {
        builder = builder.connect_timeout(ct);
    }

    if let Some(pit) = pool_idle_timeout {
        builder = builder.pool_idle_timeout(pit);
    }

    // M6: apply proxy if configured.
    if let Some(url) = proxy_url {
        let proxy = reqwest::Proxy::all(url)?;
        builder = builder.proxy(proxy);
    }

    builder.build()
}

// ---------------------------------------------------------------------------
// Default timeouts store
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
struct DefaultTimeouts {
    read: Option<Duration>,
    write: Option<Duration>,
    connect: Option<Duration>,
    pool: Option<Duration>,
    media_write: Option<Duration>,
}

/// Concrete timeout values after applying caller overrides on top of defaults.
#[derive(Debug, Clone, Copy)]
struct ResolvedTimeouts {
    read: Option<Duration>,
    write: Option<Duration>,
    /// Kept for documentation purposes; not currently applied per-request since
    /// reqwest applies it at client construction time.
    #[allow(dead_code)]
    connect: Option<Duration>,
    /// Kept for documentation purposes; reqwest applies this at construction.
    #[allow(dead_code)]
    pool: Option<Duration>,
}

impl DefaultTimeouts {
    /// Resolve caller overrides against these defaults.
    ///
    /// - `Some(Some(d))` = caller explicitly set `d`.
    /// - `Some(None)` = caller explicitly set "no timeout".
    /// - `None` = caller did not specify (use our default).
    fn resolve(&self, override_: TimeoutOverride, has_files: bool) -> ResolvedTimeouts {
        let write = match override_.write {
            Some(v) => v,
            None => {
                if has_files {
                    self.media_write
                } else {
                    self.write
                }
            }
        };

        ResolvedTimeouts {
            read: override_.read.unwrap_or(self.read),
            write,
            connect: override_.connect.unwrap_or(self.connect),
            pool: override_.pool.unwrap_or(self.pool),
        }
    }
}

// ---------------------------------------------------------------------------
// ReqwestRequest
// ---------------------------------------------------------------------------

/// `reqwest`-backed implementation of [`BaseRequest`].
///
/// Construct via [`ReqwestRequest::builder()`] or [`ReqwestRequest::new()`]
/// for sensible defaults.
///
/// This type is `Clone` — cloning shares the same underlying connection pools.
#[derive(Clone)]
pub struct ReqwestRequest {
    /// Client used for all Bot API method calls.
    api_client: Client,
    /// Client used for file downloads ([`BaseRequest::retrieve`]).
    file_client: Client,
    /// Default timeout configuration.
    defaults: Arc<DefaultTimeouts>,
    /// Whether `initialize()` has been called at least once.
    initialized: Arc<AtomicBool>,
}

impl std::fmt::Debug for ReqwestRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReqwestRequest")
            .field("defaults", &self.defaults)
            .field("initialized", &self.initialized.load(Ordering::Relaxed))
            .finish_non_exhaustive()
    }
}

impl ReqwestRequest {
    /// Create a builder to customise the client.
    pub fn builder() -> ReqwestRequestBuilder {
        ReqwestRequestBuilder::default()
    }

    /// Create a [`ReqwestRequest`] with all default settings.
    pub fn new() -> std::result::Result<Self, reqwest::Error> {
        Self::builder().build()
    }

    /// `true` after [`BaseRequest::initialize`] has been called.
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::Relaxed)
    }
}

// ---------------------------------------------------------------------------
// BaseRequest implementation
// ---------------------------------------------------------------------------

#[async_trait]
impl BaseRequest for ReqwestRequest {
    async fn initialize(&self) -> Result<()> {
        // reqwest::Client is ready immediately after construction.  We record
        // the fact so that callers who check `is_initialized()` get a
        // meaningful answer.
        self.initialized.store(true, Ordering::Relaxed);
        debug!("ReqwestRequest initialised");
        Ok(())
    }

    async fn shutdown(&self) -> Result<()> {
        if !self.initialized.load(Ordering::Relaxed) {
            debug!("ReqwestRequest.shutdown called but already shut down — returning");
            return Ok(());
        }
        // reqwest manages its own connection-pool lifecycle; there is no
        // explicit close call.  We just mark the instance as shut down.
        self.initialized.store(false, Ordering::Relaxed);
        debug!("ReqwestRequest shut down");
        Ok(())
    }

    fn default_read_timeout(&self) -> Option<Duration> {
        self.defaults.read
    }

    async fn do_request(
        &self,
        url: &str,
        method: HttpMethod,
        request_data: Option<&RequestData>,
        timeouts: TimeoutOverride,
    ) -> Result<(u16, Bytes)> {
        let has_files = request_data.is_some_and(RequestData::contains_files);
        let resolved = self.defaults.resolve(timeouts, has_files);

        // Select the correct client: file downloads go through file_client to
        // keep them isolated from the API call pool.
        let client = match method {
            HttpMethod::Get => &self.file_client,
            HttpMethod::Post => &self.api_client,
        };

        // Build the reqwest request.
        let mut req_builder = match method {
            HttpMethod::Post => client.post(url),
            HttpMethod::Get => client.get(url),
        };

        // Apply the effective per-request timeout.
        // We use the max of read and write so neither operation is cut short.
        let effective_timeout = max_duration(resolved.read, resolved.write);
        if let Some(t) = effective_timeout {
            req_builder = req_builder.timeout(t);
        }

        // Attach body.
        req_builder = match request_data {
            None => req_builder,
            Some(data) if data.contains_files() => {
                let form = build_multipart_form(data)?;
                req_builder.multipart(form)
            }
            Some(data) => {
                // JSON parameters sent as `application/x-www-form-urlencoded`
                // body — matches what httpx sends when `data=` is passed.
                let params = data.json_parameters();
                req_builder.form(&params)
            }
        };

        // Execute.
        let response = req_builder.send().await.map_err(map_reqwest_error)?;

        let status = response.status().as_u16();
        let body = response
            .bytes()
            .await
            .map_err(|e| TelegramError::Network(format!("Failed to read response body: {e}")))?;

        Ok((status, body))
    }

    async fn do_request_json_bytes(
        &self,
        url: &str,
        body: &[u8],
        timeouts: TimeoutOverride,
    ) -> Result<(u16, Bytes)> {
        // Text-only requests never carry files, so use normal write timeout.
        let resolved = self.defaults.resolve(timeouts, false);

        let mut req_builder = self
            .api_client
            .post(url)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(body.to_vec());

        let effective_timeout = max_duration(resolved.read, resolved.write);
        if let Some(t) = effective_timeout {
            req_builder = req_builder.timeout(t);
        }

        let response = req_builder.send().await.map_err(map_reqwest_error)?;

        let status = response.status().as_u16();
        let resp_body = response
            .bytes()
            .await
            .map_err(|e| TelegramError::Network(format!("Failed to read response body: {e}")))?;

        Ok((status, resp_body))
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Convert a `reqwest::Error` into an appropriate [`TelegramError`].
///
/// Mirrors the httpx error-mapping in `HTTPXRequest.do_request`:
/// - Timeouts → [`TelegramError::TimedOut`]
/// - Everything else → [`TelegramError::Network`]
fn map_reqwest_error(e: reqwest::Error) -> TelegramError {
    if e.is_timeout() || e.is_connect() {
        // reqwest surfaces both timeout-during-connect and read-timeout as
        // `is_timeout()`.  Pool-exhaustion manifests as a connect error with
        // an "operation timed out" message.
        let msg = if e.is_timeout() {
            format!("Request timed out: {e}")
        } else {
            format!("Connection error: {e}")
        };
        warn!("{msg}");
        TelegramError::TimedOut(msg)
    } else {
        let msg = format!("reqwest error: {e}");
        warn!("{msg}");
        TelegramError::Network(msg)
    }
}

/// Build a `reqwest` multipart form from a [`RequestData`].
fn build_multipart_form(data: &RequestData) -> Result<Form> {
    let parts = data
        .multipart_data()
        .expect("called only when contains_files() is true");

    let mut form = Form::new();

    // Add all file parts first.
    for (part_name, multipart_part) in &parts {
        let bytes = multipart_part.bytes.clone();
        let mut part = Part::bytes(bytes)
            .mime_str(&multipart_part.mime_type)
            .map_err(|e| {
                TelegramError::Network(format!(
                    "Invalid MIME type '{}': {e}",
                    multipart_part.mime_type
                ))
            })?;

        if let Some(ref fname) = multipart_part.file_name {
            part = part.file_name(fname.clone());
        }

        form = form.part(part_name.clone(), part);
    }

    // Add all non-file (JSON) parameters as text parts.
    for (name, value) in data.json_parameters() {
        form = form.text(name, value);
    }

    Ok(form)
}

/// Return the larger of two `Option<Duration>` values.
///
/// - `(None, None)` → `None`
/// - `(Some(a), None)` → `Some(a)`
/// - `(None, Some(b))` → `Some(b)`
/// - `(Some(a), Some(b))` → `Some(max(a, b))`
fn max_duration(a: Option<Duration>, b: Option<Duration>) -> Option<Duration> {
    match (a, b) {
        (None, None) => None,
        (Some(v), None) | (None, Some(v)) => Some(v),
        (Some(x), Some(y)) => Some(x.max(y)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------
    // Builder / construction
    // ------------------------------------------------------------------

    #[test]
    fn builder_defaults_produce_valid_client() {
        ReqwestRequest::new().expect("default client construction must succeed");
    }

    #[test]
    fn builder_custom_pool_size() {
        ReqwestRequest::builder()
            .connection_pool_size(4)
            .build()
            .expect("small pool size should be valid");
    }

    #[test]
    fn builder_with_proxy() {
        // Constructing with a proxy URL must not panic.  We cannot actually
        // verify the proxy is applied without a live server, but the builder
        // path must succeed.
        let req = ReqwestRequest::builder()
            .proxy("http://127.0.0.1:8080")
            .build()
            .expect("proxy builder should succeed");
        assert!(!req.is_initialized());
    }

    // ------------------------------------------------------------------
    // initialize / shutdown
    // ------------------------------------------------------------------

    #[tokio::test]
    async fn initialize_sets_initialized_flag() {
        let req = ReqwestRequest::new().unwrap();
        assert!(!req.is_initialized());
        req.initialize().await.unwrap();
        assert!(req.is_initialized());
    }

    #[tokio::test]
    async fn shutdown_clears_initialized_flag() {
        let req = ReqwestRequest::new().unwrap();
        req.initialize().await.unwrap();
        req.shutdown().await.unwrap();
        assert!(!req.is_initialized());
    }

    #[tokio::test]
    async fn shutdown_idempotent() {
        let req = ReqwestRequest::new().unwrap();
        // Not yet initialized — should return Ok without panicking.
        req.shutdown().await.unwrap();
        req.shutdown().await.unwrap();
    }

    // ------------------------------------------------------------------
    // default_read_timeout
    // ------------------------------------------------------------------

    #[test]
    fn default_read_timeout_matches_builder() {
        let req = ReqwestRequest::builder()
            .read_timeout(Some(Duration::from_secs(99)))
            .build()
            .unwrap();
        assert_eq!(req.default_read_timeout(), Some(Duration::from_secs(99)));
    }

    #[test]
    fn default_read_timeout_none_when_unset() {
        let req = ReqwestRequest::builder()
            .read_timeout(None)
            .build()
            .unwrap();
        assert_eq!(req.default_read_timeout(), None);
    }

    // ------------------------------------------------------------------
    // max_duration helper
    // ------------------------------------------------------------------

    #[test]
    fn max_duration_both_none() {
        assert_eq!(max_duration(None, None), None);
    }

    #[test]
    fn max_duration_left_some() {
        let d = Duration::from_secs(5);
        assert_eq!(max_duration(Some(d), None), Some(d));
    }

    #[test]
    fn max_duration_right_some() {
        let d = Duration::from_secs(3);
        assert_eq!(max_duration(None, Some(d)), Some(d));
    }

    #[test]
    fn max_duration_returns_larger() {
        let a = Duration::from_secs(5);
        let b = Duration::from_secs(20);
        assert_eq!(max_duration(Some(a), Some(b)), Some(b));
        assert_eq!(max_duration(Some(b), Some(a)), Some(b));
    }

    // ------------------------------------------------------------------
    // DefaultTimeouts::resolve
    // ------------------------------------------------------------------

    #[test]
    fn resolve_uses_defaults_when_no_overrides() {
        let defaults = DefaultTimeouts {
            read: Some(Duration::from_secs(5)),
            write: Some(Duration::from_secs(5)),
            connect: Some(Duration::from_secs(5)),
            pool: Some(Duration::from_secs(1)),
            media_write: Some(Duration::from_secs(20)),
        };
        let resolved = defaults.resolve(TimeoutOverride::default_none(), false);
        assert_eq!(resolved.read, Some(Duration::from_secs(5)));
        assert_eq!(resolved.write, Some(Duration::from_secs(5)));
    }

    #[test]
    fn resolve_uses_media_write_timeout_when_has_files() {
        let defaults = DefaultTimeouts {
            read: Some(Duration::from_secs(5)),
            write: Some(Duration::from_secs(5)),
            connect: Some(Duration::from_secs(5)),
            pool: Some(Duration::from_secs(1)),
            media_write: Some(Duration::from_secs(20)),
        };
        let resolved = defaults.resolve(TimeoutOverride::default_none(), true);
        assert_eq!(resolved.write, Some(Duration::from_secs(20)));
    }

    #[test]
    fn resolve_caller_override_takes_precedence() {
        let defaults = DefaultTimeouts {
            read: Some(Duration::from_secs(5)),
            write: Some(Duration::from_secs(5)),
            connect: Some(Duration::from_secs(5)),
            pool: Some(Duration::from_secs(1)),
            media_write: Some(Duration::from_secs(20)),
        };
        let overrides = TimeoutOverride {
            read: Some(Some(Duration::from_secs(30))),
            write: Some(None), // explicit "no timeout"
            ..TimeoutOverride::default_none()
        };
        let resolved = defaults.resolve(overrides, false);
        assert_eq!(resolved.read, Some(Duration::from_secs(30)));
        assert_eq!(resolved.write, None);
    }

    #[test]
    fn resolve_explicit_none_overrides_media_timeout_even_with_files() {
        let defaults = DefaultTimeouts {
            read: Some(Duration::from_secs(5)),
            write: Some(Duration::from_secs(5)),
            connect: Some(Duration::from_secs(5)),
            pool: Some(Duration::from_secs(1)),
            media_write: Some(Duration::from_secs(20)),
        };
        // Caller says: no write timeout for this particular upload.
        let overrides = TimeoutOverride {
            write: Some(None),
            ..TimeoutOverride::default_none()
        };
        let resolved = defaults.resolve(overrides, true);
        assert_eq!(
            resolved.write, None,
            "explicit None must win over media_write"
        );
    }
}
