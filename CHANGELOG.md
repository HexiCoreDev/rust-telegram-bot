# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0-beta.4c] - 2026-04-10

### Fixed
- docs.rs build failure: replaced `include_str!("../../../README.md")` with inline crate docs (path doesn't resolve in isolated crate builds)
- `make bump` now handles shields.io badge URL encoding (`--` for `-`)

## [1.0.0-beta.4b] - 2026-04-10

### Fixed
- Switch `reqwest` from `native-tls` to `rustls-tls` — removes `openssl-sys` dependency entirely, fixes aarch64 cross-compilation failure in release workflow
- Inter-crate path dependencies now include `version` for crates.io publish
- Publish workflow handles "already exists" gracefully (`|| echo "Already published, skipping"`)
- GitHub Actions upgraded to latest: checkout v6, upload-artifact v7, download-artifact v8, codecov-action v6

### Changed
- README logo uses Vercel-hosted URL instead of local `assets/logo.png`

## [1.0.0-beta.4] - 2026-04-09

### Added
- Comprehensive doc comments on all 1700+ public items — enums, variants, structs, fields, methods, re-exports
- `#![forbid(unsafe_code)]` enforced on all library crates
- Codecov integration with cargo-tarpaulin in Docker container
- Vercel deployment for mdBook documentation site
- Makefile with 40+ targets: build, test, lint, coverage, examples, benchmarks, release
- CI coverage job using `xd009642/tarpaulin:develop-nightly` + `codecov-action@v6`
- mdBook docs auto-deploy to Vercel on push to main

### Fixed
- CI: all workflows pass (check, test, clippy, fmt, examples, docs, coverage)
- CI: dynamic mdBook version download (was hardcoded to nonexistent v0.4.44)
- CI: `workflow_call` trigger for reusable workflow support in release pipeline
- Release workflow: proper conditions for tag vs main-push (build/release only on tags)
- `clippy::derivable_impls` on `UpdateKind` — now uses `#[derive(Default)]` with `#[default]`
- Unused import removed from macros.rs
- All rustfmt diffs resolved

### Changed
- README badges: comprehensive set matching PTB (CI, codecov, docs.rs, mdBook, rustfmt, clippy, unsafe-forbidden, MSRV, issue resolution)
- mdBook badge points to Vercel (`rust-tg-bot-docs.vercel.app`)
- `fail_ci_if_error: false` on coverage upload (non-blocking)
- `RUSTDOCFLAGS: -D warnings` removed from cargo doc CI job (docs use `#![warn(missing_docs)]` instead)

## [1.0.0-beta.3] - 2026-04-09

### Breaking Changes
- `#[non_exhaustive]` added to all 344+ public structs and enums — external crates must use constructors or serde deserialization instead of struct expressions
- Crate renamed from `telegram-bot` to `rust-tg-bot` (and similarly for sub-crates) to avoid crates.io name collision

### Added
- **`rust-tg-bot-macros` crate**: `#[derive(BotCommands)]` proc macro for declarative command handler registration with automatic help text generation
- **Redis persistence backend** (`persistence-redis` feature): production-ready persistence using the `redis` crate
- **PostgreSQL persistence backend** (`persistence-postgres` feature): JSONB-based persistence using `sqlx`
- **Webhook TLS auto-configuration** (`webhooks-tls` feature): automatic TLS termination via `tokio-rustls`
- **Rate limiter wired into request pipeline**: `RateLimitedRequest` adapter wrapping `BaseRequest` for transparent rate limiting
- Facade crate now forwards all feature flags: `persistence-redis`, `persistence-postgres`, `webhooks-tls`, `macros`
- `User::new()`, `CallbackQuery::new()` constructors for `#[non_exhaustive]` types
- `#[non_exhaustive]` on `UpdateKind` enum and 17 ext-crate enums (`ConversationState`, `JobStatus`, `PersistenceError`, etc.)

### Fixed
- CI workflow: corrected package name from `telegram-bot` to `rust-tg-bot` in examples job
- Release workflow: corrected all publish commands to use `rust-tg-bot-*` package names; added macros crate to publish order
- Test compilation: all test code updated to use constructors instead of struct expressions for `#[non_exhaustive]` types
- README: updated version references, architecture diagram, feature flags, and test count

## [1.0.0-beta.2] - 2026-04-08

### Breaking Changes
- Removed `extra: HashMap<String, Value>` from all 281 types (unknown fields now silently dropped)
- `Update` is now an enum-based `UpdateKind` (not flat struct with 27 Option fields)
- Message large fields boxed (game, poll, venue, invoice, etc.)
- `Option<bool>` fields on Message changed to `bool` with `#[serde(default)]`
- Handler callbacks receive `Arc<Update>` (was `Update`)
- `rust_tg_bot::run()` deprecated — use `#[tokio::main]` directly

### Performance
- Idle memory: 20 → 15 MB (-25%) — matches teloxide
- Load memory: 32 → 17 MB (-47%) — matches teloxide
- Binary size: 12 → 6.2 MB (-48%) — smaller than teloxide (6.6 MB)
- RTB now uses 3.5x less memory than PTB (15 MB vs 57 MB)
- Connection pool: 256 → 8 connections
- Direct serialization for 21 text-only builders (no double serde pass)
- Bounded update channel (capacity 64) replacing unbounded
- AtomicBool for flags, OnceCell for cached bot data
- Selective tokio features (not "full")
- All filters use typed Update access (zero serde_json::to_value)
- Arc<str> for token/URLs, Arc<Update> in dispatch
- LTO, `codegen-units = 1`, `strip = true`, `opt-level = "z"` in release profile

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
- All examples use #[tokio::main] (not rust_tg_bot::run())

## [1.0.0-beta] - 2026-04-07

### Added

- Complete port of python-telegram-bot's architecture to Rust
- Full Telegram Bot API 9.6 coverage (169 methods, 301 types)
- Three-crate workspace: `rust-tg-bot-raw`, `rust-tg-bot-ext`, `rust-tg-bot`
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
- `rust_tg_bot::run()` helper for correct async runtime configuration
- `Defaults` builder for default `parse_mode`, `disable_notification`, etc.
- `prelude` module for ergonomic imports
- 20 example bots matching python-telegram-bot's full example set
- Comprehensive documentation: README, 8 guide docs, zero-to-hero tutorial
- `From<ParseMode> for String` and `PartialEq<MessageEntityType> for String` for seamless constant usage

### Architecture

- **rust-tg-bot-raw**: Pure API types + Bot methods. No framework opinions. Depends only on serde, reqwest, tokio.
- **rust-tg-bot-ext**: Application framework. Handlers, filters, persistence, job queue.
- **rust-tg-bot**: Umbrella crate re-exporting both.

### Heritage

Faithfully ported from [python-telegram-bot](https://github.com/python-telegram-bot/python-telegram-bot) v22.x (Bot API 9.5), then upgraded to Bot API 9.6 and adapted to idiomatic Rust patterns.
