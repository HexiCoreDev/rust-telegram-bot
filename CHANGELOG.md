# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
