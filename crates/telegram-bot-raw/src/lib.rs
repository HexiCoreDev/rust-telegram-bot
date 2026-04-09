//! Low-level Telegram Bot API types and methods.
//!
//! This crate provides a faithful Rust port of the core Telegram Bot API layer,
//! including all request/response types, the [`Bot`](bot::Bot) client, and
//! helper utilities.  It is designed to be used directly for low-level API
//! access or as the foundation for the higher-level `rust-tg-bot-ext` framework.

#![warn(missing_docs)]
#![forbid(unsafe_code)]

/// Internal macros for reducing boilerplate.
#[macro_use]
mod macros;

/// The Telegram Bot client and API method implementations.
pub mod bot;
/// Builder types for constructing a [`Bot`](bot::Bot) with custom configuration.
pub mod bot_builders;
/// Typed constants for chat types, parse modes, message entity types, etc.
pub mod constants;
/// Error types used throughout the crate.
pub mod error;
/// Helper functions for common Telegram API patterns.
pub mod helpers;
/// HTTP request abstraction layer.
pub mod request;
/// All Telegram API object types (User, Chat, Message, Update, etc.).
pub mod types;
/// Utility modules for date/time handling, entity parsing, markup, and more.
pub mod utils;

// Re-export serde_json so downstream crates (including tests and examples)
// can use `rust_tg_bot_raw::serde_json` without adding it as a direct dependency.
pub use serde_json;
