//! Application framework for Telegram bots.
//!
//! `telegram-bot-ext` provides the high-level handler/filter/persistence layer
//! on top of [`telegram_bot_raw`].  It is modeled after Python's
//! `python-telegram-bot` extension package and offers:
//!
//! - **Handlers**: [`CommandHandler`](handlers::command::CommandHandler),
//!   [`MessageHandler`](handlers::message::MessageHandler),
//!   [`CallbackQueryHandler`](handlers::callback_query::CallbackQueryHandler), etc.
//! - **Filters**: composable with `&`, `|`, `^`, `!` operators via [`F`](filters::base::F).
//! - **Persistence** *(feature `persistence`)*: in-memory
//!   ([`DictPersistence`](persistence::dict::DictPersistence)) and JSON-file
//!   ([`JsonFilePersistence`](persistence::json_file::JsonFilePersistence)).
//! - **Application**: the main dispatch loop ([`Application`](application::Application)).
//!
//! # Quick start
//!
//! ```rust,ignore
//! use telegram_bot_ext::prelude::*;
//!
//! async fn start(update: Update, context: Context) -> HandlerResult {
//!     context.reply_text(&update, "Hello!").await?;
//!     Ok(())
//! }
//!
//! let app = ApplicationBuilder::new("BOT_TOKEN").build().await;
//! app.add_typed_handler(CommandHandler::new("start", start), 0).await;
//! app.run_polling().await?;
//! ```

#![warn(missing_docs)]

/// The core [`Application`](application::Application) that dispatches updates
/// to registered handlers.
pub mod application;
/// Builder for constructing an [`Application`](application::Application) with
/// custom configuration.
pub mod builder;
/// Cache for arbitrary callback-data payloads attached to inline keyboards.
pub mod callback_data_cache;
/// The [`CallbackContext`](context::CallbackContext) passed to handler callbacks.
pub mod context;
/// Configuration types for context data categories.
pub mod context_types;
/// User-configurable defaults for outgoing API calls.
pub mod defaults;
/// Extended bot wrapping the raw [`Bot`](telegram_bot_raw::bot::Bot) with
/// defaults, callback-data cache, and rate-limiter support.
pub mod ext_bot;
/// Composable filters for routing updates to handlers.
pub mod filters;
/// Update handler trait and concrete handler implementations.
pub mod handlers;
/// Scheduled and repeating job execution.
///
/// Requires the `job-queue` feature.
#[cfg(feature = "job-queue")]
pub mod job_queue;
/// Persistence backends for storing user/chat/bot data across restarts.
///
/// Requires the `persistence` feature.
#[cfg(feature = "persistence")]
pub mod persistence;
/// Convenient re-exports for writing clean bot code.
pub mod prelude;
/// Rate-limiter support.
///
/// Requires the `rate-limiter` feature.
#[cfg(feature = "rate-limiter")]
pub mod rate_limiter;
/// Base update processor abstraction.
pub mod update_processor;
/// The polling/webhook updater that feeds updates into the application.
pub mod updater;
/// Internal utility types and helpers.
pub mod utils;
