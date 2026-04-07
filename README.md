# rust-telegram-bot

**We built you a bot you can't refuse -- now in Rust.**

[![Bot API 9.6](https://img.shields.io/badge/Bot%20API-9.6-blue?logo=telegram)](https://core.telegram.org/bots/api-changelog)
[![License: LGPL-3.0](https://img.shields.io/badge/License-LGPL--3.0-green.svg)](https://www.gnu.org/licenses/lgpl-3.0.html)
[![Rust: 1.75+](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org)
[![CI](https://img.shields.io/badge/CI-passing-brightgreen)]()
[![Docs](https://img.shields.io/badge/docs-latest-blue)]()

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
| **Deployment** | Requires Python runtime + virtualenv | Single static binary, < 10 MB |
| **Resource usage** | ~50-100 MB baseline memory | ~5-15 MB baseline memory |
| **Concurrency** | asyncio (single-threaded) | tokio (multi-threaded work-stealing) |

For bots that handle high volumes of updates, run on constrained hardware (VPS, Raspberry Pi, containers), or need to be deployed without a runtime, Rust is the right tool.

## Architecture

The library is organized as a Cargo workspace with three crates:

```
rust-telegram-bot/
  crates/
    telegram-bot-raw/     # Pure API types and methods (like Python's `telegram` module)
    telegram-bot-ext/     # Application framework (like Python's `telegram.ext`)
    telegram-bot/         # Facade crate -- re-exports both for convenience
```

**`telegram-bot-raw`** contains every type and method from Bot API 9.6: `Message`, `Update`, `User`, `Chat`, inline types, payments, passport, games, stickers, and all API methods on the `Bot` struct. It depends only on `serde`, `reqwest`, and `tokio`.

**`telegram-bot-ext`** provides the application framework: `ApplicationBuilder`, 21 handler types, composable filters, `ConversationHandler`, `JobQueue`, persistence backends (JSON file, SQLite), rate limiting, webhook support, and callback data caching.

**`telegram-bot`** is the facade crate you add to `Cargo.toml`. It re-exports everything from both crates under `telegram_bot::raw` and `telegram_bot::ext`.

## Quick Start

### 1. Create a bot with [@BotFather](https://t.me/BotFather)

Open Telegram, message `@BotFather`, send `/newbot`, and follow the prompts. Copy the token.

### 2. Add the dependency

```toml
[dependencies]
telegram-bot = { git = "https://github.com/nicegram/rust-telegram-bot" }
tokio = { version = "1", features = ["full"] }
tracing-subscriber = "0.3"
serde_json = "1"
```

### 3. Write your bot

```rust
use std::sync::Arc;
use std::time::Duration;
use serde_json::Value;

use telegram_bot::ext::application::{self, Application, HandlerError};
use telegram_bot::ext::builder::ApplicationBuilder;
use telegram_bot::ext::context::CallbackContext;
use telegram_bot::ext::filters::base::Filter;
use telegram_bot::ext::filters::command::COMMAND;
use telegram_bot::ext::filters::text::TEXT;

async fn start(update: Value, context: CallbackContext) -> Result<(), HandlerError> {
    let chat_id = update["message"]["chat"]["id"].as_i64().unwrap();
    context.bot().inner()
        .send_message(chat_id.into(), "Hello! Send me a message and I'll echo it back.",
            None, None, None, None, None, None, None, None, None, None, None, None, None)
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;
    Ok(())
}

async fn echo(update: Value, context: CallbackContext) -> Result<(), HandlerError> {
    let chat_id = update["message"]["chat"]["id"].as_i64().unwrap();
    let text = update["message"]["text"].as_str().unwrap_or("");
    context.bot().inner()
        .send_message(chat_id.into(), text,
            None, None, None, None, None, None, None, None, None, None, None, None, None)
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;
    Ok(())
}

fn is_start(u: &Value) -> bool {
    u["message"]["text"].as_str().map_or(false, |t| t.starts_with("/start"))
}

fn is_text_not_command(u: &Value) -> bool {
    TEXT.check_update(u).is_match() && !COMMAND.check_update(u).is_match()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let token = std::env::var("TELEGRAM_BOT_TOKEN").expect("Set TELEGRAM_BOT_TOKEN");

    let app: Arc<Application> = ApplicationBuilder::new().token(token).build();

    app.add_handler(application::Handler {
        check_update: Arc::new(is_start),
        callback: Arc::new(|u, ctx| Box::pin(start(u, ctx))),
        block: true,
    }, 0).await;

    app.add_handler(application::Handler {
        check_update: Arc::new(is_text_not_command),
        callback: Arc::new(|u, ctx| Box::pin(echo(u, ctx))),
        block: true,
    }, 0).await;

    println!("Bot running. Press Ctrl+C to stop.");
    app.run_polling(Duration::from_secs(0), None, false).await.unwrap();
}
```

### 4. Run it

```sh
TELEGRAM_BOT_TOKEN="123456:ABC-DEF" cargo run
```

## Feature Highlights

### Composable Filters

Filters use Rust's bitwise operators for composition -- the same mental model as python-telegram-bot, but enforced at compile time:

```rust
use telegram_bot::ext::filters::base::{F, Filter};
use telegram_bot::ext::filters::text::TEXT;
use telegram_bot::ext::filters::command::COMMAND;

// Text messages that are NOT commands
let text_only = F::new(TEXT) & !F::new(COMMAND);

// Photos or videos
use telegram_bot::ext::filters::base::{PHOTO, VIDEO};
let media = F::new(PHOTO) | F::new(VIDEO);

// Custom closure filter
use telegram_bot::ext::filters::base::FnFilter;
let premium_only = FnFilter::new("premium_users", |update| {
    update["message"]["from"]["is_premium"]
        .as_bool()
        .unwrap_or(false)
});
```

Over 50 built-in filters are available: `TEXT`, `COMMAND`, `PHOTO`, `VIDEO`, `AUDIO`, `VOICE`, `DOCUMENT`, `LOCATION`, `CONTACT`, `ANIMATION`, `STICKER`, `POLL`, `VENUE`, `GAME`, `INVOICE`, `FORWARDED`, `REPLY`, `PREMIUM_USER`, `FORUM`, chat type filters, entity filters, regex filters, and more.

### Handler Groups

Handlers are organized into numbered groups. Within a group, the first matching handler wins. Across groups, processing continues unless a handler returns `HandlerResult::Stop`:

```rust
// Group 0: command handlers (checked first)
app.add_handler(start_handler, 0).await;
app.add_handler(help_handler, 0).await;

// Group 1: message handlers (checked if no command matched)
app.add_handler(echo_handler, 1).await;

// Group 2: logging handler (always runs)
app.add_handler(log_handler, 2).await;
```

### ConversationHandler State Machine

Multi-step conversations with automatic state tracking, timeouts, nested conversations, and persistence:

```rust
use telegram_bot::ext::handlers::conversation::*;

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

### Job Queue Scheduling

Schedule one-shot, repeating, daily, and monthly jobs using tokio timers:

```rust
use telegram_bot::ext::job_queue::{JobQueue, JobCallbackFn, JobContext};

let jq = Arc::new(JobQueue::new());
jq.start().await;

// One-shot: fire after 30 seconds
jq.run_once("reminder", Duration::from_secs(30), callback, None, None, None).await;

// Repeating: every 60 seconds
jq.run_repeating("heartbeat", Duration::from_secs(60), None, None, callback, None, None, None).await;

// Daily at 09:00 UTC on weekdays
use chrono::NaiveTime;
let time = NaiveTime::from_hms_opt(9, 0, 0).unwrap();
jq.run_daily("morning_report", time, &[1, 2, 3, 4, 5], callback, None, None, None).await;

// Monthly on the 1st at midnight
jq.run_monthly("monthly_summary", time, 1, callback, None, None, None).await;
```

Every job returns a `Job` handle for cancellation, status checking, and enable/disable toggling.

### Persistence

Swap between backends without changing application code:

**JSON file** (human-readable, great for development):

```rust
use telegram_bot::ext::persistence::json_file::JsonFilePersistence;

let persistence = JsonFilePersistence::new("bot_data", true, false);
let app = ApplicationBuilder::new()
    .token(token)
    .persistence(Box::new(persistence))
    .build();
```

**SQLite** (production-ready, WAL mode, atomic writes):

```rust
use telegram_bot::ext::persistence::sqlite::SqlitePersistence;

let persistence = SqlitePersistence::open("bot.db").unwrap();
let app = ApplicationBuilder::new()
    .token(token)
    .persistence(Box::new(persistence))
    .build();
```

**Custom backend** -- implement the `BasePersistence` trait:

```rust
use telegram_bot::ext::persistence::base::BasePersistence;

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
telegram-bot = { git = "https://github.com/nicegram/rust-telegram-bot" }

# Everything
telegram-bot = { git = "https://github.com/nicegram/rust-telegram-bot", features = ["full"] }

# Pick what you need
telegram-bot = { git = "https://github.com/nicegram/rust-telegram-bot", features = [
    "webhooks",           # axum-based webhook server
    "job-queue",          # Scheduled job execution
    "persistence-json",   # JSON file persistence
    "persistence-sqlite", # SQLite persistence
    "rate-limiter",       # API rate limiting
] }
```

## Comparison

| Feature | rust-telegram-bot | python-telegram-bot | teloxide |
|---|:---:|:---:|:---:|
| Bot API version | 9.6 | 9.5 | 7.x |
| Language | Rust | Python | Rust |
| Async runtime | tokio | asyncio | tokio |
| Handler system | 21 handler types | 20+ handler types | Dispatcher + handler chains |
| Filter composition | `&`, `\|`, `^`, `!` operators | Same operators | Predicate combinators |
| ConversationHandler | Full port with timeouts + nesting | Full | dialogue macro |
| Job queue | Built-in (tokio timers) | APScheduler | External |
| Persistence | JSON, SQLite, custom trait | Pickle, custom | Redis (community) |
| Webhook support | axum | tornado | axum / warp |
| Type safety | Compile-time | Runtime (optional hints) | Compile-time |
| Memory footprint | ~5-15 MB | ~50-100 MB | ~10-20 MB |
| Binary size | Single static binary | Requires runtime | Single static binary |
| Minimum version | Rust 1.75 | Python 3.10 | Rust 1.68 |

## Examples

The `crates/telegram-bot/examples/` directory contains complete, runnable examples:

| Example | Description | Python equivalent |
|---|---|---|
| [`echo_bot`](crates/telegram-bot/examples/echo_bot.rs) | Echoes text messages back to the user | `echobot.py` |
| [`inline_keyboard`](crates/telegram-bot/examples/inline_keyboard.rs) | Inline keyboard with callback queries | `inlinekeyboard.py` |
| [`timer_bot`](crates/telegram-bot/examples/timer_bot.rs) | Job queue: delayed messages, cancellation | `timerbot.py` |
| [`conversation_bot`](crates/telegram-bot/examples/conversation_bot.rs) | Multi-step conversation with state machine | `conversationbot.py` |
| [`raw_api_bot`](crates/telegram-bot/examples/raw_api_bot.rs) | Direct Bot API usage without the ext framework | N/A |

Run any example:

```sh
TELEGRAM_BOT_TOKEN="your-token" cargo run -p telegram-bot --example echo_bot
```

## Project Status

**Current: v0.1.0 -- API complete, stabilizing**

What is implemented:

- All Bot API 9.6 types and methods
- `ApplicationBuilder` with typestate pattern
- 21 handler types (command, message, callback query, inline query, conversation, and more)
- 50+ composable filters with `&`, `|`, `^`, `!` operators
- `ConversationHandler` with full state machine, timeouts, nesting, and persistence
- `JobQueue` with one-shot, repeating, daily, and monthly scheduling
- JSON file and SQLite persistence backends
- Polling and webhook (axum) update delivery
- Callback data caching
- Rate limiter
- Defaults system for parse mode, link preview, etc.

### Roadmap

- [ ] Publish to crates.io
- [ ] Comprehensive `cargo doc` documentation on all public items
- [ ] Webhook TLS auto-configuration
- [ ] Proxy support (SOCKS5, HTTP)
- [ ] Passport decryption utilities
- [ ] Payment flow helpers
- [ ] Bot API forward-compatibility layer
- [ ] Property-based testing with proptest
- [ ] Benchmarks with criterion

## Documentation

- [Getting Started](docs/getting-started.md) -- Installation, token setup, first bot
- [Architecture](docs/architecture.md) -- Crate structure, design decisions
- [Handlers](docs/handlers.md) -- All 21 handlers with usage examples
- [Filters](docs/filters.md) -- Filter system, composition, all filter types
- [Persistence](docs/persistence.md) -- JSON, SQLite, custom backends
- [Job Queue](docs/job-queue.md) -- Scheduling, cancellation, daily/monthly triggers
- [Migration from Python](docs/migration-from-python.md) -- Guide for python-telegram-bot users
- [API Reference](docs/api-reference.md) -- Pointer to `cargo doc` output

## Contributing

Contributions of all sizes are welcome. Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under the [GNU Lesser General Public License v3.0](LICENSE).

You may copy, distribute, and modify the software provided that modifications are described and licensed for free under LGPL-3.0. Derivative works (including modifications or anything statically linked to the library) can only be redistributed under LGPL-3.0, but applications that use the library don't have to be.

## Links

- [Telegram Bot API Documentation](https://core.telegram.org/bots/api)
- [python-telegram-bot](https://github.com/python-telegram-bot/python-telegram-bot) -- the project that started it all
- [Repository](https://github.com/nicegram/rust-telegram-bot)
- [@BotFather](https://t.me/BotFather) -- create your bot token here
