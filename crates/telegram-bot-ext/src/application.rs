//! The core Application that dispatches updates to registered handlers.
//!
//! Ported from `python-telegram-bot/src/telegram/ext/_application.py`.
//!
//! ## Lifecycle
//!
//! ```text
//! initialize  ->  start  ->  idle (run_polling / run_webhook)  ->  stop  ->  shutdown
//! ```
//!
//! ## Handler dispatch
//!
//! Handlers are organised into *groups* (a `BTreeMap<i32, Vec<Handler>>`).  Groups are
//! iterated in ascending numeric order.  Within each group, the first handler whose
//! `check_update` returns `true` wins -- at most one handler per group fires.

use std::collections::{BTreeMap, HashMap};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Weak};

use serde_json::Value;
use tokio::sync::{mpsc, Notify, RwLock};
use tracing::{debug, error, info, warn};

use telegram_bot_raw::types::update::Update;

use crate::context::CallbackContext;
use crate::context_types::{ContextTypes, DefaultData};
use crate::ext_bot::ExtBot;
use crate::job_queue::JobQueue;
use crate::persistence::base::{BasePersistence, PersistenceInput, PersistenceResult};
use crate::update_processor::BaseUpdateProcessor;
use crate::utils::types::JsonMap;

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

/// A boxed, type-erased async handler callback.
pub type HandlerCallback = Arc<
    dyn Fn(
            Arc<Update>,
            CallbackContext,
        ) -> Pin<Box<dyn Future<Output = Result<(), HandlerError>> + Send>>
        + Send
        + Sync,
>;

/// A boxed, type-erased async error handler callback.
///
/// Returns `true` if the error handler signals that processing should stop.
pub type ErrorHandlerCallback = Arc<
    dyn Fn(Option<Arc<Update>>, CallbackContext) -> Pin<Box<dyn Future<Output = bool> + Send>>
        + Send
        + Sync,
>;

/// Post-lifecycle hook signature.
pub type LifecycleHook =
    Arc<dyn Fn(Arc<Application>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

type PersistenceDataMap = HashMap<i64, JsonMap>;
type PersistenceFuture<'a, T> = Pin<Box<dyn Future<Output = PersistenceResult<T>> + Send + 'a>>;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors that can occur during update processing.
#[derive(Debug, thiserror::Error)]
pub enum HandlerError {
    #[error("ApplicationHandlerStop")]
    HandlerStop { state: Option<Value> },

    #[error("{0}")]
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl From<telegram_bot_raw::error::TelegramError> for HandlerError {
    fn from(e: telegram_bot_raw::error::TelegramError) -> Self {
        HandlerError::Other(Box::new(e))
    }
}

/// Errors from the Application itself.
#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("This Application was not initialized via `Application::initialize`")]
    NotInitialized,

    #[error("This Application is already running")]
    AlreadyRunning,

    #[error("This Application is not running")]
    NotRunning,

    #[error("This Application is still running")]
    StillRunning,

    #[error("{0}")]
    Bot(#[from] telegram_bot_raw::error::TelegramError),

    #[error("{0}")]
    UpdateProcessor(#[from] crate::update_processor::UpdateProcessorError),
}

// ---------------------------------------------------------------------------
// DynPersistence -- object-safe wrapper for BasePersistence
// ---------------------------------------------------------------------------

/// Object-safe wrapper around [`BasePersistence`] so we can store it as a
/// trait object inside [`Application`].
pub trait DynPersistence: Send + Sync + std::fmt::Debug {
    /// Load all user data.
    fn get_user_data(&self) -> PersistenceFuture<'_, PersistenceDataMap>;
    /// Load all chat data.
    fn get_chat_data(&self) -> PersistenceFuture<'_, PersistenceDataMap>;
    /// Load bot-wide data.
    fn get_bot_data(&self) -> PersistenceFuture<'_, JsonMap>;
    /// Persist user data for a single user.
    fn update_user_data(&self, user_id: i64, data: JsonMap) -> PersistenceFuture<'_, ()>;
    /// Persist chat data for a single chat.
    fn update_chat_data(&self, chat_id: i64, data: JsonMap) -> PersistenceFuture<'_, ()>;
    /// Persist bot-wide data.
    fn update_bot_data(&self, data: JsonMap) -> PersistenceFuture<'_, ()>;
    /// Flush all pending writes.
    fn flush(&self) -> PersistenceFuture<'_, ()>;
    /// Update interval in seconds.
    fn update_interval(&self) -> f64;
    /// Which data categories to persist.
    fn store_data(&self) -> PersistenceInput;
}

impl<T: BasePersistence + std::fmt::Debug> DynPersistence for T {
    fn get_user_data(&self) -> PersistenceFuture<'_, PersistenceDataMap> {
        Box::pin(BasePersistence::get_user_data(self))
    }
    fn get_chat_data(&self) -> PersistenceFuture<'_, PersistenceDataMap> {
        Box::pin(BasePersistence::get_chat_data(self))
    }
    fn get_bot_data(&self) -> PersistenceFuture<'_, JsonMap> {
        Box::pin(BasePersistence::get_bot_data(self))
    }
    fn update_user_data(&self, user_id: i64, data: JsonMap) -> PersistenceFuture<'_, ()> {
        Box::pin(async move { BasePersistence::update_user_data(self, user_id, &data).await })
    }
    fn update_chat_data(&self, chat_id: i64, data: JsonMap) -> PersistenceFuture<'_, ()> {
        Box::pin(async move { BasePersistence::update_chat_data(self, chat_id, &data).await })
    }
    fn update_bot_data(&self, data: JsonMap) -> PersistenceFuture<'_, ()> {
        Box::pin(async move { BasePersistence::update_bot_data(self, &data).await })
    }
    fn flush(&self) -> PersistenceFuture<'_, ()> {
        Box::pin(BasePersistence::flush(self))
    }
    fn update_interval(&self) -> f64 {
        BasePersistence::update_interval(self)
    }
    fn store_data(&self) -> PersistenceInput {
        BasePersistence::store_data(self)
    }
}

