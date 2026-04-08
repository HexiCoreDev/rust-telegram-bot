//! Convenient re-exports for writing clean Telegram bot code.
//!
//! ```rust,ignore
//! use telegram_bot_ext::prelude::*;
//! ```

pub use crate::application::{Application, HandlerError};
pub use crate::builder::ApplicationBuilder;
pub use crate::context::{CallbackContext, DataReadGuard, DataWriteGuard};
pub use crate::filters::base::{Filter, FilterResult, F};
pub use crate::handlers::base::FnHandler;
pub use crate::handlers::callback_query::CallbackQueryHandler;
pub use crate::handlers::command::CommandHandler;
pub use crate::handlers::message::MessageHandler;
pub use std::sync::Arc;

pub use telegram_bot_raw::types::message::Message;
pub use telegram_bot_raw::types::update::Update;

// Typed constant enums — prefer these over magic strings.
pub use telegram_bot_raw::constants::{
    ChatAction, ChatMemberStatus, ChatType, MessageEntityType, ParseMode,
};

// ---------------------------------------------------------------------------
// Keyboard & inline keyboard constructors (D1-D4)
// ---------------------------------------------------------------------------
pub use telegram_bot_raw::types::inline::inline_keyboard_button::InlineKeyboardButton;
pub use telegram_bot_raw::types::inline::inline_keyboard_markup::InlineKeyboardMarkup;
pub use telegram_bot_raw::types::keyboard_button::KeyboardButton;
pub use telegram_bot_raw::types::reply_keyboard_markup::ReplyKeyboardMarkup;
pub use telegram_bot_raw::types::reply_keyboard_remove::ReplyKeyboardRemove;
pub use telegram_bot_raw::types::force_reply::ForceReply;

// ---------------------------------------------------------------------------
// Common types developers always need
// ---------------------------------------------------------------------------
pub use telegram_bot_raw::types::user::User;
pub use telegram_bot_raw::types::chat::Chat;
pub use telegram_bot_raw::types::callback_query::CallbackQuery;
pub use telegram_bot_raw::types::files::input_file::InputFile;
pub use telegram_bot_raw::types::files::photo_size::PhotoSize;
pub use telegram_bot_raw::bot::ChatId;

// ---------------------------------------------------------------------------
// Re-export serde_json so users don't need it as a direct dependency
// ---------------------------------------------------------------------------
pub use serde_json::{json, Value as JsonValue};

// ---------------------------------------------------------------------------
// Re-export commonly needed async / collection types
// ---------------------------------------------------------------------------
pub use tokio::sync::RwLock;
pub use std::collections::HashMap;

// Re-export tokio so users can use #[tokio::main] without adding tokio as direct dep
pub use tokio;

// ---------------------------------------------------------------------------
// Webhook types (feature-gated)
// ---------------------------------------------------------------------------
#[cfg(feature = "webhooks")]
pub use crate::utils::webhook_handler::{WebhookHandler, WebhookServer};
#[cfg(feature = "webhooks")]
pub use crate::updater::WebhookConfig;

/// Type alias matching python-telegram-bot's `HandlerResult`.
pub type HandlerResult = Result<(), HandlerError>;

/// Type alias for ergonomic handler signatures: `async fn(Update, Context) -> HandlerResult`.
pub type Context = CallbackContext;

/// Returns the `TEXT` filter wrapped as [`F`] for use with bitwise operators.
///
/// ```rust,ignore
/// MessageHandler::new(text_filter() & !command_filter(), echo)
/// ```
#[allow(non_snake_case)]
#[must_use]
pub fn TEXT() -> F {
    F::new(crate::filters::text::TextAny)
}

/// Returns the `COMMAND` filter wrapped as [`F`] for use with bitwise operators.
///
/// ```rust,ignore
/// MessageHandler::new(text_filter() & !command_filter(), echo)
/// ```
#[allow(non_snake_case)]
#[must_use]
pub fn COMMAND() -> F {
    F::new(crate::filters::command::CommandFilter::starts())
}
