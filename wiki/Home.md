# rust-telegram-bot Wiki

**A Rust framework for building Telegram bots with the same developer experience as python-telegram-bot.**

*Author: Jude Etuka*
*Repository: [https://github.com/HexiCoreDev/rust-telegram-bot](https://github.com/HexiCoreDev/rust-telegram-bot)*

---

## Quick Links

| Resource | Description |
|----------|-------------|
| [Zero to Hero Guide](../docs/ZERO_TO_HERO.md) | Comprehensive tutorial from first bot to production deployment |
| [Telegram Bot API Overview](Telegram-Bot-API-Overview) | Quick primer on how the Bot API works |
| [Examples Overview](Examples-Overview) | Guide to all 20 included examples |
| [FAQ](FAQ) | Frequently asked questions |
| [Troubleshooting](Troubleshooting) | Common issues and solutions |
| [Comparison with Python](Comparison-with-Python) | Detailed migration guide for PTB users |
| [Performance Tuning](Performance-Tuning) | Memory, concurrency, and release mode tips |

---

## Getting Started in 60 Seconds

1. Talk to [@BotFather](https://t.me/BotFather) on Telegram and create a bot with `/newbot`. Copy the token.

2. Create a new Rust project:

```sh
cargo new my_bot && cd my_bot
```

3. Add the dependency to `Cargo.toml`:

```toml
[dependencies]
telegram-bot = { git = "https://github.com/HexiCoreDev/rust-telegram-bot", package = "telegram-bot" }
tracing-subscriber = "0.3"
```

4. Write your bot in `src/main.rs`:

```rust
use telegram_bot::ext::prelude::*;

async fn hello(update: Update, context: Context) -> HandlerResult {
    context.reply_text(&update, "Hello from Rust!").await?;
    Ok(())
}

fn main() {
    telegram_bot::run(async {
        tracing_subscriber::fmt::init();
        let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
        let app = ApplicationBuilder::new().token(token).build();
        app.add_typed_handler(CommandHandler::new("start", hello), 0).await;
        app.run_polling().await.unwrap();
    });
}
```

5. Run it:

```sh
TELEGRAM_BOT_TOKEN="your-token" cargo run
```

---

## Architecture

The framework is split into three crates:

| Crate | Purpose |
|-------|---------|
| `telegram-bot-raw` | Low-level Bot API types, HTTP client, request builders |
| `telegram-bot-ext` | Application framework: handlers, filters, context, persistence, job queue |
| `telegram-bot` | Convenience crate that re-exports both and provides `telegram_bot::run()` |

You only need to depend on `telegram-bot`. The other two are implementation details.

### Handler Dispatch Model

```
Update arrives
    |
    v
Group 0: [CommandHandler("/start"), CommandHandler("/help"), MessageHandler(TEXT & !CMD)]
    |  first match wins within group
    v
Group 1: [FnHandler(state_check, conversation_step)]
    |  first match wins within group
    v
Group 2: [FnHandler::on_any(fallback)]
    |
    v
Error handlers (if any handler returned Err)
```

Groups are processed in ascending numeric order. Within each group, the first handler whose `check_update` returns `Some` wins. Only one handler per group fires. This design lets you layer commands, conversation logic, and fallbacks cleanly.

---

## Feature Flags

| Feature | Description |
|---------|-------------|
| `persistence-json` | JSON file persistence backend |
| `persistence-sqlite` | SQLite persistence backend (bundles SQLite) |
| `webhooks` | Webhook support via axum |

Enable features in your `Cargo.toml`:

```toml
telegram-bot = { git = "...", features = ["persistence-json", "webhooks"] }
```

---

## Performance at a Glance

| Metric | Value |
|--------|-------|
| Binary size (stripped, release, LTO) | ~10 MB |
| RSS at idle (release) | 15-20 MB |
| RSS under load (release) | 20-27 MB |
| Cold start time | < 500ms |
| Updates/second (single core) | > 1,000 |

See [Performance Tuning](Performance-Tuning) for optimization strategies.

---

## Contributing

Contributions are welcome. Please open an issue first to discuss any significant changes. The codebase follows standard Rust conventions (`cargo fmt`, `cargo clippy`, `cargo test`).