/// Wrap any `BasePersistence` implementor into a boxed `DynPersistence`.
pub fn boxed_persistence<T: BasePersistence + std::fmt::Debug + 'static>(
    p: T,
) -> Box<dyn DynPersistence> {
    Box::new(p)
}

// ---------------------------------------------------------------------------
// Handler wrapper
// ---------------------------------------------------------------------------

/// A registered handler: the `check_update` predicate + callback + blocking flag.
pub struct Handler {
    pub check_update: Arc<dyn Fn(&Update) -> bool + Send + Sync>,
    pub callback: HandlerCallback,
    pub block: bool,
}

impl std::fmt::Debug for Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Handler")
            .field("block", &self.block)
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Application
// ---------------------------------------------------------------------------

/// The main entry point for a PTB-style Telegram bot application.
pub struct Application {
    bot: Arc<ExtBot>,
    #[allow(dead_code)]
    context_types: ContextTypes,
    update_processor: Arc<BaseUpdateProcessor>,

    handlers: RwLock<BTreeMap<i32, Vec<Handler>>>,
    error_handlers: RwLock<Vec<(ErrorHandlerCallback, bool)>>,

    user_data: Arc<RwLock<HashMap<i64, DefaultData>>>,
    chat_data: Arc<RwLock<HashMap<i64, DefaultData>>>,
    bot_data: Arc<RwLock<DefaultData>>,

    persistence: Option<Box<dyn DynPersistence>>,
    job_queue: Option<Arc<JobQueue>>,
    pending_tasks: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,

    initialized: RwLock<bool>,
    running: RwLock<bool>,

    update_tx: mpsc::UnboundedSender<Arc<Update>>,
    update_rx: RwLock<Option<mpsc::UnboundedReceiver<Arc<Update>>>>,
    stop_notify: Arc<Notify>,

    post_init: Option<LifecycleHook>,
    post_stop: Option<LifecycleHook>,
    post_shutdown: Option<LifecycleHook>,
}

impl std::fmt::Debug for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Application")
            .field("bot_token", &self.bot.token())
            .finish()
    }
}

pub const DEFAULT_GROUP: i32 = 0;

pub(crate) struct ApplicationConfig {
    pub(crate) bot: Arc<ExtBot>,
    pub(crate) context_types: ContextTypes,
    pub(crate) update_processor: Arc<BaseUpdateProcessor>,
    pub(crate) post_init: Option<LifecycleHook>,
    pub(crate) post_stop: Option<LifecycleHook>,
    pub(crate) post_shutdown: Option<LifecycleHook>,
    pub(crate) persistence: Option<Box<dyn DynPersistence>>,
    pub(crate) job_queue: Option<Arc<JobQueue>>,
}

impl ApplicationConfig {
    pub(crate) fn new(
        bot: Arc<ExtBot>,
        context_types: ContextTypes,
        update_processor: Arc<BaseUpdateProcessor>,
    ) -> Self {
        Self {
            bot,
            context_types,
            update_processor,
            post_init: None,
            post_stop: None,
            post_shutdown: None,
            persistence: None,
            job_queue: None,
        }
    }
}

impl Application {
    /// Creates a new `Application`.
    ///
    /// Prefer [`ApplicationBuilder`](crate::builder::ApplicationBuilder) for public
    /// construction -- this avoids long positional argument lists.
    #[must_use]
    pub(crate) fn new(config: ApplicationConfig) -> Arc<Self> {
        let ApplicationConfig {
            bot,
            context_types,
            update_processor,
            post_init,
            post_stop,
            post_shutdown,
            persistence,
            job_queue,
        } = config;
        let (tx, rx) = mpsc::unbounded_channel();
        let bot_data_initial = context_types.bot_data();
        Arc::new(Self {
            bot,
            context_types,
            update_processor,
            handlers: RwLock::new(BTreeMap::new()),
            error_handlers: RwLock::new(Vec::new()),
            user_data: Arc::new(RwLock::new(HashMap::new())),
            chat_data: Arc::new(RwLock::new(HashMap::new())),
            bot_data: Arc::new(RwLock::new(bot_data_initial)),
            persistence,
            job_queue,
            pending_tasks: Arc::new(RwLock::new(Vec::new())),
            initialized: RwLock::new(false),
            running: RwLock::new(false),
            update_tx: tx,
            update_rx: RwLock::new(Some(rx)),
            stop_notify: Arc::new(Notify::new()),
            post_init,
            post_stop,
            post_shutdown,
        })
    }

