// Re-export everything from both crates for convenience
pub use telegram_bot_raw as raw;
pub use telegram_bot_raw::bot::Bot;
pub use telegram_bot_raw::error;
pub use telegram_bot_raw::types;

pub use telegram_bot_ext as ext;

/// Run an async entry point with a tokio runtime configured for Telegram bot workloads.
///
/// The Telegram Bot API call chain produces deeply nested async state machines.
/// This helper builds a multi-threaded tokio runtime with enough worker thread
/// stack space to handle them safely.
///
/// # Example
///
/// ```rust,ignore
/// fn main() {
///     telegram_bot::run(async {
///         let app = ApplicationBuilder::new().token(token).build();
///         app.run_polling().await.unwrap();
///     });
/// }
/// ```
pub fn run<F: std::future::Future<Output = ()> + Send>(future: F) {
    tokio::runtime::Builder::new_multi_thread()
        .thread_stack_size(8 * 1024 * 1024)
        .enable_all()
        .build()
        .expect("failed to build tokio runtime")
        .block_on(future);
}
