//! Webhook HTTP server built on `axum`.
//!
//! Port of `telegram.ext._utils.webhookhandler`. Gated behind `feature = "webhooks"`.
//!
//! The server accepts POST requests on a configurable path, validates the
//! `Content-Type` and optional secret token, deserializes the Telegram
//! `Update`, and pushes it into a `tokio::sync::mpsc` channel.

#![cfg(feature = "webhooks")]

use std::net::SocketAddr;
use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Notify};
use tracing::{debug, error};

use telegram_bot_raw::types::update::Update;

// ---------------------------------------------------------------------------
// Shared state
// ---------------------------------------------------------------------------

/// Internal state shared across all request handlers.
#[derive(Debug, Clone)]
struct WebhookState {
    /// Channel through which parsed updates are forwarded.
    update_tx: mpsc::Sender<Update>,
    /// Optional secret token for request validation.
    secret_token: Option<String>,
}

// ---------------------------------------------------------------------------
// WebhookServer
// ---------------------------------------------------------------------------

/// A thin wrapper around an `axum` HTTP server that receives Telegram webhook
/// POSTs and pushes deserialized updates into a channel.
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
        let path = if url_path.starts_with('/') {
            url_path.to_owned()
        } else {
            format!("/{url_path}")
        };

        let state = WebhookState {
            update_tx,
            secret_token,
        };

        let router = Router::new()
            .route(&path, post(handle_webhook))
            .with_state(state);

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
    /// The server runs until [`shutdown`](Self::shutdown) is called.
    pub async fn serve_forever(
        &self,
        ready: Option<Arc<Notify>>,
    ) -> Result<(), std::io::Error> {
        let addr: SocketAddr = format!("{}:{}", self.listen, self.port)
            .parse()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

        let listener = TcpListener::bind(addr).await?;
        self.running
            .store(true, std::sync::atomic::Ordering::Relaxed);

        debug!("Webhook server started on {addr}");

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
        debug!("Webhook server stopped");
        Ok(())
    }

    /// Signal the server to shut down gracefully.
    pub fn shutdown(&self) {
        if self.is_running() {
            debug!("Shutting down webhook server");
            self.shutdown_notify.notify_one();
        }
    }
}

// ---------------------------------------------------------------------------
// axum handler
// ---------------------------------------------------------------------------

/// The POST handler that receives Telegram updates.
async fn handle_webhook(
    State(state): State<WebhookState>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    // -- validate content type -----------------------------------------------
    let ct = headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !ct.starts_with("application/json") {
        debug!("Rejected request with Content-Type: {ct}");
        return StatusCode::FORBIDDEN;
    }

    // -- validate secret token -----------------------------------------------
    if let Some(ref expected) = state.secret_token {
        let provided = headers
            .get("x-telegram-bot-api-secret-token")
            .and_then(|v| v.to_str().ok());
        match provided {
            None => {
                debug!("Request did not include the secret token");
                return StatusCode::FORBIDDEN;
            }
            Some(tok) if tok != expected => {
                debug!("Request had wrong secret token: {tok}");
                return StatusCode::FORBIDDEN;
            }
            Some(_) => {}
        }
    }

    // -- parse update --------------------------------------------------------
    let update: Update = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to parse update JSON: {e}");
            return StatusCode::BAD_REQUEST;
        }
    };

    debug!("Webhook received update");

    if let Err(e) = state.update_tx.send(update).await {
        error!("Failed to enqueue update: {e}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

// ---------------------------------------------------------------------------
// WebhookApp (convenience builder)
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

#[cfg(test)]
mod tests {
    use super::*;

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
        let response = resp.into_response();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn rejects_missing_secret_token() {
        let (tx, _rx) = mpsc::channel(1);
        let state = WebhookState {
            update_tx: tx,
            secret_token: Some("my-secret".into()),
        };
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        let resp = handle_webhook(
            State(state),
            headers,
            Bytes::from_static(b"{}"),
        )
        .await;
        let response = resp.into_response();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn rejects_wrong_secret_token() {
        let (tx, _rx) = mpsc::channel(1);
        let state = WebhookState {
            update_tx: tx,
            secret_token: Some("correct".into()),
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
        let response = resp.into_response();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
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
        let response = resp.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let update = rx.recv().await.unwrap();
        assert_eq!(update.update_id, 1);
    }

    #[tokio::test]
    async fn accepts_valid_request_with_secret() {
        let (tx, mut rx) = mpsc::channel(1);
        let state = WebhookState {
            update_tx: tx,
            secret_token: Some("mysecret".into()),
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
        let response = resp.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let update = rx.recv().await.unwrap();
        assert_eq!(update.update_id, 42);
    }
}
