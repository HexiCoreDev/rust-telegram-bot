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
