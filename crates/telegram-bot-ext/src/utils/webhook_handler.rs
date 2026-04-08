//! High-performance webhook HTTP server built on `axum`.
//!
//! Port of `telegram.ext._utils.webhookhandler`. Gated behind `feature = "webhooks"`.
//!
//! The server accepts POST requests on a configurable path, validates the
//! `Content-Type` and optional secret token, deserializes the Telegram
//! `Update` via zero-copy `serde_json::from_slice`, and pushes it into a
//! `tokio::sync::mpsc` channel using non-blocking `try_send` with
//! backpressure (503 when the channel is full).
//!
//! # Optimizations over a naive implementation
//!
//! - **Zero-copy deserialization**: reads `axum::body::Bytes` directly and
//!   calls `serde_json::from_slice` -- no intermediate `String` allocation.
//! - **Constant-time secret token comparison**: prevents timing side-channels.
//! - **Non-blocking channel send**: `try_send` returns 503 under backpressure
//!   rather than blocking the HTTP handler task.
//! - **TCP_NODELAY**: set on every accepted connection via `tap_io` for
//!   minimal latency (disables Nagle's algorithm).
//! - **Empty 200 OK responses**: Telegram ignores the body; we save bytes on
//!   the wire and an allocation.
//! - **Static health-check**: `/healthcheck` returns a pre-allocated response.
//! - **Structured tracing**: hot-path logging is `debug`-level only.
//! - **Graceful shutdown**: in-flight requests complete before the server
//!   exits.

#![cfg(feature = "webhooks")]

use std::net::SocketAddr;
use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::serve::ListenerExt;
use axum::Router;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Notify};
use tracing::{debug, error, info, warn};

use telegram_bot_raw::types::update::Update;

// ---------------------------------------------------------------------------
// Constant-time comparison
// ---------------------------------------------------------------------------

/// Compare two byte slices in constant time to prevent timing attacks.
///
/// Both slices are always compared in full regardless of where a mismatch
/// occurs.  Returns `true` only when the slices have equal length and
/// identical contents.
#[inline]
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    // XOR each byte pair and OR the results together. If any bit differs the
    // accumulator becomes non-zero.  The compiler cannot short-circuit this
    // because the fold processes every element unconditionally.
    let diff = a
        .iter()
        .zip(b.iter())
        .fold(0u8, |acc, (&x, &y)| acc | (x ^ y));
    diff == 0
}

// ---------------------------------------------------------------------------
// Pre-allocated static responses
// ---------------------------------------------------------------------------

/// Empty 200 OK -- Telegram does not read the response body.
#[inline(always)]
fn ok_response() -> Response {
    StatusCode::OK.into_response()
}

/// 403 Forbidden -- content-type or secret token mismatch.
#[inline(always)]
fn forbidden_response() -> Response {
    StatusCode::FORBIDDEN.into_response()
}

/// 400 Bad Request -- malformed JSON body.
#[inline(always)]
fn bad_request_response() -> Response {
    StatusCode::BAD_REQUEST.into_response()
}

/// 503 Service Unavailable -- channel full (backpressure).
#[inline(always)]
fn service_unavailable_response() -> Response {
    StatusCode::SERVICE_UNAVAILABLE.into_response()
}

