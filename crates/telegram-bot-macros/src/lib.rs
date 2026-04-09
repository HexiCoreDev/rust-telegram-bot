//! Procedural macros for the `rust-tg-bot` Telegram Bot framework.
//!
//! This crate provides the [`BotCommands`] derive macro, which turns an enum
//! into a self-parsing command handler with automatic help-text generation and
//! Telegram `setMyCommands` integration.
//!
//! # Example
//!
//! ```rust,ignore
//! use rust_tg_bot_macros::BotCommands;
//!
//! #[derive(BotCommands, Clone)]
//! #[command(rename_rule = "lowercase")]
//! enum Command {
//!     #[command(description = "Display help text")]
//!     Help,
//!     #[command(description = "Start the bot")]
//!     Start,
//!     #[command(description = "Set username")]
//!     Username(String),
//!     #[command(description = "Set age")]
//!     Age(u32),
//! }
//! ```

extern crate proc_macro;

mod bot_commands;
mod command;
mod command_attr;
mod command_enum;
mod error;
mod fields_parse;
mod rename_rules;

pub(crate) use error::{compile_error, Result};
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::bot_commands::bot_commands_impl;

/// Derive macro that generates command-parsing infrastructure for a Telegram bot.
///
/// Annotate an enum with `#[derive(BotCommands)]` to automatically generate:
///
/// - `parse(text, bot_name) -> Result<Self, ParseError>` -- parse incoming message text
/// - `descriptions() -> String` -- formatted help text for all commands
/// - `bot_commands() -> Vec<BotCommand>` -- suitable for the `setMyCommands` API call
///
/// # Enum-level attributes
///
/// | Attribute | Default | Description |
/// |-----------|---------|-------------|
/// | `rename_rule` | `"identity"` | How variant names map to command strings |
/// | `prefix` | `"/"` | Command prefix character(s) |
/// | `description` | none | Global description header for help text |
/// | `command_separator` | `" "` | Separator between the command and its arguments |
///
/// # Variant-level attributes
///
/// | Attribute | Description |
/// |-----------|-------------|
/// | `description` | Help text for this command |
/// | `rename` | Override the command string for this variant |
/// | `parse_with` | `"default"`, `"split"`, or a custom `fn(String) -> Result<T, E>` |
/// | `separator` | Argument separator when using `parse_with = "split"` |
/// | `hide` | Exclude from help text and `bot_commands()` |
#[proc_macro_derive(BotCommands, attributes(command))]
pub fn bot_commands_derive(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    bot_commands_impl(input)
        .unwrap_or_else(<_>::into)
        .into()
}
