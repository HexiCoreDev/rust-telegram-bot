//! Fetches updates via long polling or webhook and pushes them into a channel.
//!
//! Port of `telegram.ext._updater.Updater`.
//!
//! The `Updater` is the bridge between Telegram and the application: it either
//! polls `getUpdates` or starts a webhook server, then forwards every
//! `Update` into a `tokio::sync::mpsc` channel for the `Application` to
//! consume.

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{mpsc, watch, Mutex};
use tracing::{debug, error, warn};

use telegram_bot_raw::error::TelegramError;

use crate::utils::network_loop::{network_retry_loop, NetworkLoopConfig};

#[cfg(feature = "webhooks")]
use tokio::sync::Notify;

#[cfg(feature = "webhooks")]
use crate::utils::webhook_handler::WebhookServer;

#[cfg(feature = "webhooks")]
use telegram_bot_raw::types::update::Update;

// ---------------------------------------------------------------------------
// Function types
// ---------------------------------------------------------------------------

/// A function that fetches updates from the Telegram API.
/// Signature: `(offset, timeout, allowed_updates) -> Result<Vec<Value>>`.
pub type GetUpdatesFn = Arc<
    dyn Fn(
            i64,
            Duration,
            Option<Vec<String>>,
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<Output = Result<Vec<serde_json::Value>, TelegramError>>
                    + Send,
            >,
        > + Send
        + Sync,
>;

/// A function that deletes the webhook. Signature: `(drop_pending) -> Result<()>`.
pub type DeleteWebhookFn = Arc<
    dyn Fn(
            bool,
        )
            -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TelegramError>> + Send>>
        + Send
        + Sync,
>;

// ---------------------------------------------------------------------------
// Configuration types
// ---------------------------------------------------------------------------

/// Configuration for [`Updater::start_polling`].
#[derive(Clone)]
pub struct PollingConfig {
    pub poll_interval: Duration,
    pub timeout: Duration,
    pub bootstrap_retries: i32,
    pub allowed_updates: Option<Vec<String>>,
    pub drop_pending_updates: bool,
    /// The function used to call `getUpdates`.
    pub get_updates: GetUpdatesFn,
    /// The function used to delete the webhook during bootstrap.
    pub delete_webhook: DeleteWebhookFn,
}

/// Configuration for [`Updater::start_webhook`].
#[cfg(feature = "webhooks")]
#[derive(Clone)]
pub struct WebhookConfig {
    pub listen: String,
    pub port: u16,
    pub url_path: String,
    pub webhook_url: Option<String>,
    pub secret_token: Option<String>,
    pub bootstrap_retries: i32,
    pub drop_pending_updates: bool,
    pub allowed_updates: Option<Vec<String>>,
    pub max_connections: u32,
}

#[cfg(feature = "webhooks")]
impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            listen: "127.0.0.1".into(),
            port: 80,
            url_path: String::new(),
            webhook_url: None,
            secret_token: None,
            bootstrap_retries: 0,
            drop_pending_updates: false,
            allowed_updates: None,
            max_connections: 40,
        }
    }
}

// ---------------------------------------------------------------------------
// Updater
// ---------------------------------------------------------------------------

/// Fetches updates for the bot via long polling or webhooks and forwards
/// them through [`take_update_rx`](Updater::take_update_rx).
pub struct Updater {
    update_tx: mpsc::Sender<serde_json::Value>,
    update_rx: Mutex<Option<mpsc::Receiver<serde_json::Value>>>,
    running: std::sync::atomic::AtomicBool,
    initialized: std::sync::atomic::AtomicBool,
    last_update_id: Mutex<i64>,
    /// Sending `true` signals the polling loop to stop.
    stop_tx: watch::Sender<bool>,
    /// The webhook server, if one was started.
    #[cfg(feature = "webhooks")]
    httpd: Mutex<Option<Arc<WebhookServer>>>,
}

