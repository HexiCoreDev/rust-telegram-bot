#![doc = include_str!("../../../README.md")]

// Re-export everything from both crates for convenience
pub use rust_tg_bot_raw as raw;
pub use rust_tg_bot_raw::bot::Bot;
pub use rust_tg_bot_raw::error;
pub use rust_tg_bot_raw::types;

pub use rust_tg_bot_ext as ext;
pub use rust_tg_bot_ext::prelude;

/// Runtime configuration for [`run`].
pub struct RuntimeConfig {
    /// Number of tokio worker threads. `None` = system default (CPU core count).
    pub worker_threads: Option<usize>,
    /// Stack size per worker thread in bytes. Default: 8 MB.
    pub thread_stack_size: usize,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            worker_threads: None,
            thread_stack_size: 8 * 1024 * 1024,
        }
    }
}

impl RuntimeConfig {
    /// Set the number of worker threads.
    pub fn workers(mut self, n: usize) -> Self {
        self.worker_threads = Some(n);
        self
    }
    /// Set the stack size per worker thread in bytes.
    pub fn stack_size(mut self, bytes: usize) -> Self {
        self.thread_stack_size = bytes;
        self
    }
}

/// Run an async entry point with a tokio runtime configured for Telegram bot workloads.
///
/// Uses sensible defaults: all CPU cores as worker threads, 8 MB stack per thread
/// (needed for the deeply nested async state machines in Bot API calls).
///
/// # Basic usage
///
/// ```rust,ignore
/// fn main() {
///     rust_tg_bot::run(async {
///         let app = ApplicationBuilder::new().token(token).build();
///         app.run_polling().await.unwrap();
///     });
/// }
/// ```
///
/// # Custom configuration
///
/// ```rust,ignore
/// fn main() {
///     rust_tg_bot::run_configured(
///         RuntimeConfig::default().workers(4),
///         async { /* ... */ },
///     );
/// }
/// ```
#[deprecated(since = "1.0.0-beta.2", note = "Use #[tokio::main] directly instead")]
pub fn run<F: std::future::Future<Output = ()> + Send>(future: F) {
    run_configured(RuntimeConfig::default(), future);
}

/// Like [`run`], but with explicit [`RuntimeConfig`].
pub fn run_configured<F: std::future::Future<Output = ()> + Send>(
    config: RuntimeConfig,
    future: F,
) {
    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder
        .thread_stack_size(config.thread_stack_size)
        .enable_all();
    if let Some(n) = config.worker_threads {
        builder.worker_threads(n);
    }
    builder
        .build()
        .expect("failed to build tokio runtime")
        .block_on(future);
}