/// 500 Internal Server Error -- channel closed.
#[inline(always)]
fn internal_error_response() -> Response {
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

// ---------------------------------------------------------------------------
// Shared state
// ---------------------------------------------------------------------------

/// Internal state shared across all request handlers.
#[derive(Debug, Clone)]
struct WebhookState {
    /// Channel through which parsed updates are forwarded.
    update_tx: mpsc::Sender<Update>,
    /// Pre-computed secret token bytes for constant-time comparison.
    /// `None` means no secret is required.
    secret_token: Option<Arc<[u8]>>,
}

// ---------------------------------------------------------------------------
// WebhookHandler (composable handler for custom axum routers)
// ---------------------------------------------------------------------------

/// A composable webhook handler that can be mounted into any `axum::Router`.
///
/// This is the low-level building block for custom webhook setups. For the
/// standard all-in-one server, use [`WebhookServer`] instead.
///
/// # Example
///
/// ```rust,ignore
/// use telegram_bot_ext::utils::webhook_handler::WebhookHandler;
/// use tokio::sync::mpsc;
///
/// let (tx, rx) = mpsc::channel(256);
/// let handler = WebhookHandler::new(tx, Some("my-secret".into()));
///
/// // Mount into your own axum router:
/// let app = handler.into_router("/webhook");
/// ```
#[derive(Debug, Clone)]
pub struct WebhookHandler {
    state: WebhookState,
}

impl WebhookHandler {
    /// Create a new webhook handler.
    ///
    /// - `update_tx`: bounded channel sender for forwarding parsed updates.
    /// - `secret_token`: if `Some`, every request must carry a matching
    ///   `X-Telegram-Bot-Api-Secret-Token` header.
    pub fn new(update_tx: mpsc::Sender<Update>, secret_token: Option<String>) -> Self {
        let secret_token = secret_token.map(|s| Arc::from(s.into_bytes().into_boxed_slice()));
        Self {
            state: WebhookState {
                update_tx,
                secret_token,
            },
        }
    }

    /// Build an `axum::Router` with the webhook POST handler mounted at
    /// the given path, plus a GET `/healthcheck` endpoint.
    ///
    /// The returned router can be merged into an existing axum application.
    pub fn into_router(self, url_path: &str) -> Router {
        let path = if url_path.starts_with('/') {
            url_path.to_owned()
        } else {
            format!("/{url_path}")
        };

        Router::new()
            .route(&path, post(handle_webhook))
            .route("/healthcheck", get(handle_healthcheck))
            .with_state(self.state)
    }
}

// ---------------------------------------------------------------------------
// WebhookServer
// ---------------------------------------------------------------------------

/// A thin wrapper around an `axum` HTTP server that receives Telegram webhook
/// POSTs and pushes deserialized updates into a channel.
///
/// For custom setups where you bring your own `axum::Router`, use
/// [`WebhookHandler`] directly.
#[derive(Debug)]
pub struct WebhookServer {
    /// Address the server will bind to.
    listen: String,
    port: u16,
    /// The axum `Router`.
    router: Router,
    /// Notified when `shutdown` is called.
    shutdown_notify: Arc<Notify>,
    /// Whether the server is currently running.
    running: std::sync::atomic::AtomicBool,
}

impl WebhookServer {
    /// Create a new webhook server. Does **not** start listening yet.
    ///
    /// - `listen`: IP address to bind to (e.g. `"0.0.0.0"`).
    /// - `port`: TCP port.
    /// - `url_path`: the URL path to mount the handler on (e.g. `"/webhook"`).
    /// - `update_tx`: channel sender for forwarding parsed updates.
    /// - `secret_token`: if `Some`, every request must carry a matching
    ///   `X-Telegram-Bot-Api-Secret-Token` header.
    pub fn new(
        listen: impl Into<String>,
        port: u16,
        url_path: &str,
        update_tx: mpsc::Sender<Update>,
        secret_token: Option<String>,
    ) -> Self {
        let handler = WebhookHandler::new(update_tx, secret_token);
        let router = handler.into_router(url_path);

        Self {
            listen: listen.into(),
            port,
            router,
            shutdown_notify: Arc::new(Notify::new()),
            running: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Whether the server is currently serving.
    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Start serving. Resolves when `ready` is notified (server is bound) or
    /// an error occurs.
    ///
    /// The server runs until [`shutdown`](Self::shutdown) is called. In-flight
    /// requests are drained before the server exits (graceful shutdown).
    pub async fn serve_forever(
        &self,
        ready: Option<Arc<Notify>>,
    ) -> Result<(), std::io::Error> {
        let addr: SocketAddr = format!("{}:{}", self.listen, self.port)
            .parse()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

        let listener = TcpListener::bind(addr).await?;

        // Use axum's ListenerExt::tap_io to set TCP_NODELAY on every accepted
        // connection. This disables Nagle's algorithm so small webhook
        // responses are sent immediately instead of being delayed.
        let listener = listener.tap_io(|tcp_stream| {
            if let Err(e) = tcp_stream.set_nodelay(true) {
                warn!("Failed to set TCP_NODELAY: {e}");
            }
        });

        self.running
            .store(true, std::sync::atomic::Ordering::Relaxed);

        info!("Webhook server started on {addr}");

        if let Some(n) = ready {
            n.notify_one();
        }

        let shutdown_notify = self.shutdown_notify.clone();
        axum::serve(listener, self.router.clone())
            .with_graceful_shutdown(async move {
                shutdown_notify.notified().await;
            })
            .await?;

        self.running
            .store(false, std::sync::atomic::Ordering::Relaxed);
        info!("Webhook server stopped");
        Ok(())
    }

    /// Signal the server to shut down gracefully.
    ///
    /// In-flight requests will be allowed to complete. New connections will
    /// be refused.
    pub fn shutdown(&self) {
        if self.is_running() {
            debug!("Shutting down webhook server");
            self.shutdown_notify.notify_one();
        }
    }
}

// ---------------------------------------------------------------------------
// axum handlers
// ---------------------------------------------------------------------------

/// The POST handler that receives Telegram updates.
///
/// Hot path -- every allocation here adds latency. The body is read as raw
/// `Bytes` and deserialized with `serde_json::from_slice` (zero-copy where
/// the Update type allows it). The channel send is non-blocking; if the
/// receiver cannot keep up we return 503 (backpressure) rather than blocking
/// the HTTP task.
async fn handle_webhook(
    State(state): State<WebhookState>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    // -- validate content type -----------------------------------------------
    let ct = headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !ct.starts_with("application/json") {
        debug!("Rejected request with Content-Type: {ct}");
        return forbidden_response();
    }

    // -- validate secret token (constant-time) -------------------------------
    if let Some(ref expected) = state.secret_token {
        let provided = headers
            .get("x-telegram-bot-api-secret-token")
            .map(|v| v.as_bytes());
        match provided {
            None => {
                debug!("Request missing secret token header");
                return forbidden_response();
            }
            Some(tok) if !constant_time_eq(tok, expected) => {
                debug!("Request had invalid secret token");
                return forbidden_response();
            }
            Some(_) => {}
        }
    }

    // -- zero-copy deserialization -------------------------------------------
    let update: Update = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to parse update JSON: {e}");
            return bad_request_response();
        }
    };

    debug!(update_id = update.update_id, "Webhook received update");

    // -- non-blocking channel send with backpressure -------------------------
    match state.update_tx.try_send(update) {
        Ok(()) => ok_response(),
        Err(mpsc::error::TrySendError::Full(_)) => {
            warn!("Update channel full -- applying backpressure (503)");
            service_unavailable_response()
        }
        Err(mpsc::error::TrySendError::Closed(_)) => {
            error!("Update channel closed");
            internal_error_response()
        }
    }
}

/// GET /healthcheck -- zero-allocation static response.
async fn handle_healthcheck() -> StatusCode {
    StatusCode::OK
}

// ---------------------------------------------------------------------------
// WebhookApp (convenience builder -- backward compat)
// ---------------------------------------------------------------------------

/// A convenience builder mirroring Python's `WebhookAppClass`.
pub struct WebhookApp;

impl WebhookApp {
    /// Create a [`WebhookServer`] from the standard set of parameters.
    pub fn new(
        listen: impl Into<String>,
        port: u16,
        url_path: &str,
        update_tx: mpsc::Sender<Update>,
        secret_token: Option<String>,
    ) -> WebhookServer {
        WebhookServer::new(listen, port, url_path, update_tx, secret_token)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- constant_time_eq tests ----------------------------------------------

    #[test]
    fn ct_eq_equal_slices() {
        assert!(constant_time_eq(b"hello", b"hello"));
    }

    #[test]
    fn ct_eq_different_slices() {
        assert!(!constant_time_eq(b"hello", b"world"));
    }

    #[test]
    fn ct_eq_different_lengths() {
        assert!(!constant_time_eq(b"short", b"longer"));
    }

    #[test]
    fn ct_eq_empty_slices() {
        assert!(constant_time_eq(b"", b""));
    }

    #[test]
    fn ct_eq_single_bit_diff() {
        // 'A' = 0x41, 'B' = 0x42 -- differ by one bit
        assert!(!constant_time_eq(b"A", b"B"));
    }

    // -- handler tests -------------------------------------------------------

    #[tokio::test]
    async fn rejects_wrong_content_type() {
        let (tx, _rx) = mpsc::channel(1);
        let state = WebhookState {
            update_tx: tx,
            secret_token: None,
        };
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "text/plain".parse().unwrap());

        let resp = handle_webhook(
            State(state),
            headers,
            Bytes::from_static(b"{}"),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn rejects_missing_secret_token() {
        let (tx, _rx) = mpsc::channel(1);
        let state = WebhookState {
            update_tx: tx,
            secret_token: Some(Arc::from(b"my-secret".to_vec().into_boxed_slice())),
        };
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        let resp = handle_webhook(
            State(state),
            headers,
            Bytes::from_static(b"{}"),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn rejects_wrong_secret_token() {
        let (tx, _rx) = mpsc::channel(1);
        let state = WebhookState {
            update_tx: tx,
            secret_token: Some(Arc::from(b"correct".to_vec().into_boxed_slice())),
        };
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());
        headers.insert(
            "x-telegram-bot-api-secret-token",
            "wrong".parse().unwrap(),
        );

        let resp = handle_webhook(
            State(state),
            headers,
            Bytes::from_static(b"{}"),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn accepts_valid_request() {
        let (tx, mut rx) = mpsc::channel(1);
        let state = WebhookState {
            update_tx: tx,
            secret_token: None,
        };
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        let resp = handle_webhook(
            State(state),
            headers,
            Bytes::from_static(b"{\"update_id\": 1}"),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);

        let update = rx.recv().await.unwrap();
        assert_eq!(update.update_id, 1);
    }

    #[tokio::test]
    async fn accepts_valid_request_with_secret() {
        let (tx, mut rx) = mpsc::channel(1);
        let state = WebhookState {
            update_tx: tx,
            secret_token: Some(Arc::from(b"mysecret".to_vec().into_boxed_slice())),
        };
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());
        headers.insert(
            "x-telegram-bot-api-secret-token",
            "mysecret".parse().unwrap(),
        );

        let resp = handle_webhook(
            State(state),
            headers,
            Bytes::from_static(b"{\"update_id\": 42}"),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);

        let update = rx.recv().await.unwrap();
        assert_eq!(update.update_id, 42);
    }

    #[tokio::test]
    async fn returns_503_when_channel_full() {
        // Channel capacity 1, pre-fill it.
        let (tx, _rx) = mpsc::channel(1);
        let prefill: Update = serde_json::from_str("{\"update_id\": 0}").unwrap();
        tx.try_send(prefill).unwrap();

        let state = WebhookState {
            update_tx: tx,
            secret_token: None,
        };
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        let resp = handle_webhook(
            State(state),
            headers,
            Bytes::from_static(b"{\"update_id\": 99}"),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn returns_500_when_channel_closed() {
        let (tx, rx) = mpsc::channel(1);
        drop(rx); // close the receiver

        let state = WebhookState {
            update_tx: tx,
            secret_token: None,
        };
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        let resp = handle_webhook(
            State(state),
            headers,
            Bytes::from_static(b"{\"update_id\": 1}"),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn returns_400_on_malformed_json() {
        let (tx, _rx) = mpsc::channel(1);
        let state = WebhookState {
            update_tx: tx,
            secret_token: None,
        };
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        let resp = handle_webhook(
            State(state),
            headers,
            Bytes::from_static(b"this is not json"),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn healthcheck_returns_200() {
        let status = handle_healthcheck().await;
        assert_eq!(status, StatusCode::OK);
    }

    // -- WebhookHandler tests ------------------------------------------------

    #[test]
    fn webhook_handler_creates_router() {
        let (tx, _rx) = mpsc::channel(1);
        let handler = WebhookHandler::new(tx, Some("secret".into()));
        let _router = handler.into_router("/webhook");
        // If we got here without panic, the router was built successfully.
    }

    #[test]
    fn webhook_handler_normalizes_path_without_slash() {
        let (tx, _rx) = mpsc::channel(1);
        let handler = WebhookHandler::new(tx, None);
        let _router = handler.into_router("webhook");
        // Path normalization succeeded (prepends '/').
    }
}
