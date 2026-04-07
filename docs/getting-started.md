# Getting Started

This guide walks you from zero to a running Telegram bot using rust-telegram-bot.

---

## Prerequisites

- **Rust 1.75 or later.** The library uses `async fn` in traits, which was stabilised in 1.75.
  Check your version with `rustc --version`. Install or update via [rustup](https://rustup.rs/).
- **A Telegram bot token.** Open Telegram, message [@BotFather](https://t.me/BotFather), send
  `/newbot`, and follow the prompts. Copy the token (looks like `123456:ABC-DEF...`).

---

## Installation

Add dependencies to your `Cargo.toml`:

```toml
[dependencies]
telegram-bot = { git = "https://github.com/HexiCoreDev/rust-telegram-bot" }
tracing-subscriber = "0.3"
```

If you need optional features, pick what you need:

```toml
[dependencies]
telegram-bot = { git = "https://github.com/HexiCoreDev/rust-telegram-bot", features = [
    "job-queue",          # Scheduled jobs (once, repeating, etc.)
    "persistence-json",   # JSON file persistence
    "persistence-sqlite", # SQLite persistence
    "webhooks",           # axum-based webhook server
] }
```

Or enable everything at once:

```toml
telegram-bot = { git = "https://github.com/HexiCoreDev/rust-telegram-bot", features = ["full"] }
```

---

## Creating Your First Bot

The example below is a complete echo bot. It responds to `/start` with a greeting and
echoes back any plain text message.

Create `src/main.rs`:

```rust
use telegram_bot::ext::prelude::*;

/// Respond to `/start` with a greeting.
async fn start(update: Update, context: Context) -> HandlerResult {
    let name = update
        .effective_user()
        .map(|u| u.first_name.as_str())
        .unwrap_or("there");
    context
        .reply_text(
            &update,
            &format!(
                "Hi {name}! I am an echo bot. Send me any message and I will repeat it back.\n\n\
                 Use /help to see available commands."
            ),
        )
        .await?;
    Ok(())
}

/// Respond to `/help` with usage instructions.
async fn help(update: Update, context: Context) -> HandlerResult {
    context
        .reply_text(
            &update,
            "Available commands:\n\
             /start - Start the bot\n\
             /help - Show this help message\n\n\
             Send any text message and I will echo it back!",
        )
        .await?;
    Ok(())
}

/// Echo back whatever text the user sends.
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
        app.add_typed_handler(CommandHandler::new("help", help), 0).await;
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

---

## Understanding the Code

### Entry point

`telegram_bot::run()` builds a multi-threaded tokio runtime with an 8 MB thread stack
(needed for deeply nested async state machines) and blocks on the provided future. You
do not need to add `#[tokio::main]` or configure the runtime yourself.

### Prelude import

`use telegram_bot::ext::prelude::*` brings in everything you need for typical bot code:

- `Update`, `Message` -- strongly-typed API types
- `Context` (alias for `CallbackContext`) -- passed to every handler
- `HandlerResult` -- `Result<(), HandlerError>`
- `ApplicationBuilder`, `Application` -- framework entry points
- `CommandHandler`, `MessageHandler`, `FnHandler` -- handler types
- `TEXT()`, `COMMAND()` -- filter constructors
- `ParseMode`, `ChatType`, `MessageEntityType` -- typed constants
- `F`, `Filter`, `FilterResult` -- filter composition

### Handler registration

`app.add_typed_handler(handler, group)` registers a handler in a numbered group.
Within a group, the first matching handler wins. Across groups, processing continues
unless a handler returns `HandlerResult::Stop`.

### Handler signatures

All handler callbacks have the signature:

```rust
async fn my_handler(update: Update, context: Context) -> HandlerResult
```

The `Update` is a strongly-typed struct with methods like `effective_user()`,
`effective_chat()`, and `effective_message()`. The `Context` provides access to
the bot, data stores, and the job queue.

### Convenience methods

`context.reply_text(&update, text)` is a shorthand that extracts the chat ID from the
update and calls `context.bot().send_message(chat_id, text).await`. For more control,
use the builder directly:

```rust
context
    .bot()
    .send_message(chat_id, "<b>Hello!</b>")
    .parse_mode(ParseMode::Html)
    .send()
    .await?;
```

---

## Running the Bot

```sh
TELEGRAM_BOT_TOKEN="123456:ABC-DEF..." cargo run
```

Expected output:

```
Echo bot is running. Press Ctrl+C to stop.
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

# Typed data access: bot_data, chat_data, user tracking
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example context_types_bot
```

---

## Next Steps

- [Architecture](architecture.md) -- how the two-crate design works and why
- [Handlers](handlers.md) -- handler types with usage examples
- [Filters](filters.md) -- composable filters and the `&`, `|`, `!` operators
- [Persistence](persistence.md) -- saving user/chat data across restarts
- [Job Queue](job-queue.md) -- scheduled and recurring tasks
- [Migration from Python](migration-from-python.md) -- if you're coming from python-telegram-bot
