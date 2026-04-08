# Running Your Bot

There are two ways your bot can receive updates from Telegram: **long polling** and **webhooks**. This page covers both, along with environment configuration and graceful shutdown.

## Environment Variables

The framework reads the bot token from your code, not from environment variables directly. However, the convention across all examples is:

```sh
export TELEGRAM_BOT_TOKEN="123456789:ABCdefGHIjklMNOpqrsTUVwxyz"
```

Then in your code:

```rust
let token = std::env::var("TELEGRAM_BOT_TOKEN")
    .expect("TELEGRAM_BOT_TOKEN must be set");
```

Never hard-code your token. Use environment variables, a `.env` file, or a secrets manager.

## Long Polling

Long polling is the simplest approach and the right choice for development and most deployments.

```rust
let app = ApplicationBuilder::new().token(token).build();

// ... register handlers ...

app.run_polling().await.unwrap();
```

`run_polling()` handles the full lifecycle internally:

1. Calls `initialize()` to set up the application.
2. Calls `start()` to begin processing.
3. Enters a loop calling `getUpdates` on the Telegram API.
4. Dispatches each update to your registered handlers.
5. On `Ctrl+C`, calls `stop()` and `shutdown()`.

### How Long Polling Works

Your bot sends a request to Telegram saying "give me any new updates." Telegram holds the connection open (up to 30 seconds by default) until either a new update arrives or the timeout expires. This is efficient -- there is no busy-waiting.

## Webhooks

For production deployments behind a reverse proxy, webhooks are more efficient. Telegram pushes updates to your server instead of your server pulling them.

Enable the `webhooks` feature:

```toml
[dependencies]
telegram-bot = { version = "1.0.0-beta.2", features = ["webhooks"] }
```

### Simple Webhook

```rust
use telegram_bot::ext::prelude::{ApplicationBuilder, Arc, Context, HandlerResult, Update};

// ... define handlers ...

#[tokio::main]
async fn main() {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let webhook_url = std::env::var("WEBHOOK_URL").unwrap();

    let app = ApplicationBuilder::new().token(token).build();

    // ... register handlers ...

    app.run_webhook(&webhook_url, "0.0.0.0:8443").await.unwrap();
}
```

### Custom Webhook with axum

For full control, use the manual lifecycle and run your own axum server alongside the bot. This lets you add health checks, custom endpoints, and other routes. See the [Webhooks guide](../guides/webhooks.md) for a complete example.

## The Application Lifecycle

Whether you use polling or webhooks, the application goes through these stages:

```text
initialize  -->  start  -->  idle  -->  stop  -->  shutdown
```

| Stage | What Happens |
|---|---|
| `initialize()` | Calls `getMe` to validate the token, loads persistence data, runs `post_init` hook |
| `start()` | Begins the update processing loop |
| idle | The bot is running and handling updates |
| `stop()` | Signals the processing loop to stop, runs `post_stop` hook |
| `shutdown()` | Flushes persistence, cleans up resources, runs `post_shutdown` hook |

When you call `run_polling()` or `run_webhook()`, all of these stages are managed automatically. You only need to call them manually if you are building a custom server setup.

## Lifecycle Hooks

You can register hooks that run at specific lifecycle stages:

```rust
use telegram_bot::ext::prelude::{ApplicationBuilder, Arc};

let post_init = Arc::new(|app: Arc<_>| Box::pin(async move {
    println!("Bot initialized! Username: {:?}",
        app.bot().bot_data().and_then(|d| d.username.clone()));
}) as std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>);

let app = ApplicationBuilder::new()
    .token(token)
    .post_init(post_init)
    .build();
```

Available hooks:
- `post_init` -- runs after `initialize()`.
- `post_stop` -- runs after `stop()`.
- `post_shutdown` -- runs after `shutdown()`.

## Concurrent Update Processing

By default, updates are processed one at a time. For bots with high traffic, enable concurrent processing:

```rust
let app = ApplicationBuilder::new()
    .token(token)
    .concurrent_updates(8)
    .build();
```

This allows up to 8 updates to be processed simultaneously. The value `0` is treated as `1`.

## Logging

Enable structured logging for debugging:

```rust
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // ... rest of your bot ...
}
```

Control verbosity with the `RUST_LOG` environment variable:

```sh
RUST_LOG=info TELEGRAM_BOT_TOKEN="..." cargo run
RUST_LOG=telegram_bot=debug cargo run
RUST_LOG=trace cargo run
```

## Next Steps

Now that your bot is running, learn about the building blocks:

- [Update](../core-concepts/update.md) -- what an incoming Telegram update looks like.
- [Handlers](../core-concepts/handlers.md) -- the dispatch system.
- [Filters](../core-concepts/filters.md) -- how to route updates to the right handler.