    // -- Accessors --
    #[must_use]
    pub fn bot(&self) -> &Arc<ExtBot> {
        &self.bot
    }
    pub async fn is_initialized(&self) -> bool {
        *self.initialized.read().await
    }
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
    #[must_use]
    pub fn concurrent_updates(&self) -> usize {
        self.update_processor.max_concurrent_updates()
    }
    #[must_use]
    pub fn user_data(&self) -> &Arc<RwLock<HashMap<i64, DefaultData>>> {
        &self.user_data
    }
    #[must_use]
    pub fn chat_data(&self) -> &Arc<RwLock<HashMap<i64, DefaultData>>> {
        &self.chat_data
    }
    #[must_use]
    pub fn bot_data(&self) -> &Arc<RwLock<DefaultData>> {
        &self.bot_data
    }
    #[must_use]
    pub fn update_sender(&self) -> mpsc::UnboundedSender<Arc<Update>> {
        self.update_tx.clone()
    }
    #[must_use]
    pub fn job_queue(&self) -> Option<&Arc<JobQueue>> {
        self.job_queue.as_ref()
    }

    // -- Lifecycle: initialize --
    pub async fn initialize(&self) -> Result<(), ApplicationError> {
        let mut init = self.initialized.write().await;
        if *init {
            debug!("This Application is already initialized.");
            return Ok(());
        }

        self.bot.initialize().await?;
        self.update_processor.initialize().await;

        // C8: Load data from persistence
        if let Some(ref persistence) = self.persistence {
            let sd = persistence.store_data();
            if sd.user_data {
                if let Ok(data) = persistence.get_user_data().await {
                    *self.user_data.write().await = data;
                }
            }
            if sd.chat_data {
                if let Ok(data) = persistence.get_chat_data().await {
                    *self.chat_data.write().await = data;
                }
            }
            if sd.bot_data {
                if let Ok(data) = persistence.get_bot_data().await {
                    *self.bot_data.write().await = data;
                }
            }
        }

        // M14: Start the job queue
        if let Some(ref jq) = self.job_queue {
            jq.start().await;
        }

        *init = true;
        Ok(())
    }

    // -- Lifecycle: shutdown --
    pub async fn shutdown(&self) -> Result<(), ApplicationError> {
        if *self.running.read().await {
            return Err(ApplicationError::StillRunning);
        }
        let mut init = self.initialized.write().await;
        if !*init {
            debug!("This Application is already shut down.");
            return Ok(());
        }

        // C8: Flush persistence
        if let Some(ref persistence) = self.persistence {
            if let Err(e) = persistence.flush().await {
                error!("Failed to flush persistence: {e}");
            }
        }

        self.bot.shutdown().await?;
        self.update_processor.shutdown().await;
        *init = false;
        Ok(())
    }

