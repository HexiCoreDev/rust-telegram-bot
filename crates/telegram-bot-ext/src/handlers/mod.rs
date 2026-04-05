//! Update handler framework.
//!
//! Each module provides a handler struct that implements the [`Handler`] trait
//! from [`base`]. Handlers are matched against incoming
//! [`Update`](telegram_bot_raw::types::update::Update) objects via
//! [`Handler::check_update`](base::Handler::check_update), and dispatched via
//! [`Handler::handle_update`](base::Handler::handle_update).

pub mod base;
pub mod business_connection;
pub mod business_messages_deleted;
pub mod callback_query;
pub mod chat_boost;
pub mod chat_join_request;
pub mod chat_member;
pub mod chosen_inline_result;
pub mod command;
pub mod conversation;
pub mod inline_query;
pub mod message;
pub mod message_reaction;
pub mod paid_media_purchased;
pub mod poll;
pub mod poll_answer;
pub mod pre_checkout_query;
pub mod prefix;
pub mod shipping_query;
pub mod string_command;
pub mod string_regex;
pub mod type_handler;

// Re-export the core trait and types for convenience.
pub use base::{FnHandler, Handler, HandlerCallback, HandlerResult, MatchResult};