impl std::fmt::Debug for Updater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Updater")
            .field("running", &self.is_running())
            .field(
                "initialized",
                &self.initialized.load(std::sync::atomic::Ordering::Relaxed),
            )
            .finish()
    }
}

impl Updater {
    /// Create a new `Updater`.
    ///
    /// `channel_size` controls the bounded channel capacity.
    pub fn new(channel_size: usize) -> Self {
        let (update_tx, update_rx) = mpsc::channel(channel_size);
        let (stop_tx, _stop_rx) = watch::channel(false);
        Self {
            update_tx,
            update_rx: Mutex::new(Some(update_rx)),
            running: false.into(),
            initialized: false.into(),
            last_update_id: Mutex::new(0),
            stop_tx,
            #[cfg(feature = "webhooks")]
            httpd: Mutex::new(None),
        }
    }

    /// Take ownership of the receiving end of the update channel. Can only be
    /// called once; subsequent calls return `None`.
    pub async fn take_update_rx(&self) -> Option<mpsc::Receiver<serde_json::Value>> {
        self.update_rx.lock().await.take()
    }

    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::Relaxed)
    }

    // -----------------------------------------------------------------------
    // Lifecycle
    // -----------------------------------------------------------------------

    /// Initialize the updater.
    pub async fn initialize(&self) {
        if self.initialized.load(std::sync::atomic::Ordering::Relaxed) {
            debug!("Updater already initialized");
            return;
        }
        self.initialized
            .store(true, std::sync::atomic::Ordering::Relaxed);
        debug!("Updater initialized");
    }

    /// Shut down the updater. Must not be called while still running.
    pub async fn shutdown(&self) -> Result<(), UpdaterError> {
        if self.is_running() {
            return Err(UpdaterError::StillRunning);
        }
        if !self.initialized.load(std::sync::atomic::Ordering::Relaxed) {
            debug!("Updater already shut down");
            return Ok(());
        }
        self.initialized
            .store(false, std::sync::atomic::Ordering::Relaxed);
        debug!("Updater shut down");
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Polling
    // -----------------------------------------------------------------------

    /// Start polling for updates.
    ///
    /// Returns immediately after the bootstrap phase completes. Updates are
    /// sent through the channel returned by [`take_update_rx`](Self::take_update_rx).
    pub async fn start_polling(
        self: &Arc<Self>,
        config: PollingConfig,
    ) -> Result<(), UpdaterError> {
        if self.is_running() {
            return Err(UpdaterError::AlreadyRunning);
        }
        if !self.initialized.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(UpdaterError::NotInitialized);
        }

        self.running
            .store(true, std::sync::atomic::Ordering::Relaxed);

        // Reset the stop signal from any prior run.
        let _ = self.stop_tx.send(false);

        // Bootstrap: delete any existing webhook.
        let delete_fn = config.delete_webhook.clone();
        let drop_pending = config.drop_pending_updates;
        let bootstrap_retries = config.bootstrap_retries;

        if let Err(e) = self
            .bootstrap_delete_webhook(delete_fn, drop_pending, bootstrap_retries)
            .await
        {
            self.running
                .store(false, std::sync::atomic::Ordering::Relaxed);
            return Err(UpdaterError::Bootstrap(e.to_string()));
        }

        debug!("Bootstrap complete, starting polling loop");

        let updater = Arc::clone(self);
        let stop_rx = self.stop_tx.subscribe();

        tokio::spawn(async move {
            let tx = updater.update_tx.clone();
            let timeout = config.timeout;
            let poll_interval = config.poll_interval;
            let allowed = config.allowed_updates.clone();
            let get_updates_fn = config.get_updates.clone();

            let result = network_retry_loop(NetworkLoopConfig {
                action_cb: || {
                    let tx = tx.clone();
                    let updater_inner = updater.clone();
                    let allowed_inner = allowed.clone();
                    let get_fn = get_updates_fn.clone();
                    async move {
                        let last_id = { *updater_inner.last_update_id.lock().await };
                        let updates: Vec<serde_json::Value> =
                            get_fn(last_id, timeout, allowed_inner).await?;
                        if !updates.is_empty() {
                            if !updater_inner.is_running() {
                                warn!(
                                    "Updater stopped unexpectedly. Pulled updates will be \
                                     ignored and pulled again on restart."
                                );
                                return Ok(());
                            }
                            for update in &updates {
                                if let Err(e) = tx.send(update.clone()).await {
                                    error!("Failed to enqueue update: {e}");
                                }
                            }
                            if let Some(last) = updates.last() {
                                if let Some(uid) = last.get("update_id").and_then(|v| v.as_i64()) {
                                    *updater_inner.last_update_id.lock().await = uid + 1;
                                }
                            }
                        }
                        Ok(())
                    }
                },
                on_err_cb: Some(|e: &TelegramError| {
                    error!("Error while polling for updates: {e}");
                }),
                description: "Polling Updates",
                interval: poll_interval.as_secs_f64(),
                stop_rx: Some(stop_rx),
                is_running: Some(Box::new({
                    let u = updater.clone();
                    move || u.is_running()
                })),
                max_retries: -1,
                repeat_on_success: true,
            })
            .await;

            if let Err(e) = result {
                error!("Polling loop exited with error: {e}");
            }
        });

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Webhook
    // -----------------------------------------------------------------------

    /// Start a webhook server to receive updates.
    #[cfg(feature = "webhooks")]
    pub async fn start_webhook(
        self: &Arc<Self>,
        config: WebhookConfig,
    ) -> Result<(), UpdaterError> {
        if self.is_running() {
            return Err(UpdaterError::AlreadyRunning);
        }
        if !self.initialized.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(UpdaterError::NotInitialized);
        }

        self.running
            .store(true, std::sync::atomic::Ordering::Relaxed);
        let _ = self.stop_tx.send(false);

        // WebhookServer expects Sender<Update> but the updater channel carries
        // serde_json::Value. Bridge the two with an intermediate typed channel.
        let (typed_tx, mut typed_rx) = mpsc::channel::<Update>(256);
        let value_tx = self.update_tx.clone();
        tokio::spawn(async move {
            while let Some(update) = typed_rx.recv().await {
                match serde_json::to_value(&update) {
                    Ok(v) => {
                        let _ = value_tx.send(v).await;
                    }
                    Err(e) => {
                        error!("Failed to serialize Update to Value: {e}");
                    }
                }
            }
        });

        let server = Arc::new(WebhookServer::new(
            &config.listen,
            config.port,
            &config.url_path,
            typed_tx,
            config.secret_token,
        ));

        let ready = Arc::new(Notify::new());
        let ready_clone = ready.clone();

        let srv = server.clone();
        tokio::spawn(async move {
            if let Err(e) = srv.serve_forever(Some(ready_clone)).await {
                error!("Webhook server error: {e}");
            }
        });

        ready.notified().await;
        debug!(
            "Webhook server started on {}:{}",
            config.listen, config.port
        );

        *self.httpd.lock().await = Some(server);

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Stop
    // -----------------------------------------------------------------------

    /// Stop the updater (both polling and webhook).
    pub async fn stop(&self) -> Result<(), UpdaterError> {
        if !self.is_running() {
            return Err(UpdaterError::NotRunning);
        }
        debug!("Stopping updater");
        self.running
            .store(false, std::sync::atomic::Ordering::Relaxed);

        // Signal the polling loop to stop.
        let _ = self.stop_tx.send(true);

        // Shut down webhook server if present.
        #[cfg(feature = "webhooks")]
        {
            let httpd = self.httpd.lock().await;
            if let Some(ref server) = *httpd {
                server.shutdown();
            }
        }

        debug!("Updater stopped");
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Bootstrap helpers
    // -----------------------------------------------------------------------

    async fn bootstrap_delete_webhook(
        &self,
        delete_fn: DeleteWebhookFn,
        drop_pending: bool,
        max_retries: i32,
    ) -> Result<(), TelegramError> {
        debug!("Deleting webhook (bootstrap)");
        network_retry_loop(NetworkLoopConfig {
            action_cb: || {
                let f = delete_fn.clone();
                async move { f(drop_pending).await }
            },
            on_err_cb: None::<fn(&TelegramError)>,
            description: "Bootstrap delete webhook",
            interval: 1.0,
            stop_rx: None,
            is_running: None,
            max_retries,
            repeat_on_success: false,
        })
        .await
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum UpdaterError {
    #[error("this Updater is already running")]
    AlreadyRunning,

    #[error("this Updater is not running")]
    NotRunning,

    #[error("this Updater was not initialized")]
    NotInitialized,

    #[error("this Updater is still running")]
    StillRunning,

    #[error("bootstrap failed: {0}")]
    Bootstrap(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn noop_get_updates() -> GetUpdatesFn {
        Arc::new(|_offset, _timeout, _allowed| Box::pin(async { Ok(Vec::new()) }))
    }

    fn noop_delete_webhook() -> DeleteWebhookFn {
        Arc::new(|_drop_pending| Box::pin(async { Ok(()) }))
    }

    fn default_config() -> PollingConfig {
        PollingConfig {
            poll_interval: Duration::ZERO,
            timeout: Duration::from_secs(1),
            bootstrap_retries: 0,
            allowed_updates: None,
            drop_pending_updates: false,
            get_updates: noop_get_updates(),
            delete_webhook: noop_delete_webhook(),
        }
    }

    #[tokio::test]
    async fn lifecycle() {
        let updater = Arc::new(Updater::new(16));
        assert!(!updater.is_running());

        updater.initialize().await;

        // Can't stop before starting.
        assert!(updater.stop().await.is_err());

        updater.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn start_polling_requires_init() {
        let updater = Arc::new(Updater::new(16));
        let result = updater.start_polling(default_config()).await;
        assert!(matches!(result, Err(UpdaterError::NotInitialized)));
    }

    #[tokio::test]
    async fn start_and_stop_polling() {
        let updater = Arc::new(Updater::new(16));
        updater.initialize().await;
        updater.start_polling(default_config()).await.unwrap();
        assert!(updater.is_running());

        // Can't start twice.
        let result = updater.start_polling(default_config()).await;
        assert!(matches!(result, Err(UpdaterError::AlreadyRunning)));

        updater.stop().await.unwrap();
        assert!(!updater.is_running());
    }

    #[tokio::test]
    async fn take_update_rx_once() {
        let updater = Arc::new(Updater::new(16));
        let rx = updater.take_update_rx().await;
        assert!(rx.is_some());
        let rx2 = updater.take_update_rx().await;
        assert!(rx2.is_none());
    }

    #[tokio::test]
    async fn polling_delivers_updates() {
        let updater = Arc::new(Updater::new(16));
        updater.initialize().await;

        let mut rx = updater.take_update_rx().await.unwrap();

        // A get_updates that returns one update then empty.
        let call_count = Arc::new(std::sync::atomic::AtomicU32::new(0));
        let cc = call_count.clone();
        let get_fn: GetUpdatesFn = Arc::new(move |_offset, _timeout, _allowed| {
            let cc = cc.clone();
            Box::pin(async move {
                let n = cc.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n == 0 {
                    Ok(vec![serde_json::json!({"update_id": 100, "message": {}})])
                } else {
                    Ok(Vec::new())
                }
            })
        });

        let config = PollingConfig {
            poll_interval: Duration::from_millis(10),
            timeout: Duration::from_secs(1),
            bootstrap_retries: 0,
            allowed_updates: None,
            drop_pending_updates: false,
            get_updates: get_fn,
            delete_webhook: noop_delete_webhook(),
        };

        updater.start_polling(config).await.unwrap();

        // Should receive the update within a reasonable time.
        let update = tokio::time::timeout(Duration::from_secs(2), rx.recv())
            .await
            .expect("timeout waiting for update")
            .expect("channel closed");

        assert_eq!(update["update_id"], 100);

        updater.stop().await.unwrap();
    }
}