    // -- Lifecycle: start / stop --
    pub async fn start(self: &Arc<Self>) -> Result<(), ApplicationError> {
        if *self.running.read().await {
            return Err(ApplicationError::AlreadyRunning);
        }
        self.check_initialized().await?;
        {
            *self.running.write().await = true;
        }

        // Wire job queue hooks so that job runs trigger persistence
        // flushes (GAP 1) and route errors to error handlers (GAP 2),
        // matching PTB's Job._run() behavior.
        if let Some(ref jq) = self.job_queue {
            let app_weak: Weak<Application> = Arc::downgrade(self);

            // GAP 1: After every job callback, flush persistence.
            let weak_complete = app_weak.clone();
            jq.set_on_job_complete(Arc::new(move || {
                let weak = weak_complete.clone();
                Box::pin(async move {
                    if let Some(app) = weak.upgrade() {
                        app.update_persistence().await;
                    }
                })
            }))
            .await;

            // GAP 2: When a job callback errors, route through process_error.
            let weak_error = app_weak;
            jq.set_on_job_error(Arc::new(
                move |err: Box<dyn std::error::Error + Send + Sync>| {
                    let weak = weak_error.clone();
                    Box::pin(async move {
                        if let Some(app) = weak.upgrade() {
                            app.process_error(None, err).await;
                        }
                    })
                },
            ))
            .await;
        }

        let rx = { self.update_rx.write().await.take() };
        if let Some(mut rx) = rx {
            let app = Arc::clone(self);
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        Some(update) = rx.recv() => {
                            debug!("Processing update");
                            let app2 = Arc::clone(&app);
                            let up = app.update_processor.clone();
                            let update_clone = Arc::clone(&update);
                            let fut: Pin<Box<dyn Future<Output = ()> + Send>> = Box::pin(async move {
                                if let Err(e) = app2.process_update(update_clone).await { error!("Error processing update: {e}"); }
                            });
                            if app.update_processor.max_concurrent_updates() > 1 {
                                tokio::spawn(async move { up.process_update(update, fut).await; });
                            } else {
                                up.process_update(update, fut).await;
                            }
                        }
                        _ = app.stop_notify.notified() => { debug!("Update fetcher received stop signal"); break; }
                    }
                }
                info!("Update fetcher stopped");
            });
        }
        info!("Application started");
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), ApplicationError> {
        if !*self.running.read().await {
            return Err(ApplicationError::NotRunning);
        }
        info!("Application is stopping. This might take a moment.");
        self.stop_notify.notify_waiters();

        if let Some(ref jq) = self.job_queue {
            jq.stop().await;
        }

        // M10: Await pending tasks with timeout
        {
            let mut tasks = self.pending_tasks.write().await;
            let handles: Vec<_> = tasks.drain(..).collect();
            drop(tasks);
            if !handles.is_empty() {
                debug!("Waiting for {} pending tasks", handles.len());
                let _ = tokio::time::timeout(
                    std::time::Duration::from_secs(5),
                    futures_join_all(handles),
                )
                .await;
            }
        }

        {
            *self.running.write().await = false;
        }
        info!("Application.stop() complete");
        Ok(())
    }

    pub fn stop_running(&self) {
        self.stop_notify.notify_waiters();
    }

    /// Spawn a background task and track its [`JoinHandle`](tokio::task::JoinHandle)
    /// in `pending_tasks`.
    ///
    /// The task will be awaited (with a timeout) when the application stops,
    /// preventing fire-and-forget futures from silently vanishing.
    ///
    /// This mirrors Python's `Application.create_task`.
    pub async fn create_task(&self, future: impl Future<Output = ()> + Send + 'static) {
        let handle = tokio::spawn(future);
        self.pending_tasks.write().await.push(handle);
    }

    // -- C8: Persistence update --
    pub async fn update_persistence(&self) {
        let persistence = match self.persistence.as_ref() {
            Some(p) => p,
            None => return,
        };
        let sd = persistence.store_data();
        if sd.user_data {
            for (uid, data) in self.user_data.read().await.iter() {
                let _ = persistence.update_user_data(*uid, data.clone()).await;
            }
        }
        if sd.chat_data {
            for (cid, data) in self.chat_data.read().await.iter() {
                let _ = persistence.update_chat_data(*cid, data.clone()).await;
            }
        }
        if sd.bot_data {
            let _ = persistence
                .update_bot_data(self.bot_data.read().await.clone())
                .await;
        }
    }

    // -- run_polling --
    /// Start the bot with long-polling using sensible defaults.
    ///
    /// Matches Python's `application.run_polling()` -- zero arguments needed.
    /// Defaults: poll_interval=0s, timeout=10s, no update filter, don't drop pending.
    pub async fn run_polling(self: Arc<Self>) -> Result<(), ApplicationError> {
        self.run_polling_configured(
            std::time::Duration::ZERO,
            std::time::Duration::from_secs(10),
            None,
            false,
        )
        .await
    }

    /// Returns a [`PollingBuilder`] for configuring and starting polling.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// app.polling()
    ///     .timeout(Duration::from_secs(30))
    ///     .drop_pending(true)
    ///     .start()
    ///     .await?;
    /// ```
    #[must_use]
    pub fn polling(self: &Arc<Self>) -> PollingBuilder {
        PollingBuilder::new(Arc::clone(self))
    }

    /// Start the bot with long-polling and custom configuration.
    ///
    /// Prefer [`Application::polling()`] builder for public use.
    pub(crate) async fn run_polling_configured(
        self: Arc<Self>,
        poll_interval: std::time::Duration,
        timeout: std::time::Duration,
        allowed_updates: Option<Vec<String>>,
        drop_pending_updates: bool,
    ) -> Result<(), ApplicationError> {
        self.initialize().await?;
        if let Some(ref hook) = self.post_init {
            hook(Arc::clone(&self)).await;
        }
        self.start().await?;

        // C8: periodic persistence
        let persistence_handle = if let Some(persistence) = self.persistence.as_ref() {
            let secs = persistence.update_interval();
            let app = Arc::clone(&self);
            let stop = Arc::clone(&self.stop_notify);
            Some(tokio::spawn(async move {
                let mut iv = tokio::time::interval(std::time::Duration::from_secs_f64(secs));
                iv.tick().await;
                loop {
                    tokio::select! { _ = iv.tick() => { app.update_persistence().await; } _ = stop.notified() => { break; } }
                }
            }))
        } else {
            None
        };

        let bot = Arc::clone(&self.bot);
        let tx = self.update_tx.clone();
        let stop = Arc::clone(&self.stop_notify);
        let allowed = allowed_updates;

        let poll_handle = tokio::spawn(async move {
            let mut offset: Option<i64> = None;
            if drop_pending_updates {
                if let Ok(updates) = bot
                    .inner()
                    .get_updates(Some(-1), Some(1), Some(0), None)
                    .await
                {
                    if let Some(last) = updates.last() {
                        offset = Some(last.update_id + 1);
                    }
                }
            }
            let timeout_secs = timeout.as_secs().max(1) as i32;
            loop {
                tokio::select! {
                    result = bot.inner().get_updates(offset, Some(100), Some(timeout_secs), allowed.clone()) => {
                        match result {
                            Ok(updates) => {
                                for update in updates {
                                    offset = Some(update.update_id + 1);
                                    let _ = tx.send(Arc::new(update));
                                }
                            }
                            Err(e) => { error!("Error fetching updates: {e}"); tokio::time::sleep(std::time::Duration::from_secs(1)).await; }
                        }
                    }
                    _ = stop.notified() => { return; }
                }
                if !poll_interval.is_zero() {
                    tokio::time::sleep(poll_interval).await;
                }
            }
        });

        info!("Application is running. Press Ctrl+C to stop.");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => { info!("Received Ctrl+C, shutting down..."); }
            _ = self.stop_notify.notified() => { info!("Received stop signal"); }
        }

        self.stop_notify.notify_waiters();
        let _ = poll_handle.await;
        if let Some(ph) = persistence_handle {
            let _ = ph.await;
        }
        if *self.running.read().await {
            self.stop().await?;
        }
        if let Some(ref hook) = self.post_stop {
            hook(Arc::clone(&self)).await;
        }
        self.shutdown().await?;
        if let Some(ref hook) = self.post_shutdown {
            hook(Arc::clone(&self)).await;
        }
        Ok(())
    }

    // -- M8: run_webhook --
    #[cfg(feature = "webhooks")]
    pub async fn run_webhook(
        self: Arc<Self>,
        config: crate::updater::WebhookConfig,
    ) -> Result<(), ApplicationError> {
        use crate::utils::webhook_handler::WebhookServer;

        self.initialize().await?;
        if let Some(ref hook) = self.post_init {
            hook(Arc::clone(&self)).await;
        }
        self.start().await?;

        let persistence_handle = if self.persistence.is_some() {
            let secs = self.persistence.as_ref().unwrap().update_interval();
            let app = Arc::clone(&self);
            let stop = Arc::clone(&self.stop_notify);
            Some(tokio::spawn(async move {
                let mut iv = tokio::time::interval(std::time::Duration::from_secs_f64(secs));
                iv.tick().await;
                loop {
                    tokio::select! { _ = iv.tick() => { app.update_persistence().await; } _ = stop.notified() => { break; } }
                }
            }))
        } else {
            None
        };

        let (bounded_tx, mut bounded_rx) = mpsc::channel::<Update>(256);
        let unbounded_tx = self.update_tx.clone();
        let bridge = tokio::spawn(async move {
            while let Some(u) = bounded_rx.recv().await {
                if unbounded_tx.send(Arc::new(u)).is_err() {
                    break;
                }
            }
        });

        let server = Arc::new(WebhookServer::new(
            &config.listen,
            config.port,
            &config.url_path,
            bounded_tx,
            config.secret_token.clone(),
        ));
        let ready = Arc::new(Notify::new());
        let rc = ready.clone();
        let srv = server.clone();
        let wh = tokio::spawn(async move {
            if let Err(e) = srv.serve_forever(Some(rc)).await {
                error!("Webhook server error: {e}");
            }
        });
        ready.notified().await;
        info!(
            "Webhook server started on {}:{}",
            config.listen, config.port
        );

        info!("Application is running via webhook. Press Ctrl+C to stop.");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => { info!("Received Ctrl+C"); }
            _ = self.stop_notify.notified() => { info!("Received stop signal"); }
        }

        self.stop_notify.notify_waiters();
        server.shutdown();
        let _ = wh.await;
        bridge.abort();
        if let Some(ph) = persistence_handle {
            let _ = ph.await;
        }
        if *self.running.read().await {
            self.stop().await?;
        }
        if let Some(ref hook) = self.post_stop {
            hook(Arc::clone(&self)).await;
        }
        self.shutdown().await?;
        if let Some(ref hook) = self.post_shutdown {
            hook(Arc::clone(&self)).await;
        }
        Ok(())
    }

    // -- Handler registration --
    pub async fn add_handler(&self, handler: Handler, group: i32) {
        self.handlers
            .write()
            .await
            .entry(group)
            .or_default()
            .push(handler);
    }
    pub async fn add_handlers(&self, new_handlers: Vec<Handler>, group: i32) {
        self.handlers
            .write()
            .await
            .entry(group)
            .or_default()
            .extend(new_handlers);
    }
    pub async fn remove_handler(&self, group: i32, index: usize) -> Option<Handler> {
        let mut handlers = self.handlers.write().await;
        if let Some(gh) = handlers.get_mut(&group) {
            if index < gh.len() {
                let removed = gh.remove(index);
                if gh.is_empty() {
                    handlers.remove(&group);
                }
                return Some(removed);
            }
        }
        None
    }
    pub async fn add_error_handler(&self, callback: ErrorHandlerCallback, block: bool) {
        self.error_handlers.write().await.push((callback, block));
    }

    /// Register a trait-based handler (`CommandHandler`, `MessageHandler`, etc.)
    /// into the Application's dispatch system.
    ///
    /// This bridges the trait-based handler system into the Application's
    /// internal `Handler` struct, creating the `CallbackContext` and calling
    /// `handle_update_with_context` so that ergonomic handlers receive both
    /// the typed `Update` and a fully-populated `CallbackContext`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use telegram_bot_ext::prelude::*;
    ///
    /// async fn start(update: Update, context: Context) -> HandlerResult {
    ///     context.reply_text(&update, "Hello!").await?;
    ///     Ok(())
    /// }
    ///
    /// app.add_typed_handler(CommandHandler::new("start", start), 0).await;
    /// ```
    pub async fn add_typed_handler(
        &self,
        handler: impl crate::handlers::base::Handler + 'static,
        group: i32,
    ) {
        use crate::handlers::base::HandlerResult as TraitHandlerResult;

        let handler = Arc::new(handler);

        let check_handler = Arc::clone(&handler);
        let callback_handler = Arc::clone(&handler);
        let bot = Arc::clone(&self.bot);
        let user_data = Arc::clone(&self.user_data);
        let chat_data = Arc::clone(&self.chat_data);
        let bot_data_ref = Arc::clone(&self.bot_data);
        let job_queue = self.job_queue.clone();

        let legacy = Handler {
            check_update: Arc::new(move |update: &Update| {
                check_handler.check_update(update).is_some()
            }),
            callback: Arc::new(move |update: Arc<Update>, _ctx: CallbackContext| {
                let h = Arc::clone(&callback_handler);
                let bot = Arc::clone(&bot);
                let ud = Arc::clone(&user_data);
                let cd = Arc::clone(&chat_data);
                let bd = Arc::clone(&bot_data_ref);
                let jq = job_queue.clone();
                Box::pin(async move {
                    let match_result = h
                        .check_update(&update)
                        .unwrap_or(crate::handlers::base::MatchResult::Empty);

                    // Create a proper CallbackContext from the typed update.
                    let mut ctx = CallbackContext::from_update(&update, bot, ud, cd, bd);
                    if let Some(jq) = jq {
                        ctx = ctx.with_job_queue(jq);
                    }

                    // Let the handler populate additional context (args, matches, etc.)
                    h.collect_additional_context(&mut ctx, &match_result);

                    // Call the context-aware handler.
                    match h
                        .handle_update_with_context(update, match_result, ctx)
                        .await
                    {
                        TraitHandlerResult::Continue => Ok(()),
                        TraitHandlerResult::Stop => Err(HandlerError::HandlerStop { state: None }),
                        TraitHandlerResult::Error(e) => Err(HandlerError::Other(e)),
                    }
                }) as Pin<Box<dyn Future<Output = Result<(), HandlerError>> + Send>>
            }),
            block: handler.block(),
        };

        self.add_handler(legacy, group).await;
    }

    // -- Core dispatch --
    pub async fn process_update(&self, update: Arc<Update>) -> Result<(), ApplicationError> {
        self.check_initialized().await?;
        let mut context: Option<CallbackContext> = None;
        let groups: Vec<(i32, Vec<usize>)> = {
            let h = self.handlers.read().await;
            h.iter()
                .map(|(g, hs)| (*g, (0..hs.len()).collect()))
                .collect()
        };
        for (gid, indices) in &groups {
            let guard = self.handlers.read().await;
            let group = match guard.get(gid) {
                Some(g) => g,
                None => continue,
            };
            for &idx in indices {
                let handler = match group.get(idx) {
                    Some(h) => h,
                    None => continue,
                };
                if !(handler.check_update)(&update) {
                    continue;
                }
                if context.is_none() {
                    let mut ctx = CallbackContext::from_update(
                        &update,
                        Arc::clone(&self.bot),
                        Arc::clone(&self.user_data),
                        Arc::clone(&self.chat_data),
                        Arc::clone(&self.bot_data),
                    );
                    if let Some(ref jq) = self.job_queue {
                        ctx = ctx.with_job_queue(Arc::clone(jq));
                    }
                    context = Some(ctx);
                }
                let ctx = context.clone().unwrap();
                let cb = Arc::clone(&handler.callback);
                let uc = Arc::clone(&update);
                if handler.block {
                    match cb(uc, ctx).await {
                        Ok(()) => {}
                        Err(HandlerError::HandlerStop { .. }) => {
                            return Ok(());
                        }
                        Err(HandlerError::Other(e)) => {
                            if self.process_error(Some(Arc::clone(&update)), e).await {
                                return Ok(());
                            }
                        }
                    }
                } else {
                    let tasks = Arc::clone(&self.pending_tasks);
                    let handle = tokio::spawn(async move {
                        if let Err(e) = cb(uc, ctx).await {
                            warn!("Non-blocking handler error: {e}");
                        }
                    });
                    tasks.write().await.push(handle);
                }
                break;
            }
            drop(guard);
        }
        Ok(())
    }

    /// M9: error handlers can signal stop by returning `true`.
    pub async fn process_error(
        &self,
        update: Option<Arc<Update>>,
        error: Box<dyn std::error::Error + Send + Sync>,
    ) -> bool {
        let handlers = self.error_handlers.read().await;
        if handlers.is_empty() {
            error!("No error handlers registered: {error}");
            return false;
        }
        let error_arc: Arc<dyn std::error::Error + Send + Sync> = Arc::from(error);
        for (callback, block) in handlers.iter() {
            let mut ctx = CallbackContext::from_error(
                update.as_deref(),
                Arc::clone(&error_arc),
                Arc::clone(&self.bot),
                Arc::clone(&self.user_data),
                Arc::clone(&self.chat_data),
                Arc::clone(&self.bot_data),
            );
            if let Some(ref jq) = self.job_queue {
                ctx = ctx.with_job_queue(Arc::clone(jq));
            }
            if *block {
                if callback(update.clone(), ctx).await {
                    return true;
                }
            } else {
                let cb = Arc::clone(callback);
                let upd = update.clone();
                tokio::spawn(async move {
                    cb(upd, ctx).await;
                });
            }
        }
        false
    }

    // -- Data management --
    pub async fn drop_chat_data(&self, chat_id: i64) {
        self.chat_data.write().await.remove(&chat_id);
    }
    pub async fn drop_user_data(&self, user_id: i64) {
        self.user_data.write().await.remove(&user_id);
    }
    pub async fn migrate_chat_data(&self, old: i64, new: i64) {
        let mut s = self.chat_data.write().await;
        if let Some(d) = s.remove(&old) {
            s.insert(new, d);
        }
    }

    async fn check_initialized(&self) -> Result<(), ApplicationError> {
        if !*self.initialized.read().await {
            return Err(ApplicationError::NotInitialized);
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// PollingBuilder
// ---------------------------------------------------------------------------

/// Builder for configuring and starting the polling loop.
///
/// Obtained via [`Application::polling()`]. Provides a clean API without
/// positional `None` arguments.
///
/// # Example
///
/// ```rust,ignore
/// use std::time::Duration;
///
/// // Minimal -- all defaults:
/// app.run_polling().await?;
///
/// // Custom configuration via builder:
/// app.polling()
///     .timeout(Duration::from_secs(30))
///     .poll_interval(Duration::from_secs(1))
///     .drop_pending(true)
///     .allowed_updates(vec!["message".into()])
///     .start()
///     .await?;
/// ```
#[derive(Debug)]
pub struct PollingBuilder {
    app: Arc<Application>,
    poll_interval: std::time::Duration,
    timeout: std::time::Duration,
    allowed_updates: Option<Vec<String>>,
    drop_pending_updates: bool,
}

impl PollingBuilder {
    fn new(app: Arc<Application>) -> Self {
        Self {
            app,
            poll_interval: std::time::Duration::ZERO,
            timeout: std::time::Duration::from_secs(10),
            allowed_updates: None,
            drop_pending_updates: false,
        }
    }

    /// Sets the interval between polling requests. Default: zero (no delay).
    #[must_use]
    pub fn poll_interval(mut self, interval: std::time::Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    /// Sets the long-polling timeout. Default: 10 seconds.
    #[must_use]
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the list of update types to receive. Default: all types.
    #[must_use]
    pub fn allowed_updates(mut self, updates: Vec<String>) -> Self {
        self.allowed_updates = Some(updates);
        self
    }

    /// If `true`, pending updates will be dropped on startup. Default: `false`.
    #[must_use]
    pub fn drop_pending(mut self, drop: bool) -> Self {
        self.drop_pending_updates = drop;
        self
    }

    /// Starts the polling loop with the configured parameters.
    pub async fn start(self) -> Result<(), ApplicationError> {
        self.app
            .run_polling_configured(
                self.poll_interval,
                self.timeout,
                self.allowed_updates,
                self.drop_pending_updates,
            )
            .await
    }
}

async fn futures_join_all(handles: Vec<tokio::task::JoinHandle<()>>) {
    for h in handles {
        let _ = h.await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ext_bot::test_support::mock_request;
    use telegram_bot_raw::bot::Bot;

    fn make_app() -> Arc<Application> {
        let bot = Bot::new("test_token", mock_request());
        let ext_bot = Arc::new(ExtBot::from_bot(bot));
        let processor = Arc::new(crate::update_processor::simple_processor(1).unwrap());
        Application::new(ApplicationConfig::new(
            ext_bot,
            ContextTypes::default(),
            processor,
        ))
    }

    fn make_update(json_val: serde_json::Value) -> Update {
        serde_json::from_value(json_val).unwrap()
    }

    #[tokio::test]
    async fn initialize_and_shutdown() {
        let app = make_app();
        assert!(!app.is_initialized().await);
        app.initialize().await.unwrap();
        assert!(app.is_initialized().await);
        app.initialize().await.unwrap();
        app.shutdown().await.unwrap();
        assert!(!app.is_initialized().await);
    }

    #[tokio::test]
    async fn shutdown_while_running_errors() {
        let app = make_app();
        app.initialize().await.unwrap();
        app.start().await.unwrap();
        assert!(app.shutdown().await.is_err());
        app.stop().await.unwrap();
        app.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn add_and_process_handler() {
        let app = make_app();
        app.initialize().await.unwrap();
        let called = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let c2 = called.clone();
        app.add_handler(
            Handler {
                check_update: Arc::new(|u| u.message.is_some()),
                callback: Arc::new(move |_, _| {
                    let c = c2.clone();
                    Box::pin(async move {
                        c.store(true, std::sync::atomic::Ordering::SeqCst);
                        Ok(())
                    })
                }),
                block: true,
            },
            DEFAULT_GROUP,
        )
        .await;
        app.process_update(Arc::new(make_update(serde_json::json!({"update_id":1,"message":{"message_id":1,"date":0,"chat":{"id":1,"type":"private"},"text":"hello"}})))).await.unwrap();
        assert!(called.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[tokio::test]
    async fn handler_groups_priority() {
        let app = make_app();
        app.initialize().await.unwrap();
        let order = Arc::new(RwLock::new(Vec::new()));
        let o1 = order.clone();
        app.add_handler(
            Handler {
                check_update: Arc::new(|_| true),
                callback: Arc::new(move |_, _| {
                    let o = o1.clone();
                    Box::pin(async move {
                        o.write().await.push(1);
                        Ok(())
                    })
                }),
                block: true,
            },
            1,
        )
        .await;
        let o0 = order.clone();
        app.add_handler(
            Handler {
                check_update: Arc::new(|_| true),
                callback: Arc::new(move |_, _| {
                    let o = o0.clone();
                    Box::pin(async move {
                        o.write().await.push(0);
                        Ok(())
                    })
                }),
                block: true,
            },
            0,
        )
        .await;
        app.process_update(Arc::new(make_update(serde_json::json!({"update_id":1,"message":{"message_id":1,"date":0,"chat":{"id":1,"type":"private"}}})))).await.unwrap();
        assert_eq!(*order.read().await, vec![0, 1]);
    }

    #[tokio::test]
    async fn handler_stop_prevents_further_groups() {
        let app = make_app();
        app.initialize().await.unwrap();
        let reached = Arc::new(std::sync::atomic::AtomicBool::new(false));
        app.add_handler(
            Handler {
                check_update: Arc::new(|_| true),
                callback: Arc::new(|_, _| {
                    Box::pin(async { Err(HandlerError::HandlerStop { state: None }) })
                }),
                block: true,
            },
            0,
        )
        .await;
        let r = reached.clone();
        app.add_handler(
            Handler {
                check_update: Arc::new(|_| true),
                callback: Arc::new(move |_, _| {
                    let r = r.clone();
                    Box::pin(async move {
                        r.store(true, std::sync::atomic::Ordering::SeqCst);
                        Ok(())
                    })
                }),
                block: true,
            },
            1,
        )
        .await;
        app.process_update(Arc::new(make_update(serde_json::json!({"update_id":1}))))
            .await
            .unwrap();
        assert!(!reached.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[tokio::test]
    async fn only_first_matching_handler_per_group() {
        let app = make_app();
        app.initialize().await.unwrap();
        let first = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let second = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let f = first.clone();
        app.add_handler(
            Handler {
                check_update: Arc::new(|_| true),
                callback: Arc::new(move |_, _| {
                    let f = f.clone();
                    Box::pin(async move {
                        f.store(true, std::sync::atomic::Ordering::SeqCst);
                        Ok(())
                    })
                }),
                block: true,
            },
            0,
        )
        .await;
        let s = second.clone();
        app.add_handler(
            Handler {
                check_update: Arc::new(|_| true),
                callback: Arc::new(move |_, _| {
                    let s = s.clone();
                    Box::pin(async move {
                        s.store(true, std::sync::atomic::Ordering::SeqCst);
                        Ok(())
                    })
                }),
                block: true,
            },
            0,
        )
        .await;
        app.process_update(Arc::new(make_update(serde_json::json!({"update_id":1}))))
            .await
            .unwrap();
        assert!(first.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!second.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[tokio::test]
    async fn error_handler_called_on_failure() {
        let app = make_app();
        app.initialize().await.unwrap();
        app.add_handler(
            Handler {
                check_update: Arc::new(|_| true),
                callback: Arc::new(|_, _| {
                    Box::pin(async {
                        Err(HandlerError::Other(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "test",
                        ))))
                    })
                }),
                block: true,
            },
            0,
        )
        .await;
        let seen = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let s = seen.clone();
        let eh: ErrorHandlerCallback = Arc::new(move |_, ctx| {
            let s = s.clone();
            Box::pin(async move {
                s.store(true, std::sync::atomic::Ordering::SeqCst);
                assert!(ctx.error.is_some());
                false
            })
        });
        app.add_error_handler(eh, true).await;
        app.process_update(Arc::new(make_update(serde_json::json!({"update_id":1}))))
            .await
            .unwrap();
        assert!(seen.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[tokio::test]
    async fn error_handler_can_signal_stop() {
        let app = make_app();
        app.initialize().await.unwrap();
        app.add_handler(
            Handler {
                check_update: Arc::new(|_| true),
                callback: Arc::new(|_, _| {
                    Box::pin(async {
                        Err(HandlerError::Other(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "e",
                        ))))
                    })
                }),
                block: true,
            },
            0,
        )
        .await;
        let eh: ErrorHandlerCallback = Arc::new(|_, _| Box::pin(async { true }));
        let reached = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let r = reached.clone();
        app.add_handler(
            Handler {
                check_update: Arc::new(|_| true),
                callback: Arc::new(move |_, _| {
                    let r = r.clone();
                    Box::pin(async move {
                        r.store(true, std::sync::atomic::Ordering::SeqCst);
                        Ok(())
                    })
                }),
                block: true,
            },
            1,
        )
        .await;
        app.add_error_handler(eh, true).await;
        app.process_update(Arc::new(make_update(serde_json::json!({"update_id":1}))))
            .await
            .unwrap();
        assert!(!reached.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[tokio::test]
    async fn process_update_before_initialize_fails() {
        let app = make_app();
        assert!(app
            .process_update(Arc::new(make_update(serde_json::json!({"update_id": 0}))))
            .await
            .is_err());
    }

    #[tokio::test]
    async fn drop_chat_and_user_data() {
        let app = make_app();
        {
            app.chat_data.write().await.insert(42, HashMap::new());
        }
        {
            app.user_data.write().await.insert(7, HashMap::new());
        }
        app.drop_chat_data(42).await;
        app.drop_user_data(7).await;
        assert!(app.chat_data.read().await.get(&42).is_none());
        assert!(app.user_data.read().await.get(&7).is_none());
    }

    #[tokio::test]
    async fn migrate_chat_data() {
        let app = make_app();
        {
            let mut s = app.chat_data.write().await;
            let mut d = HashMap::new();
            d.insert("key".into(), Value::String("val".into()));
            s.insert(100, d);
        }
        app.migrate_chat_data(100, 200).await;
        let s = app.chat_data.read().await;
        assert!(s.get(&100).is_none());
        assert_eq!(
            s.get(&200).unwrap().get("key"),
            Some(&Value::String("val".into()))
        );
    }

    #[tokio::test]
    async fn update_sender_works() {
        let app = make_app();
        assert!(app
            .update_sender()
            .send(Arc::new(make_update(serde_json::json!({"update_id":1}))))
            .is_ok());
    }

    #[tokio::test]
    async fn job_queue_accessor() {
        let app = make_app();
        assert!(app.job_queue().is_none());
    }

    #[tokio::test]
    async fn create_task_tracks_handle() {
        let app = make_app();
        let flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let f = flag.clone();
        app.create_task(async move {
            f.store(true, std::sync::atomic::Ordering::SeqCst);
        })
        .await;
        // Give the spawned task a moment to run.
        tokio::task::yield_now().await;
        assert!(flag.load(std::sync::atomic::Ordering::SeqCst));
        assert_eq!(app.pending_tasks.read().await.len(), 1);
    }
}
