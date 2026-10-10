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
//! # TLS support
//!
//! When the `webhooks-tls` feature is enabled you can pass a [`TlsConfig`]
//! to [`WebhookServer`].  The server will then accept connections through
//! `tokio-rustls` instead of plain TCP.  Without the feature flag the TLS
//! code is compiled out entirely (zero cost).
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
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::serve::ListenerExt;
use axum::Router;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Notify};
use tracing::{debug, error, info, warn};

use rust_tg_bot_raw::types::update::Update;

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

/// 413 Payload Too Large -- body exceeds [`MAX_WEBHOOK_BODY_BYTES`].
#[inline(always)]
fn payload_too_large_response() -> Response {
    StatusCode::PAYLOAD_TOO_LARGE.into_response()
}

/// Maximum accepted webhook body size (1 MiB).
///
/// Telegram update payloads are JSON and far smaller than this (tens of KiB
/// at most). The cap bounds memory per request; axum's own default body
/// limit (2 MiB) applies first at the extractor level.
pub const MAX_WEBHOOK_BODY_BYTES: usize = 1024 * 1024;

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
// TLS configuration (feature-gated)
// ---------------------------------------------------------------------------

/// TLS configuration for the webhook server.
///
/// Wraps a `tokio_rustls::TlsAcceptor` loaded from PEM certificate and key
/// files.  Only available with the `webhooks-tls` feature.
#[cfg(feature = "webhooks-tls")]
#[derive(Clone)]
pub struct TlsConfig {
    acceptor: tokio_rustls::TlsAcceptor,
}

#[cfg(feature = "webhooks-tls")]
impl std::fmt::Debug for TlsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TlsConfig")
            .field("acceptor", &"TlsAcceptor { .. }")
            .finish()
    }
}

