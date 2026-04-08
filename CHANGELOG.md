# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0-beta.2] - 2026-04-08

### Breaking Changes
- Removed `extra: HashMap<String, Value>` from all 281 types (unknown fields now silently dropped)
- `Update` is now an enum-based `UpdateKind` (not flat struct with 27 Option fields)
- Message large fields boxed (game, poll, venue, invoice, etc.)
- `Option<bool>` fields on Message changed to `bool` with `#[serde(default)]`
- Handler callbacks receive `Arc<Update>` (was `Update`)
- `telegram_bot::run()` deprecated — use `#[tokio::main]` directly

### Performance
- Idle memory: 20 → 17 MB (-15%)
- Load memory: 32 → 20 MB (-37%)
- Binary size: 12 → 9.6 MB (-20%)
- Connection pool: 256 → 8 connections
- Direct serialization for 21 text-only builders (no double serde pass)
- Bounded update channel (capacity 64) replacing unbounded
- AtomicBool for flags, OnceCell for cached bot data
- Selective tokio features (not "full")
- All filters use typed Update access (zero serde_json::to_value)
- Arc<str> for token/URLs, Arc<Update> in dispatch

### Added
- 90+ type constructors (InlineKeyboardButton::callback(), User::full_name(), etc.)
- 168 Bot API method builders with IntoFuture (directly awaitable)
- Context shortcuts: reply_html, reply_photo, reply_document, reply_sticker, reply_location
- context.answer_callback_query(), context.edit_callback_message_text()
- Expanded prelude re-exporting serde_json, tokio, keyboards, common types
- Feature-gated modules: job-queue, persistence, rate-limiter
- Optimized webhook server: constant-time secret, TCP_NODELAY, backpressure (503)
- 25 roundtrip serialization tests, 9 proptest filter tests, 10 persistence stress tests
- bot.rs split into 22 per-method-group submodules
- #![warn(missing_docs)] on both crates
- proptest fuzzing for filter composition algebra

### Fixed
- Specific imports replacing all wildcard `*` imports in library code
- All examples use typed constructors (no json!() for API types)
- All examples use specific prelude imports (no prelude::*)
- All examples use #[tokio::main] (not telegram_bot::run())

## [1.0.0-beta] - 2026-04-07

### Added

- Complete port of python-telegram-bot's architecture to Rust
- Full Telegram Bot API 9.6 coverage (169 methods, 301 types)
- Three-crate workspace: `telegram-bot-raw`, `telegram-bot-ext`, `telegram-bot`
- 29 builder structs with `IntoFuture` (directly awaitable, no `.send()`)
- 94 typed constant enums (`ParseMode`, `ChatType`, `MessageEntityType`, etc.)
- Typed `Update` struct with `effective_user()`, `effective_chat()`, `effective_message()` helpers
- 22 handler types: `CommandHandler`, `MessageHandler`, `CallbackQueryHandler`, `ConversationHandler`, `FnHandler`, and 17 more
- `FnHandler` with convenience constructors: `on_callback_query`, `on_poll`, `on_inline_query`, `on_any`, etc.
- 44+ composable filters with `&`, `|`, `!`, `^` operators and `FilterResult` data passthrough
- `Application` with handler group dispatch, lifecycle management, graceful shutdown
- Typestate `ApplicationBuilder` with `PollingBuilder` and webhook support
- `CallbackContext` with `reply_text()`, typed `DataReadGuard`/`DataWriteGuard`
- Persistence: `BasePersistence` trait + `DictPersistence`, `JsonFilePersistence`, `SqlitePersistence`
- `JobQueue`: tokio-native scheduling (`once`, `repeating`, `daily`, `monthly`, `custom`) with persistence flush hooks and error routing
- Token bucket rate limiter
- `Updater` with polling (offset tracking, long-poll timeout) and webhook (axum) support
- `ExtBot` with `Deref<Target = Bot>` for zero-cost method access
- `telegram_bot::run()` helper for correct async runtime configuration
- `Defaults` builder for default `parse_mode`, `disable_notification`, etc.
- `prelude` module for ergonomic imports
- 20 example bots matching python-telegram-bot's full example set
- Comprehensive documentation: README, 8 guide docs, zero-to-hero tutorial
- `From<ParseMode> for String` and `PartialEq<MessageEntityType> for String` for seamless constant usage

### Architecture

- **telegram-bot-raw**: Pure API types + Bot methods. No framework opinions. Depends only on serde, reqwest, tokio.
- **telegram-bot-ext**: Application framework. Handlers, filters, persistence, job queue.
- **telegram-bot**: Umbrella crate re-exporting both.

### Heritage

Faithfully ported from [python-telegram-bot](https://github.com/python-telegram-bot/python-telegram-bot) v22.x (Bot API 9.5), then upgraded to Bot API 9.6 and adapted to idiomatic Rust patterns.
