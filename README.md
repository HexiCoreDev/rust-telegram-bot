<p align="center">
  <img src="assets/logo.png" alt="rust-telegram-bot" width="200">
</p>

<h1 align="center">rust-telegram-bot</h1>

<p align="center"><strong>We built you a bot you can't refuse -- now in Rust.</strong></p>

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
| **Deployment** | Requires Python runtime + virtualenv | Single static binary, ~10 MB stripped |
| **Resource usage** | ~50-100 MB baseline memory | ~15-27 MB RSS (release mode, measured) |
| **Concurrency** | asyncio (single-threaded) | tokio (multi-threaded work-stealing) |

For bots that handle high volumes of updates, run on constrained hardware (VPS, Raspberry Pi, containers), or need to be deployed without a runtime, Rust is the right tool.

## Architecture

The library is organized as a Cargo workspace with three crates:

```text
rust-telegram-bot/
  crates/
    telegram-bot-raw/     # Pure API types and methods (like Python's `telegram` module)
    telegram-bot-ext/     # Application framework (like Python's `telegram.ext`)
    telegram-bot/         # Facade crate -- re-exports both for convenience
```

**`telegram-bot-raw`** contains every type and method from Bot API 9.6: `Message`, `Update`, `User`, `Chat`, inline types, payments, passport, games, stickers, and all API methods on the `Bot` struct. It depends only on `serde`, `reqwest`, and `tokio`.

**`telegram-bot-ext`** provides the application framework: `ApplicationBuilder`, typed handler system, composable filters, `ConversationHandler`, `JobQueue`, persistence backends (JSON file, SQLite), rate limiting, webhook support, and callback data caching.

**`telegram-bot`** is the facade crate you add to `Cargo.toml`. It re-exports everything from both crates under `telegram_bot::raw` and `telegram_bot::ext`.

## Quick Start

### 1. Create a bot with [@BotFather](https://t.me/BotFather)

Open Telegram, message `@BotFather`, send `/newbot`, and follow the prompts. Copy the token.

### 2. Add the dependency

```toml
[dependencies]
telegram-bot = { git = "https://github.com/HexiCoreDev/rust-telegram-bot" }
tracing-subscriber = "0.3"
```

### 3. Write your bot

```rust,no_run
use telegram_bot::ext::prelude::*;

async fn start(update: Update, context: Context) -> HandlerResult {
    let name = update
        .effective_user()
        .map(|u| u.first_name.as_str())
        .unwrap_or("there");
    context
        .reply_text(&update, &format!("Hi {name}! Send me any text and I will echo it back."))
        .await?;
    Ok(())
}

async fn echo(update: Update, context: Context) -> HandlerResult {
    let text = update
        .effective_message()
        .and_then(|m| m.text.as_deref())
        .unwrap_or("");
    if !text.is_empty() {
        context.reply_text(&update, text).await?;
    }
    Ok(())
}

fn main() {
    telegram_bot::run(async {
        tracing_subscriber::fmt::init();

        let token = std::env::var("TELEGRAM_BOT_TOKEN")
            .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

        let app = ApplicationBuilder::new().token(token).build();

        app.add_typed_handler(CommandHandler::new("start", start), 0).await;
        app.add_typed_handler(
            MessageHandler::new(TEXT() & !COMMAND(), echo),
            0,
        ).await;

        println!("Echo bot is running. Press Ctrl+C to stop.");

        if let Err(e) = app.run_polling().await {
            eprintln!("Error running bot: {e}");
        }
    });
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
use telegram_bot::ext::prelude::*;

// Text messages that are NOT commands
let text_only = TEXT() & !COMMAND();

// Use it with a handler
app.add_typed_handler(
    MessageHandler::new(text_only, my_callback),
    0,
).await;
```

Over 50 built-in filters are available: `TEXT`, `COMMAND`, `PHOTO`, `VIDEO`, `AUDIO`, `VOICE`, `DOCUMENT`, `LOCATION`, `CONTACT`, `ANIMATION`, `STICKER`, `POLL`, `VENUE`, `GAME`, `INVOICE`, `FORWARDED`, `REPLY`, `PREMIUM_USER`, `FORUM`, chat type filters, entity filters, regex filters, and more.

### Typed Handler Registration

Handlers use strongly-typed constructors and are registered via `add_typed_handler`:

```rust,ignore
use telegram_bot::ext::prelude::*;

// Command handler -- matches /start
app.add_typed_handler(CommandHandler::new("start", start), 0).await;

// Message handler -- matches text that is not a command
app.add_typed_handler(MessageHandler::new(TEXT() & !COMMAND(), echo), 0).await;

// Callback query handler -- matches inline keyboard button presses
app.add_typed_handler(FnHandler::on_callback_query(button), 0).await;

// Generic function handler with custom predicate
app.add_typed_handler(
    FnHandler::new(|u| u.callback_query.is_some(), my_handler),
    0,
).await;

// Catch-all handler (e.g., for logging)
app.add_typed_handler(FnHandler::on_any(track_users), -1).await;
```