#[cfg(feature = "webhooks-tls")]
impl TlsConfig {
    /// Load TLS configuration from PEM-encoded certificate and private key
    /// files.
    ///
    /// The certificate file may contain the full chain (leaf first).  The key
    /// file must contain exactly one PKCS#8, SEC1 (EC), or PKCS#1 (RSA)
    /// private key.
    pub async fn from_pem_files(cert_path: &str, key_path: &str) -> Result<Self, std::io::Error> {
        use rustls_pemfile::{certs, private_key};
        use std::io::{self, BufReader};
        use tokio_rustls::rustls::ServerConfig;

        let cert_data = tokio::fs::read(cert_path).await.map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("failed to read cert file '{cert_path}': {e}"),
            )
        })?;
        let key_data = tokio::fs::read(key_path).await.map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("failed to read key file '{key_path}': {e}"),
            )
        })?;

        // Parse certificates.
        let certs: Vec<_> = certs(&mut BufReader::new(cert_data.as_slice()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("invalid cert PEM: {e}"))
            })?;

        if certs.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("no certificates found in '{cert_path}'"),
            ));
        }

        // Parse private key -- take the first key found.
        let key = private_key(&mut BufReader::new(key_data.as_slice()))
            .map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("invalid key PEM: {e}"))
            })?
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("no private key found in '{key_path}'"),
                )
            })?;

        let server_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("TLS config error: {e}"))
            })?;

        Ok(Self {
            acceptor: tokio_rustls::TlsAcceptor::from(Arc::new(server_config)),
        })
    }
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
/// use rust_tg_bot_ext::utils::webhook_handler::WebhookHandler;
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
    ///   `X-Telegram-Bot-Api-Secret-Token` header. An empty string is treated
    ///   as `None` (no secret required), matching python-telegram-bot's
    ///   truthiness check on the same parameter.
    ///
    /// # Unauthenticated endpoints
    ///
    /// Both `None` and `Some("")` leave the endpoint accepting *any* POST that
    /// parses as an `Update`, from anyone who can reach the URL. An empty
    /// string is nearly always a misconfiguration -- an unset environment
    /// variable or a missing CLI flag expanding to `""` -- so it is logged at
    /// `error!` rather than quietly downgrading. Both cases log at `warn!`
    /// again when the server starts, so an operator who believes the endpoint
    /// is protected sees it plainly at boot.
    pub fn new(update_tx: mpsc::Sender<Update>, secret_token: Option<String>) -> Self {
        let secret_token = secret_token
            .filter(|s| {
                let keep = !s.is_empty();
                if !keep {
                    error!(
                        "Webhook secret token is EMPTY -- this endpoint will accept \
                         unauthenticated updates from anyone who can reach it. \
                         Set a non-empty secret_token, or pass None to accept this \
                         deliberately."
                    );
                }
                keep
            })
            .map(|s| Arc::from(s.into_bytes().into_boxed_slice()));
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
    /// Optional TLS acceptor. Present only when the `webhooks-tls` feature is
    /// enabled and a [`TlsConfig`] was provided at construction time.
    #[cfg(feature = "webhooks-tls")]
    tls: Option<TlsConfig>,
    /// Whether a secret token was configured. `false` means the endpoint
    /// accepts unauthenticated updates, which is reported at startup.
    authenticated: bool,
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
    /// - `tls`: optional TLS configuration (only with `webhooks-tls` feature).
    pub fn new(
        listen: impl Into<String>,
        port: u16,
        url_path: &str,
        update_tx: mpsc::Sender<Update>,
        secret_token: Option<String>,
        #[cfg(feature = "webhooks-tls")] tls: Option<TlsConfig>,
    ) -> Self {
        let handler = WebhookHandler::new(update_tx, secret_token);
        let authenticated = handler.state.secret_token.is_some();
        let router = handler.into_router(url_path);

        Self {
            listen: listen.into(),
            port,
            router,
            shutdown_notify: Arc::new(Notify::new()),
            running: std::sync::atomic::AtomicBool::new(false),
            authenticated,
            #[cfg(feature = "webhooks-tls")]
            tls,
        }
    }

    /// Whether the server is currently serving.
    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Start serving. Resolves when `ready` is notified (server is bound) or
    /// an error occurs.
    ///
    /// The server runs until `shutdown` is called. In-flight
    /// requests are drained before the server exits (graceful shutdown).
    ///
    /// When TLS is configured (via the `webhooks-tls` feature and a
    /// [`TlsConfig`]), the server accepts HTTPS connections. Otherwise it
    /// serves plain HTTP.
    pub async fn serve_forever(&self, ready: Option<Arc<Notify>>) -> Result<(), std::io::Error> {
        let addr: SocketAddr = format!("{}:{}", self.listen, self.port)
            .parse()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

        let listener = TcpListener::bind(addr).await?;

        self.running
            .store(true, std::sync::atomic::Ordering::Relaxed);

        // Dispatch to TLS or plain-text serving.
        #[cfg(feature = "webhooks-tls")]
        if let Some(ref tls) = self.tls {
            info!("Webhook server (HTTPS) started on {addr}");
            self.warn_if_unauthenticated();
            if let Some(n) = ready {
                n.notify_one();
            }
            return self.serve_tls(listener, tls.clone(), addr).await;
        }

        // Plain HTTP path (existing behavior).
        let listener = listener.tap_io(|tcp_stream| {
            if let Err(e) = tcp_stream.set_nodelay(true) {
                warn!("Failed to set TCP_NODELAY: {e}");
            }
        });

        info!("Webhook server started on {addr}");
        self.warn_if_unauthenticated();

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

    /// Serve over TLS using `tokio-rustls`.
    ///
    /// Manually accepts TCP connections, performs the TLS handshake, wraps each
    /// connection with `hyper_util::rt::TokioIo`, and serves HTTP/1.1 through
    /// `hyper_util::server::conn::auto::Builder`. The axum `Router` is cloned
    /// per connection (cheap -- internally `Arc`-wrapped) and adapted via
    /// `hyper_util::service::TowerToHyperService` for hyper compatibility.
    ///
    /// Graceful shutdown is handled via `tokio_util::sync::CancellationToken`:
    /// in-flight connections are allowed to finish while no new connections are
    /// accepted.
    #[cfg(feature = "webhooks-tls")]
    async fn serve_tls(
        &self,
        listener: TcpListener,
        tls: TlsConfig,
        addr: SocketAddr,
    ) -> Result<(), std::io::Error> {
        use hyper_util::service::TowerToHyperService;

        let shutdown_notify = self.shutdown_notify.clone();
        let router = self.router.clone();

        let graceful = tokio_util::sync::CancellationToken::new();
        let graceful_for_shutdown = graceful.clone();

        // Spawn a task that waits for the shutdown signal.
        tokio::spawn(async move {
            shutdown_notify.notified().await;
            graceful_for_shutdown.cancel();
        });

        let mut connection_handles = tokio::task::JoinSet::new();

        loop {
            tokio::select! {
                _ = graceful.cancelled() => {
                    debug!("TLS server shutting down, waiting for in-flight connections");
                    break;
                }
                accepted = listener.accept() => {
                    let (tcp_stream, remote_addr) = match accepted {
                        Ok(conn) => conn,
                        Err(e) => {
                            error!("Failed to accept TCP connection: {e}");
                            continue;
                        }
                    };

                    // Set TCP_NODELAY.
                    if let Err(e) = tcp_stream.set_nodelay(true) {
                        warn!("Failed to set TCP_NODELAY: {e}");
                    }

                    let acceptor = tls.acceptor.clone();
                    let token = graceful.clone();
                    // Router is cheap to clone (Arc internally).
                    let svc = TowerToHyperService::new(router.clone());

                    connection_handles.spawn(async move {
                        // Perform TLS handshake.
                        let tls_stream = match acceptor.accept(tcp_stream).await {
                            Ok(s) => s,
                            Err(e) => {
                                debug!("TLS handshake failed from {remote_addr}: {e}");
                                return;
                            }
                        };

                        // Wrap the TLS stream for hyper-util.
                        let io = hyper_util::rt::TokioIo::new(tls_stream);

                        let builder = hyper_util::server::conn::auto::Builder::new(
                            hyper_util::rt::TokioExecutor::new(),
                        );
                        let conn = builder.serve_connection(io, svc);

                        // Pin the connection future for graceful shutdown.
                        let mut conn = std::pin::pin!(conn);

                        tokio::select! {
                            result = conn.as_mut() => {
                                if let Err(e) = result {
                                    debug!("Connection error from {remote_addr}: {e}");
                                }
                            }
                            _ = token.cancelled() => {
                                // Graceful shutdown: signal the connection and
                                // wait for it to drain.
                                conn.as_mut().graceful_shutdown();
                                if let Err(e) = conn.await {
                                    debug!("Connection error during shutdown from {remote_addr}: {e}");
                                }
                            }
                        }
                    });
                }
            }
        }

        // Wait for all in-flight connections to complete.
        while connection_handles.join_next().await.is_some() {}

        self.running
            .store(false, std::sync::atomic::Ordering::Relaxed);
        info!("Webhook server (HTTPS) stopped on {addr}");
        Ok(())
    }

    /// Warn at startup if the endpoint accepts unauthenticated updates.
    ///
    /// An operator who believes their webhook is protected but passed `None`
    /// (or an empty string) should not have to read the source to find out.
    fn warn_if_unauthenticated(&self) {
        if self.authenticated {
            return;
        }
        warn!(
            "Webhook server is running WITHOUT a secret token -- it will accept and \
             dispatch any well-formed Update posted by anyone who can reach {addr:?}. \
             Set secret_token to the value registered with Telegram's setWebhook.",
            addr = format_args!("{}:{}", self.listen, self.port),
        );
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

/// Header-validation guard that must be listed *before* the body extractor.
///
/// This is a `FromRequestParts` extractor, which axum runs to completion before
/// it constructs the final body extractor, and returns on the first rejection.
/// So an unauthenticated request never reaches the `Bytes` extractor and its
/// body is never read off the socket or buffered in memory.
///
/// Validating inside the handler body instead would buffer the whole request
/// first, letting any unauthenticated caller make the server allocate up to
/// axum's body limit before the secret token was ever checked.
struct Authenticated;

impl FromRequestParts<WebhookState> for Authenticated {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &WebhookState,
    ) -> Result<Self, Self::Rejection> {
        validate_headers(&parts.headers, state)?;
        Ok(Authenticated)
    }
}

/// Validate `Content-Type` and the secret token. Returns `403` on any mismatch.
fn validate_headers(headers: &HeaderMap, state: &WebhookState) -> Result<(), StatusCode> {
    let ct = headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !ct.starts_with("application/json") {
        debug!("Rejected request with Content-Type: {ct}");
        return Err(StatusCode::FORBIDDEN);
    }

    if let Some(ref expected) = state.secret_token {
        let provided = headers
            .get("x-telegram-bot-api-secret-token")
            .map(|v| v.as_bytes());
        match provided {
            None => {
                debug!("Request missing secret token header");
                return Err(StatusCode::FORBIDDEN);
            }
            Some(tok) if !constant_time_eq(tok, expected) => {
                debug!("Request had invalid secret token");
                return Err(StatusCode::FORBIDDEN);
            }
            Some(_) => {}
        }
    }

    Ok(())
}

/// The POST handler that receives Telegram updates.
///
/// Authentication happens in the [`Authenticated`] extractor; this function
/// only runs once the caller has already proven they hold the secret token,
/// so by the time the body exists it is a body we are willing to parse.
///
/// `authenticated` is declared before `body` on purpose -- see [`Authenticated`].
async fn handle_webhook(
    State(state): State<WebhookState>,
    _authenticated: Authenticated,
    body: Bytes,
) -> Response {
    ingest_update(state, body)
}

/// Deserialize an already-authenticated update body and forward it.
///
/// Hot path -- every allocation here adds latency. The body is deserialized
/// with `serde_json::from_slice` (zero-copy where the `Update` type allows it).
/// The channel send is non-blocking; if the receiver cannot keep up we return
/// 503 (backpressure) rather than blocking the HTTP task.
fn ingest_update(state: WebhookState, body: Bytes) -> Response {
    // -- bound body size -------------------------------------------------------
    if body.len() > MAX_WEBHOOK_BODY_BYTES {
        warn!("Rejected oversized webhook body ({} bytes)", body.len());
        return payload_too_large_response();
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
    // Deliberately named `new` for parity with PTB's `WebhookAppClass`, even
    // though it returns a `WebhookServer` rather than `Self`.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        listen: impl Into<String>,
        port: u16,
        url_path: &str,
        update_tx: mpsc::Sender<Update>,
        secret_token: Option<String>,
    ) -> WebhookServer {
        WebhookServer::new(
            listen,
            port,
            url_path,
            update_tx,
            secret_token,
            #[cfg(feature = "webhooks-tls")]
            None,
        )
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

    #[test]
    fn rejects_wrong_content_type() {
        let (tx, _rx) = mpsc::channel(1);
        let state = WebhookState {
            update_tx: tx,
            secret_token: None,
        };
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "text/plain".parse().unwrap());

        assert_eq!(
            validate_headers(&headers, &state),
            Err(StatusCode::FORBIDDEN)
        );
    }

    #[test]
    fn rejects_missing_secret_token() {
        let (tx, _rx) = mpsc::channel(1);
        let state = WebhookState {
            update_tx: tx,
            secret_token: Some(Arc::from(b"my-secret".to_vec().into_boxed_slice())),
        };
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        assert_eq!(
            validate_headers(&headers, &state),
            Err(StatusCode::FORBIDDEN)
        );
    }

    #[test]
    fn rejects_wrong_secret_token() {
        let (tx, _rx) = mpsc::channel(1);
        let state = WebhookState {
            update_tx: tx,
            secret_token: Some(Arc::from(b"correct".to_vec().into_boxed_slice())),
        };
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());
        headers.insert("x-telegram-bot-api-secret-token", "wrong".parse().unwrap());

        assert_eq!(
            validate_headers(&headers, &state),
            Err(StatusCode::FORBIDDEN)
        );
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

        let resp = ingest_update(state, Bytes::from_static(b"{\"update_id\": 1}"));
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

        let resp = ingest_update(state, Bytes::from_static(b"{\"update_id\": 42}"));
        assert_eq!(resp.status(), StatusCode::OK);

        let update = rx.recv().await.unwrap();
        assert_eq!(update.update_id, 42);
    }

    #[test]
    fn returns_503_when_channel_full() {
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

        let resp = ingest_update(state, Bytes::from_static(b"{\"update_id\": 99}"));
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    fn returns_500_when_channel_closed() {
        let (tx, rx) = mpsc::channel(1);
        drop(rx); // close the receiver

        let state = WebhookState {
            update_tx: tx,
            secret_token: None,
        };
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        let resp = ingest_update(state, Bytes::from_static(b"{\"update_id\": 1}"));
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn rejects_oversized_body() {
        let (tx, _rx) = mpsc::channel(1);
        let state = WebhookState {
            update_tx: tx,
            secret_token: None,
        };
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        let big = Bytes::from(vec![b' '; MAX_WEBHOOK_BODY_BYTES + 1]);
        let resp = ingest_update(state, big);
        assert_eq!(resp.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[test]
    fn empty_secret_string_means_no_secret() {
        let (tx, _rx) = mpsc::channel(1);
        let handler = WebhookHandler::new(tx, Some(String::new()));
        assert!(
            handler.state.secret_token.is_none(),
            "empty secret must downgrade to no-secret mode"
        );
    }

    #[test]
    fn non_empty_secret_is_kept() {
        let (tx, _rx) = mpsc::channel(1);
        let handler = WebhookHandler::new(tx, Some("s3cret".to_owned()));
        assert_eq!(
            handler.state.secret_token.as_deref(),
            Some(b"s3cret".as_slice())
        );
    }

    #[test]
    fn returns_400_on_malformed_json() {
        let (tx, _rx) = mpsc::channel(1);
        let state = WebhookState {
            update_tx: tx,
            secret_token: None,
        };
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        let resp = ingest_update(state, Bytes::from_static(b"this is not json"));
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

    // -- auth-before-body ordering ---------------------------------------------

    /// Start the webhook router on an ephemeral port and return its address.
    async fn spawn_test_server(secret: Option<&str>) -> SocketAddr {
        let (tx, _rx) = mpsc::channel(4);
        let router = WebhookHandler::new(tx, secret.map(str::to_owned)).into_router("/webhook");
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("local_addr");
        tokio::spawn(async move {
            let _ = axum::serve(listener, router).await;
        });
        addr
    }

    /// The whole point of moving auth into an extractor: a caller who fails the
    /// secret-token check must be rejected *without* the server first reading
    /// and buffering their body.
    ///
    /// The probe announces a 4 MiB `Content-Length` but sends only a few bytes
    /// and never finishes. If the handler buffered before validating, it would
    /// block waiting for the remaining ~4 MiB and this read would time out
    /// instead of returning 403.
    #[tokio::test]
    async fn rejects_unauthenticated_request_without_reading_its_body() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let addr = spawn_test_server(Some("s3cret")).await;
        let mut stream = tokio::net::TcpStream::connect(addr).await.expect("connect");

        let head = format!(
            "POST /webhook HTTP/1.1\r\n\
             Host: {addr}\r\n\
             Content-Type: application/json\r\n\
             X-Telegram-Bot-Api-Secret-Token: wrong\r\n\
             Content-Length: 4194304\r\n\
             \r\n\
             {{\"update_id\":"
        );
        stream.write_all(head.as_bytes()).await.expect("write");

        // Read the status line with a bounded wait: the server must answer on
        // the strength of the headers alone.
        let mut buf = [0u8; 64];
        let read = tokio::time::timeout(std::time::Duration::from_secs(5), stream.read(&mut buf))
            .await
            .expect("server blocked on the unauthenticated body instead of rejecting it")
            .expect("read");

        let status = std::str::from_utf8(&buf[..read]).expect("utf8");
        assert!(
            status.starts_with("HTTP/1.1 403"),
            "expected 403, got: {status}"
        );
    }

    /// The mirror image: a valid secret with a body that is never finished must
    /// still block, because authenticated callers *are* allowed to send a body.
    #[tokio::test]
    async fn authenticated_request_still_reads_its_body() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let addr = spawn_test_server(Some("s3cret")).await;
        let mut stream = tokio::net::TcpStream::connect(addr).await.expect("connect");

        let head = format!(
            "POST /webhook HTTP/1.1\r\n\
             Host: {addr}\r\n\
             Content-Type: application/json\r\n\
             X-Telegram-Bot-Api-Secret-Token: s3cret\r\n\
             Content-Length: 4194304\r\n\
             \r\n\
             {{\"update_id\":"
        );
        stream.write_all(head.as_bytes()).await.expect("write");

        let mut buf = [0u8; 64];
        let result = tokio::time::timeout(std::time::Duration::from_secs(1), stream.read(&mut buf))
            .await;

        assert!(
            result.is_err(),
            "an authenticated request should be waiting on its body, not answered early"
        );
    }
}
