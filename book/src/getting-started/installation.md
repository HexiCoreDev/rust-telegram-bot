# Installation

## Prerequisites

Before you begin, make sure you have:

- **Rust 1.75 or later.** Check with `rustc --version`. Install or update via [rustup](https://rustup.rs/).
- **A Telegram Bot Token.** Create a bot through [@BotFather](https://t.me/BotFather) on Telegram. You will receive a token that looks like `123456789:ABCdefGHIjklMNOpqrsTUVwxyz`.

## Adding the Dependency

Add `rust-tg-bot` to your project:

```sh
cargo add rust-tg-bot
```

Or add it manually to your `Cargo.toml`:

```toml
[dependencies]
rust-tg-bot = "1.0.0-rc.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

You need `tokio` with the `rt-multi-thread` and `macros` features because the framework is fully async.

## Feature Flags

The crate uses feature flags to keep the default build lean. Enable only what you need:

| Feature | Description | Default |
|---|---|---|
| `webhooks` | Webhook-based update delivery (requires `axum`) | No |
| `job-queue` | Scheduled and recurring tasks via `JobQueue` | No |
| `persistence` | Base persistence trait | No |
| `persistence-json` | JSON file persistence backend | No |
| `persistence-sqlite` | SQLite persistence backend | No |
| `rate-limiter` | Built-in rate limiting for API calls | No |
| `full` | Enables all features | No |

Enable features in `Cargo.toml`:

```toml
[dependencies]
rust-tg-bot = { version = "1.0.0-rc.1", features = ["job-queue", "persistence-json"] }
```

Or enable everything:

```toml
[dependencies]
rust-tg-bot = { version = "1.0.0-rc.1", features = ["full"] }
```

## Verifying the Installation

Create a minimal project to confirm everything works:

```sh
cargo new my-rust-tg-bot
cd my-rust-tg-bot
cargo add rust-tg-bot
cargo add tokio --features rt-multi-thread,macros
```

Replace `src/main.rs` with:

```rust
use rust_tg_bot::ext::prelude::{ApplicationBuilder, Arc, CommandHandler, Context, HandlerResult, Update};

async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    context.reply_text(&update, "Hello from Rust!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN must be set");

    let app = ApplicationBuilder::new().token(token).build();
    app.add_handler(CommandHandler::new("start", start), 0).await;

    println!("Bot is running. Press Ctrl+C to stop.");
    app.run_polling().await.unwrap();
}
```

Run it:

```sh
TELEGRAM_BOT_TOKEN="your-token-here" cargo run
```

Send `/start` to your bot on Telegram. If you see "Hello from Rust!", you are ready. Continue to [Your First Bot](first-bot.md) for a walkthrough.

## Optional: Logging

The framework uses the [`tracing`](https://docs.rs/tracing) crate for structured logging. To see log output, add `tracing-subscriber`:

```sh
cargo add tracing-subscriber
```

Then initialize it at the top of `main()`:

```rust
tracing_subscriber::fmt::init();
```

This gives you detailed output about handler dispatch, API calls, and errors -- invaluable during development.
