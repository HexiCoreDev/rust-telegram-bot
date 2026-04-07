# Getting Started

This guide walks you from zero to a running Telegram bot using rust-telegram-bot.

---

## Prerequisites

- **Rust 1.75 or later.** The library uses `async fn` in traits, which was stabilised in 1.75.
  Check your version with `rustc --version`. Install or update via [rustup](https://rustup.rs/).
- **A Telegram bot token.** Open Telegram, message [@BotFather](https://t.me/BotFather), send
  `/newbot`, and follow the prompts. Copy the token (looks like `123456:ABC-DEF...`).
- **tokio runtime.** The library is fully async and runs on tokio.

---

## Installation

Add dependencies to your `Cargo.toml`:

```toml
[dependencies]
telegram-bot = { git = "https://github.com/nicegram/rust-telegram-bot" }
tokio = { version = "1", features = ["full"] }
tracing-subscriber = "0.3"
serde_json = "1"
```

If you need optional features, pick what you need:

```toml
[dependencies]
telegram-bot = { git = "https://github.com/nicegram/rust-telegram-bot", features = [
    "job-queue",          # Scheduled jobs (run_once, run_repeating, etc.)
    "persistence-json",   # JSON file persistence
    "persistence-sqlite", # SQLite persistence
    "webhooks",           # axum-based webhook server
] }
```

Or enable everything at once:

```toml
telegram-bot = { git = "https://github.com/nicegram/rust-telegram-bot", features = ["full"] }
```

---

## Creating Your First Bot

The example below is a complete echo bot. It responds to `/start` with a greeting and
echoes back any plain text message.

Create `src/main.rs`:

```rust
use std::sync::Arc;

use serde_json::Value;

use telegram_bot::ext::application::{Application, HandlerError};
use telegram_bot::ext::builder::ApplicationBuilder;
use telegram_bot::ext::context::CallbackContext;
use telegram_bot::ext::filters::base::Filter;
use telegram_bot::ext::filters::command::COMMAND;
use telegram_bot::ext::filters::text::TEXT;

// Sends a greeting when the user types /start.
async fn start(update: Value, context: CallbackContext) -> Result<(), HandlerError> {
    let chat_id = update["message"]["chat"]["id"]
        .as_i64()
        .ok_or_else(|| HandlerError::Other("missing chat id".into()))?;

    context
        .bot()
        .inner()
        .build_send_message(chat_id.into(), "Hello! Send me any text and I will echo it back.")
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

// Echoes the text back to the sender.
async fn echo(update: Value, context: CallbackContext) -> Result<(), HandlerError> {
    let chat_id = update["message"]["chat"]["id"]
        .as_i64()
        .ok_or_else(|| HandlerError::Other("missing chat id".into()))?;

    let text = update["message"]["text"]
        .as_str()
        .unwrap_or("(empty)");

    context
        .bot()
        .inner()
        .build_send_message(chat_id.into(), text)
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

// Check functions decide whether a handler runs for a given update.
fn is_start(u: &Value) -> bool {
    u["message"]["text"]
        .as_str()
        .map_or(false, |t| t.starts_with("/start"))
}

fn is_plain_text(u: &Value) -> bool {
    TEXT.check_update(u).is_match() && !COMMAND.check_update(u).is_match()
}

#[tokio::main]
async fn main() {
    // Set up structured logging.
    tracing_subscriber::fmt::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN environment variable not set");

    // Build the Application. The typestate builder enforces that token()
    // is called before build().
    let app: Arc<Application> = ApplicationBuilder::new()
        .token(token)
        .build();

    // Register handlers. Group 0 is checked first.
    // Within a group, the first matching handler wins.
    app.add_handler(
        telegram_bot::ext::application::Handler {
            check_update: Arc::new(is_start),
            callback: Arc::new(|u, ctx| Box::pin(start(u, ctx))),
            block: true,
        },
        0,
    )
    .await;

    app.add_handler(
        telegram_bot::ext::application::Handler {
            check_update: Arc::new(is_plain_text),
            callback: Arc::new(|u, ctx| Box::pin(echo(u, ctx))),
            block: true,
        },
        0,
    )
    .await;

    println!("Bot is running. Press Ctrl+C to stop.");

    // Start long-polling with sensible defaults.
    app.run_polling()
        .await
        .unwrap();
}
```

---

## Running the Bot

```sh
TELEGRAM_BOT_TOKEN="123456:ABC-DEF..." cargo run
```

Expected output:

```
2024-01-15T10:00:00.000Z  INFO telegram_bot_ext: Application initialized
2024-01-15T10:00:00.100Z  INFO telegram_bot_ext: Bot running. Polling for updates.
Bot is running. Press Ctrl+C to stop.
```

Open Telegram, find your bot (the username you gave @BotFather), and send `/start`.
You should receive the greeting immediately. Any other text you send will be echoed back.

---

## Understanding the Output

The library logs through `tracing`. The `tracing_subscriber::fmt::init()` call in `main()`
configures a human-readable format to stdout. In production you would swap this for a
structured JSON subscriber or forward logs to a logging backend.

Log levels used by the library:

| Level | What it reports |
|-------|-----------------|
| `INFO` | Application lifecycle events (start, stop, shutdown) |
| `DEBUG` | Individual update dispatch, handler group iteration |
| `WARN` | Non-fatal issues (handler returned error, persistence write failed) |
| `ERROR` | Fatal errors that cause update processing to stop |

Control verbosity via the `RUST_LOG` environment variable:

```sh
RUST_LOG=debug TELEGRAM_BOT_TOKEN="..." cargo run
```

---

## Running the Bundled Examples

The repository ships complete runnable examples:

```sh
# Echo bot (same as above)
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example echo_bot

# Inline keyboard with callback query handling
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example inline_keyboard

# Job queue: delayed messages and cancellation
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example timer_bot

# Multi-step conversation state machine
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example conversation_bot
```

---

## Next Steps

- [Architecture](architecture.md) -- how the two-crate design works and why
- [Handlers](handlers.md) -- all 21 handler types with usage examples
- [Filters](filters.md) -- composable filters and the `&`, `|`, `!` operators
- [Persistence](persistence.md) -- saving user/chat data across restarts
- [Job Queue](job-queue.md) -- scheduled and recurring tasks
- [Migration from Python](migration-from-python.md) -- if you're coming from python-telegram-bot