### Builder-based Bot API

Every Bot API method returns a builder with optional parameters as chainable setters. Builders implement `IntoFuture`, so you can use `.await` directly, or call `.send().await` explicitly:

```rust,ignore
// Simple -- .await directly
context.bot().send_message(chat_id, "Hello!").await?;

// With optional parameters
context
    .bot()
    .send_message(chat_id, "<b>Bold</b> text")
    .parse_mode(ParseMode::Html)
    .send()
    .await?;

// Inline keyboard
context
    .bot()
    .send_message(chat_id, "Choose:")
    .reply_markup(keyboard_json)
    .send()
    .await?;

// Edit a message
context
    .bot()
    .edit_message_text("Updated text")
    .chat_id(chat_id)
    .message_id(msg_id)
    .send()
    .await?;
```

### Typed Constants

No more magic strings. All enums are strongly typed:

```rust,ignore
use telegram_bot::ext::prelude::*;

// Parse modes
context.bot().send_message(chat_id, text)
    .parse_mode(ParseMode::Html)
    .send()
    .await?;

// Entity types
if entity.entity_type == MessageEntityType::BotCommand { /* ... */ }

// Chat types
if chat.chat_type != ChatType::Private { /* ... */ }
```

### Handler Groups

Handlers are organized into numbered groups. Within a group, the first matching handler wins. Across groups, processing continues unless a handler returns `HandlerResult::Stop`:

```rust,ignore
// Group -1: always runs first (e.g., user tracking)
app.add_typed_handler(FnHandler::on_any(track_users), -1).await;

// Group 0: command handlers
app.add_typed_handler(CommandHandler::new("start", start), 0).await;
app.add_typed_handler(CommandHandler::new("help", help), 0).await;

// Group 1: message handlers
app.add_typed_handler(
    MessageHandler::new(TEXT() & !COMMAND(), echo),
    1,
).await;
```

### Typed Data Access

The `Context` provides typed read and write guards for bot-wide, per-user, and per-chat data:

```rust,ignore
async fn handle(update: Update, context: Context) -> HandlerResult {
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

Schedule one-shot, repeating, daily, and monthly jobs using tokio timers and a builder pattern:

```rust,ignore
use telegram_bot::ext::job_queue::{JobQueue, JobCallbackFn, JobContext};

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
use telegram_bot::ext::persistence::json_file::JsonFilePersistence;

let persistence = JsonFilePersistence::new("bot_data", true, false);
let app = ApplicationBuilder::new()
    .token(token)
    .persistence(Box::new(persistence))
    .build();
```

**SQLite** (production-ready, WAL mode, atomic writes):

```rust,ignore
use telegram_bot::ext::persistence::sqlite::SqlitePersistence;

let persistence = SqlitePersistence::open("bot.db").unwrap();
let app = ApplicationBuilder::new()
    .token(token)
    .persistence(Box::new(persistence))
    .build();
```

**Custom backend** -- implement the `BasePersistence` trait:

```rust,ignore
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
telegram-bot = { git = "https://github.com/HexiCoreDev/rust-telegram-bot" }

# Everything
telegram-bot = { git = "https://github.com/HexiCoreDev/rust-telegram-bot", features = ["full"] }

# Pick what you need
telegram-bot = { git = "https://github.com/HexiCoreDev/rust-telegram-bot", features = [
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
| Handler system | Typed handlers + FnHandler | 20+ handler types | Dispatcher + handler chains |
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
| [`context_types_bot`](crates/telegram-bot/examples/context_types_bot.rs) | Typed data access: bot_data, chat_data, user tracking | `contexttypesbot.py` |

Run any example:

```sh
TELEGRAM_BOT_TOKEN="your-token" cargo run -p telegram-bot --example echo_bot
```

## Project Status

**Current: v0.1.0 -- API complete, stabilizing**

What is implemented:

- All Bot API 9.6 types and methods
- `ApplicationBuilder` with typestate pattern
- Typed handler system (`CommandHandler`, `MessageHandler`, `FnHandler`, and more)
- 50+ composable filters with `&`, `|`, `^`, `!` operators
- `ConversationHandler` with full state machine, timeouts, nesting, and persistence
- `JobQueue` with one-shot, repeating, daily, and monthly scheduling (builder pattern)
- JSON file and SQLite persistence backends
- Typed data access guards (`DataReadGuard`, `DataWriteGuard`)
- Polling and webhook (axum) update delivery
- Callback data caching
- Rate limiter
- Defaults system for parse mode, link preview, etc.
- `telegram_bot::run()` for safe async entry point with proper stack sizing

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
- [Handlers](docs/handlers.md) -- Handler types with usage examples
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
- [Repository](https://github.com/HexiCoreDev/rust-telegram-bot)
- [@BotFather](https://t.me/BotFather) -- create your bot token here
