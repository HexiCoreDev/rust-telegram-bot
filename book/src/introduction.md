<p align="center">
  <img src="favicon.png" alt="rust-tg-bot" width="150">
</p>

# rust-tg-bot

A complete, production-ready Telegram Bot API framework for Rust. Inspired by the architecture of [python-telegram-bot](https://python-telegram-bot.org/), this library brings the same developer-friendly patterns to the Rust ecosystem while delivering the performance, safety, and concurrency guarantees that Rust provides.

## Why rust-tg-bot?

- **Familiar architecture.** If you have used python-telegram-bot, you will feel right at home. The same concepts -- `Application`, `Handler`, `Filter`, `Context` -- are present, adapted to Rust idioms.
- **Type safety.** Every Telegram API type is a strongly typed Rust struct. No more guessing whether a field is a string or an integer.
- **Async from the ground up.** Built on [tokio](https://tokio.rs/) and [reqwest](https://docs.rs/reqwest), every operation is non-blocking.
- **Zero-cost abstractions.** Handler dispatch, filter composition, and builder patterns compile down to minimal overhead.
- **Feature-gated modules.** Only pull in what you need: `webhooks`, `job-queue`, `persistence`, `rate-limiter`.

## Architecture Overview

The framework is split into three crates:

| Crate | Purpose |
|---|---|
| `rust-tg-bot-raw` | Low-level Bot API types, HTTP methods, request builders |
| `rust-tg-bot-ext` | High-level `Application`, handlers, filters, context, persistence |
| `rust-tg-bot` | Facade crate that re-exports both for convenience |

You will almost always depend only on `rust-tg-bot` in your `Cargo.toml`.

## What You Will Learn

This documentation takes you from zero to a production deployment:

1. **Getting Started** -- Install the library, write your first bot, and run it.
2. **Core Concepts** -- Understand the building blocks: `Update`, `Bot`, handlers, filters, context, and the `Application` lifecycle.
3. **Guides** -- Build real features: command handling, inline keyboards, multi-step conversations, scheduled jobs, data persistence, webhooks, inline mode, and payments.
4. **Advanced** -- Write custom filters, handle errors gracefully, nest conversations, test your bot, and deploy to production.
5. **Migration** -- A side-by-side reference for developers moving from python-telegram-bot to Rust.

## Quick Taste

Here is the smallest possible echo bot:

```rust
use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, Arc, CommandHandler, Context, HandlerResult,
    MessageHandler, Update, COMMAND, TEXT,
};

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
    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN must be set");

    let app = ApplicationBuilder::new().token(token).build();

    app.add_handler(
        MessageHandler::new(TEXT() & !COMMAND(), echo), 0,
    ).await;

    app.run_polling().await.unwrap();
}
```

Ready to begin? Head to [Installation](getting-started/installation.md).
