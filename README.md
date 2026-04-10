<p align="center">
  <img src="https://rust-tg-bot-docs.vercel.app/favicon.png" alt="rust-tg-bot" width="200">
</p>

<h1 align="center">rust-tg-bot</h1>

<p align="center"><strong>We built you a bot you can't refuse -- now in Rust.</strong></p>

<p align="center">

[![Version: 1.0.0-beta.5](https://img.shields.io/badge/version-1.0.0--beta.5-blueviolet)](https://github.com/HexiCoreDev/rust-telegram-bot/releases)
[![Bot API 9.6](https://img.shields.io/badge/Bot%20API-9.6-blue?logo=telegram)](https://core.telegram.org/bots/api-changelog)
[![Rust: 1.75+](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: LGPL-3.0](https://img.shields.io/badge/License-LGPL--3.0-green.svg)](https://www.gnu.org/licenses/lgpl-3.0.html)
[![CI](https://github.com/HexiCoreDev/rust-telegram-bot/actions/workflows/ci.yml/badge.svg?branch=dev)](https://github.com/HexiCoreDev/rust-telegram-bot/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/HexiCoreDev/rust-telegram-bot/graph/badge.svg?token=F2Y0CC4WNG)](https://codecov.io/gh/HexiCoreDev/rust-telegram-bot)
[![docs.rs](https://img.shields.io/badge/docs.rs-API-blue)](https://docs.rs/rust-tg-bot)
[![mdBook](https://img.shields.io/badge/book-guide-blue)](https://rust-tg-bot-docs.vercel.app/)
[![Code Style: rustfmt](https://img.shields.io/badge/code%20style-rustfmt-blue)](https://github.com/rust-lang/rustfmt)
[![Code Quality: clippy](https://img.shields.io/badge/code%20quality-clippy%20%E2%9C%93-brightgreen)](https://doc.rust-lang.org/clippy/)
[![Unsafe: forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![MSRV: 1.75](https://img.shields.io/badge/MSRV-1.75-orange)](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html)
[![Issues](https://isitmaintained.com/badge/resolution/HexiCoreDev/rust-telegram-bot.svg)](https://isitmaintained.com/project/HexiCoreDev/rust-telegram-bot)

</p>

---

> **WARNING: This project is in active development (v1.0.0-beta.5). The API is undergoing constant changes as we work toward full feature parity with python-telegram-bot and Rust-native performance optimizations. Use at your own risk -- breaking changes will occur between releases until v1.0.0 stable.**

---

A complete, asynchronous Telegram Bot API framework for Rust, faithfully ported from [python-telegram-bot](https://github.com/python-telegram-bot/python-telegram-bot) -- the most popular Python library for the Telegram Bot API.

This project carries forward the architecture, handler system, and developer experience that made python-telegram-bot the go-to choice for thousands of bot developers, and transplants it into Rust's async ecosystem with full type safety, zero-cost abstractions, and fearless concurrency.

## Heritage

This library stands on the shoulders of [python-telegram-bot](https://github.com/python-telegram-bot/python-telegram-bot). The handler hierarchy, filter composition system, `ConversationHandler` state machine, job queue design, and persistence architecture are all direct ports of their Python counterparts. If you've built bots with python-telegram-bot, you already know how this library works.

We acknowledge and thank the python-telegram-bot maintainers and community for over a decade of work that made this project possible.

## Why Rust?

| Concern | Python | Rust |
|---|---|---|
| **Performance** | GIL-limited concurrency, interpreted | True parallelism, compiled to native code |
| **Memory safety** | Runtime GC, potential leaks in long-running bots | Ownership system prevents leaks at compile time |
| **Type safety** | Optional type hints, runtime errors | Enforced at compile time, no `AttributeError` at 3 AM |
| **Deployment** | Requires Python runtime + virtualenv | Single static binary, 6.2 MB stripped |
| **Resource usage** | 57 MB RSS (measured) | 15 MB idle / 17 MB load (matches teloxide, see [benchmarks](benchmarks/)) |
| **Concurrency** | asyncio (single-threaded) | tokio (multi-threaded work-stealing) |

For bots that handle high volumes of updates, run on constrained hardware (VPS, Raspberry Pi, containers), or need to be deployed without a runtime, Rust is the right tool.

## Architecture

The library is organized as a Cargo workspace with four crates:

```text
rust-telegram-bot/
  crates/
    telegram-bot-raw/     # Pure API types and methods (like Python's `telegram` module)
    telegram-bot-ext/     # Application framework (like Python's `telegram.ext`)
    telegram-bot-macros/  # Proc macros (#[derive(BotCommands)])
    telegram-bot/         # Facade crate -- re-exports all three for convenience
```

**`rust-tg-bot-raw`** contains every type and method from Bot API 9.6: `Message`, `Update`, `User`, `Chat`, inline types, payments, passport, games, stickers, and all API methods on the `Bot` struct. It depends only on `serde`, `reqwest`, and `tokio`.

**`rust-tg-bot-ext`** provides the application framework: `ApplicationBuilder`, typed handler system, composable filters, `ConversationHandler`, `JobQueue`, persistence backends (JSON file, SQLite, Redis, PostgreSQL), rate limiting, webhook support with optional TLS, and callback data caching.

**`rust-tg-bot-macros`** provides the `#[derive(BotCommands)]` proc macro for declarative command handler registration.

**`rust-tg-bot`** is the facade crate you add to `Cargo.toml`. It re-exports everything from the other three crates under `rust_tg_bot::raw` and `rust_tg_bot::ext`.

## Quick Start

### 1. Create a bot with [@BotFather](https://t.me/BotFather)

Open Telegram, message `@BotFather`, send `/newbot`, and follow the prompts. Copy the token.

### 2. Add the dependency

```toml
[dependencies]
rust-tg-bot = { git = "https://github.com/HexiCoreDev/rust-telegram-bot" }
tracing-subscriber = "0.3"
```

### 3. Write your bot

```rust,no_run
use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, Arc, CommandHandler, Context, HandlerResult,
    MessageHandler, Update, COMMAND, TEXT,
};

async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let name = update
        .effective_user()
        .map(|u| u.first_name.as_str())
        .unwrap_or("there");
    context
        .reply_text(&update, &format!("Hi {name}! Send me any text and I will echo it back."))
        .await?;
    Ok(())
}

async fn echo(update: Arc<Update>, context: Context) -> HandlerResult {
    let text = update
        .effective_message()
        .and_then(|m| m.text.as_deref())
        .unwrap_or("");
    if !text.is_empty() {
        context.reply_text(&update, text).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

    let app = ApplicationBuilder::new().token(token).build();

    app.add_handler(CommandHandler::new("start", start), 0).await;
    app.add_handler(
        MessageHandler::new(TEXT() & !COMMAND(), echo),
        0,
    ).await;

    println!("Echo bot is running. Press Ctrl+C to stop.");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
```

### 4. Run it

```sh
TELEGRAM_BOT_TOKEN="123456:ABC-DEF" cargo run
```

## Feature Highlights

### Composable Filters

Filters use Rust's bitwise operators for composition -- the same mental model as python-telegram-bot, but enforced at compile time:

```rust,ignore
use rust_tg_bot::ext::prelude::{MessageHandler, TEXT, COMMAND};

// Text messages that are NOT commands
let text_only = TEXT() & !COMMAND();

// Use it with a handler
app.add_handler(
    MessageHandler::new(text_only, my_callback),
    0,
).await;
```

Over 50 built-in filters are available: `TEXT`, `COMMAND`, `PHOTO`, `VIDEO`, `AUDIO`, `VOICE`, `DOCUMENT`, `LOCATION`, `CONTACT`, `ANIMATION`, `STICKER`, `POLL`, `VENUE`, `GAME`, `INVOICE`, `FORWARDED`, `REPLY`, `PREMIUM_USER`, `FORUM`, chat type filters, entity filters, regex filters, and more.

### Typed Handler Registration

Handlers use strongly-typed constructors and are registered via `add_handler`:

```rust,ignore
use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, Arc, CommandHandler, Context, FnHandler,
    HandlerResult, MessageHandler, Update, COMMAND, TEXT,
};

// Command handler -- matches /start
app.add_handler(CommandHandler::new("start", start), 0).await;

// Message handler -- matches text that is not a command
app.add_handler(MessageHandler::new(TEXT() & !COMMAND(), echo), 0).await;

// Callback query handler -- matches inline keyboard button presses
app.add_handler(FnHandler::on_callback_query(button), 0).await;

// Generic function handler with custom predicate
app.add_handler(
    FnHandler::new(|u| u.callback_query.is_some(), my_handler),
    0,
).await;

// Catch-all handler (e.g., for logging)
app.add_handler(FnHandler::on_any(track_users), -1).await;
```

### Builder-based Bot API

Every Bot API method returns a builder with optional parameters as chainable setters. Builders implement `IntoFuture`, so you can `.await` directly:

```rust,ignore
use rust_tg_bot::ext::prelude::{Context, ParseMode};

// Simple -- .await directly
context.bot().send_message(chat_id, "Hello!").await?;

// With optional parameters
context
    .bot()
    .send_message(chat_id, "<b>Bold</b> text")
    .parse_mode(ParseMode::Html)
    .await?;

// Inline keyboard using typed constructors
use rust_tg_bot::ext::prelude::{InlineKeyboardButton, InlineKeyboardMarkup};

let keyboard = InlineKeyboardMarkup::new(vec![
    vec![InlineKeyboardButton::callback("Option 1", "1")],
]);

context
    .bot()
    .send_message(chat_id, "Choose:")
    .reply_markup(keyboard)
    .await?;

// Edit a message
context
    .bot()
    .edit_message_text("Updated text")
    .chat_id(chat_id)
    .message_id(msg_id)
    .await?;
```

### Typed Constants

No more magic strings. All enums are strongly typed:

```rust,ignore
use rust_tg_bot::ext::prelude::{
    ChatType, Context, MessageEntityType, ParseMode,
};

// Parse modes
context.bot().send_message(chat_id, text)
    .parse_mode(ParseMode::Html)
    .await?;

// Entity types
if entity.entity_type == MessageEntityType::BotCommand { /* ... */ }

// Chat types
if chat.chat_type != ChatType::Private { /* ... */ }
```

### Handler Groups

Handlers are organized into numbered groups. Within a group, the first matching handler wins. Across groups, processing continues unless a handler returns `HandlerResult::Stop`:

```rust,ignore
use rust_tg_bot::ext::prelude::{
    Arc, CommandHandler, Context, FnHandler, HandlerResult,
    MessageHandler, Update, COMMAND, TEXT,
};

// Group -1: always runs first (e.g., user tracking)
app.add_handler(FnHandler::on_any(track_users), -1).await;

// Group 0: command handlers
app.add_handler(CommandHandler::new("start", start), 0).await;
app.add_handler(CommandHandler::new("help", help), 0).await;

// Group 1: message handlers
app.add_handler(
    MessageHandler::new(TEXT() & !COMMAND(), echo),
    1,
).await;
```

### Typed Data Access

The `Context` provides typed read and write guards for bot-wide, per-user, and per-chat data:

```rust,ignore
use rust_tg_bot::ext::prelude::{Arc, Context, HandlerResult, Update};

async fn handle(update: Arc<Update>, context: Context) -> HandlerResult {
    // Read bot-wide data
    let bd = context.bot_data().await;
    let name = bd.get_str("bot_name");
    let count = bd.get_i64("total_messages");

    // Write bot-wide data with typed setters
    let mut bd = context.bot_data_mut().await;
    bd.set_str("last_user", "Alice");
    bd.set_i64("total_messages", count.unwrap_or(0) + 1);
    bd.add_to_id_set("user_ids", user_id);

    Ok(())
}
```

### ConversationHandler State Machine

Multi-step conversations with automatic state tracking, timeouts, nested conversations, and persistence:

```rust,ignore
use rust_tg_bot::ext::handlers::conversation::ConversationHandler;
use std::time::Duration;

#[derive(Clone, Hash, Eq, PartialEq)]
enum State { AskName, AskAge, AskBio }

let conv = ConversationHandler::builder()
    .entry_point(start_step)
    .state(State::AskName, vec![name_step])
    .state(State::AskAge, vec![age_step])
    .state(State::AskBio, vec![bio_step])
    .fallback(cancel_step)
    .conversation_timeout(Duration::from_secs(300))
    .persistent(true)
    .name("registration".to_string())
    .build();
```

Features ported from python-telegram-bot:
- Per-chat/per-user/per-message conversation keys
- Re-entry support
- `map_to_parent` for nested conversations
- Automatic timeout with configurable handlers
- Non-blocking callbacks with state revert on failure
- Persistence integration

### Webhook Support

The simplest way to run in webhook mode -- the framework handles the axum server, secret token validation, and update dispatching internally:

```rust,ignore
use rust_tg_bot::ext::updater::WebhookConfig;

let config = WebhookConfig::new("https://your.domain/telegram")
    .port(8000)
    .url_path("/telegram")
    .secret_token("my-secret-token");

app.run_webhook(config).await?;
```

See [`webhook_bot.rs`](crates/telegram-bot/examples/webhook_bot.rs) for a complete working example, or [`custom_webhook_bot.rs`](crates/telegram-bot/examples/custom_webhook_bot.rs) for adding custom routes alongside the webhook.

### Job Queue Scheduling

Schedule one-shot, repeating, daily, and monthly jobs using tokio timers and a builder pattern:

```rust,ignore
use rust_tg_bot::ext::job_queue::{JobQueue, JobCallbackFn, JobContext};
use std::sync::Arc;
use std::time::Duration;

let jq = Arc::new(JobQueue::new());

// One-shot: fire after 30 seconds
jq.once(callback, Duration::from_secs(30))
    .name("reminder")
    .chat_id(chat_id)
    .start()
    .await;

// Repeating: every 60 seconds
jq.repeating(callback, Duration::from_secs(60))
    .name("heartbeat")
    .start()
    .await;

// Daily at 09:00 UTC on weekdays
use chrono::NaiveTime;
let time = NaiveTime::from_hms_opt(9, 0, 0).unwrap();
jq.daily(callback, time, &[1, 2, 3, 4, 5])
    .name("morning_report")
    .start()
    .await;

// Monthly on the 1st at midnight
jq.monthly(callback, time, 1)
    .name("monthly_summary")
    .start()
    .await;
```

Every job returns a `Job` handle for cancellation, status checking, and enable/disable toggling.

### Persistence

Swap between backends without changing application code:

**JSON file** (human-readable, great for development):

```rust,ignore
use rust_tg_bot::ext::persistence::json_file::JsonFilePersistence;
use rust_tg_bot::ext::prelude::ApplicationBuilder;

let persistence = JsonFilePersistence::new("bot_data", true, false);
let app = ApplicationBuilder::new()
    .token(token)
    .persistence(Box::new(persistence))
    .build();
```

**SQLite** (production-ready, WAL mode, atomic writes):

```rust,ignore
use rust_tg_bot::ext::persistence::sqlite::SqlitePersistence;
use rust_tg_bot::ext::prelude::ApplicationBuilder;

let persistence = SqlitePersistence::open("bot.db").unwrap();
let app = ApplicationBuilder::new()
    .token(token)
    .persistence(Box::new(persistence))
    .build();
```

**Custom backend** -- implement the `BasePersistence` trait:

```rust,ignore
use rust_tg_bot::ext::persistence::base::BasePersistence;
use std::collections::HashMap;

#[derive(Debug)]
struct RedisPersistence { /* ... */ }

impl BasePersistence for RedisPersistence {
    async fn get_user_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> { /* ... */ }
    async fn get_chat_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> { /* ... */ }
    // ... implement all trait methods
}
```

## Feature Flags

```toml
[dependencies]
# Default: polling support only
rust-tg-bot = { git = "https://github.com/HexiCoreDev/rust-telegram-bot" }

# Everything
rust-tg-bot = { git = "https://github.com/HexiCoreDev/rust-telegram-bot", features = ["full"] }

# Pick what you need
rust-tg-bot = { git = "https://github.com/HexiCoreDev/rust-telegram-bot", features = [
    "webhooks",              # axum-based webhook server
    "webhooks-tls",          # TLS auto-configuration for webhooks
    "job-queue",             # Scheduled job execution
    "persistence-json",      # JSON file persistence
    "persistence-sqlite",    # SQLite persistence
    "persistence-redis",     # Redis persistence
    "persistence-postgres",  # PostgreSQL persistence (JSONB)
    "rate-limiter",          # API rate limiting
    "macros",                # #[derive(BotCommands)]
] }
```

## Comparison

| Feature | rust-tg-bot | python-telegram-bot | teloxide |
|---|:---:|:---:|:---:|
| Bot API version | **9.6** | 9.5 | 9.2 |
| Language | Rust | Python | Rust |
| Async runtime | tokio | asyncio | tokio |
| Handler system | 22 typed handlers + FnHandler | 22 handler types | Dispatcher + handler chains |
| Filter composition | `&`, `\|`, `^`, `!` operators | Same operators | Predicate combinators |
| ConversationHandler | Full port (timeouts, nesting, persistence) | Full | `dialogue` macro |
| Job queue | Built-in (tokio timers) | APScheduler wrapper | External |
| Persistence | JSON file, SQLite, Redis, PostgreSQL, custom trait | Pickle, Dict, custom | Community crates |
| Webhook support | axum | tornado / starlette | axum / warp |
| Type safety | Compile-time | Runtime (optional hints) | Compile-time |
| Memory idle (measured) | **15 MB** | 57 MB | **15 MB** |
| Memory under load (measured) | **17 MB** | 60 MB | **17 MB** |
| Binary size (stripped) | **6.2 MB** | Requires Python runtime | 6.6 MB |
| Minimum version | Rust 1.75 | Python 3.10 | Rust 1.68 |
| Builder pattern | IntoFuture (directly awaitable) | Keyword args | Method chains |
| Typed constants | `ParseMode::Html` | `ParseMode.HTML` | String-based |
| Maturity | **v1.0.0-beta.5** (new) | Mature (10+ years) | Mature (3+ years) |

## Examples

The `crates/telegram-bot/examples/` directory contains complete, runnable examples:

| Example | Description | Python equivalent |
|---|---|---|
| [`echo_bot`](crates/telegram-bot/examples/echo_bot.rs) | Echoes text messages back to the user | `echobot.py` |
| [`webhook_bot`](crates/telegram-bot/examples/webhook_bot.rs) | **Webhook mode with `run_webhook()` -- simplest production setup** | N/A |
| [`inline_keyboard`](crates/telegram-bot/examples/inline_keyboard.rs) | Inline keyboard with callback queries | `inlinekeyboard.py` |
| [`timer_bot`](crates/telegram-bot/examples/timer_bot.rs) | Job queue: delayed messages, cancellation | `timerbot.py` |
| [`conversation_bot`](crates/telegram-bot/examples/conversation_bot.rs) | Multi-step conversation with state machine | `conversationbot.py` |
| [`raw_api_bot`](crates/telegram-bot/examples/raw_api_bot.rs) | Direct Bot API usage without the ext framework | N/A |
| [`context_types_bot`](crates/telegram-bot/examples/context_types_bot.rs) | Typed data access: bot_data, chat_data, user tracking | `contexttypesbot.py` |
| [`custom_webhook_bot`](crates/telegram-bot/examples/custom_webhook_bot.rs) | Custom axum routes alongside the Telegram webhook | N/A |
| [`bench_bot`](crates/telegram-bot/examples/bench_bot.rs) | Benchmark bot matching PTB/teloxide feature set | N/A |

Run any example:

```sh
TELEGRAM_BOT_TOKEN="your-token" cargo run -p rust-tg-bot --example echo_bot
```

Webhook examples require the `webhooks` feature:

```sh
TELEGRAM_BOT_TOKEN="your-token" WEBHOOK_URL="https://your.domain" \
    cargo run -p rust-tg-bot --example webhook_bot --features webhooks
```

## Project Status

**Current: v1.0.0-beta.5 -- API complete, stabilizing**

What is implemented:

- All Bot API 9.6 types and methods (281 types, 171 method builders)
- `ApplicationBuilder` with typestate pattern
- Typed handler system (`CommandHandler`, `MessageHandler`, `FnHandler`, and more)
- 50+ composable filters with `&`, `|`, `^`, `!` operators
- `ConversationHandler` with full state machine, timeouts, nesting, and persistence
- `JobQueue` with one-shot, repeating, daily, and monthly scheduling (builder pattern)
- JSON file, SQLite, Redis, and PostgreSQL persistence backends
- Typed data access guards (`DataReadGuard`, `DataWriteGuard`)
- Polling and webhook (axum) update delivery with optional TLS
- Callback data caching
- Rate limiter wired into request pipeline
- Defaults system for parse mode, link preview, etc.
- 90+ type constructors for ergonomic API type creation
- Context shortcuts: `reply_html`, `reply_photo`, `reply_document`, `reply_sticker`, `reply_location`
- `answer_callback_query()` and `edit_callback_message_text()` on Context
- Arc<Update> dispatch with bounded update channel (capacity 64)
- `#[non_exhaustive]` on all 344+ public types/enums for forward compatibility
- `#[derive(BotCommands)]` proc macro for declarative command registration
- `#![warn(missing_docs)]` on all crates

### Build & Test

- 385+ tests passing, zero clippy warnings
- 25 roundtrip serialization tests, 9 proptest filter tests, 10 persistence stress tests
- GitHub Actions CI: check, test, clippy, format, examples, docs (stable + MSRV 1.75)
- Release pipeline: cross-compile binaries + crates.io publish
- Measured performance: 6.2 MB binary (stripped), 15 MB idle / 17 MB RSS under load (release) -- matches teloxide, beats it on binary size

### Forward Compatibility

Unlike python-telegram-bot's `api_kwargs` (which captures unknown JSON fields into a dict), RTB uses Rust's `#[non_exhaustive]` attribute on all 344+ public types and enums. This means:

- **New fields** added by Telegram are silently dropped on deserialization until the library is updated.
- **New enum variants** can be added without breaking downstream code.
- **Downstream crates** cannot construct types via struct expressions — they must use constructors or `serde_json::from_value()`.

This is a deliberate trade-off: `#[non_exhaustive]` provides compile-time forward compatibility guarantees that `api_kwargs` cannot, at the cost of not preserving unknown fields on roundtrip. For most bot use cases, unknown fields are irrelevant; if you need to inspect raw JSON, use `serde_json::from_str::<serde_json::Value>()` directly.

### Roadmap

- [ ] Publish to crates.io
- [ ] Comprehensive `cargo doc` documentation with `#[deny(missing_docs)]`
- [ ] Integration tests against real Bot API payloads
- [ ] Benchmarks with `criterion` (throughput, memory, latency)
- [ ] Webhook TLS auto-configuration
- [ ] Passport decryption utilities
- [ ] Payment flow helpers
- [ ] Bot API forward-compatibility layer (auto-update from spec)

## Documentation

- **Guide**: [rust-tg-bot-docs.vercel.app](https://rust-tg-bot-docs.vercel.app/) — mdBook with tutorials, guides, and architecture docs
- **API reference**: [docs.rs/rust-tg-bot](https://docs.rs/rust-tg-bot) — auto-generated from source (available after crates.io publish)

Generate API docs locally:

```sh
cargo doc --open --no-deps
```

## Contributing

Contributions of all sizes are welcome. Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under the [GNU Lesser General Public License v3.0](LICENSE).

You may copy, distribute, and modify the software provided that modifications are described and licensed for free under LGPL-3.0. Derivative works (including modifications or anything statically linked to the library) can only be redistributed under LGPL-3.0, but applications that use the library don't have to be.

## Links

- [Telegram Bot API Documentation](https://core.telegram.org/bots/api)
- [python-telegram-bot](https://github.com/python-telegram-bot/python-telegram-bot) -- the project that started it all
- [Repository](https://github.com/HexiCoreDev/rust-telegram-bot)
- [@BotFather](https://t.me/BotFather) -- create your bot token here
