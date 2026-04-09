# Webhooks

Long polling is great for development, but production bots should use webhooks. Instead of your bot repeatedly asking Telegram "any new updates?", Telegram pushes updates to your server the moment they arrive.

## Polling vs Webhooks

| | Polling | Webhooks |
|---|---|---|
| Setup | Zero config | Requires HTTPS endpoint |
| Latency | Depends on poll interval | Near-instant |
| Resource usage | Constant network calls | Idle until update arrives |
| Best for | Development, low-traffic bots | Production, high-traffic bots |

## Built-in Webhook Server

The framework includes a built-in webhook server. Enable it with the `webhooks` feature:

```toml
[dependencies]
rust-tg-bot = { version = "1.0.0-beta.4c", features = ["webhooks"] }
```

Then use `run_webhook()` instead of `run_polling()`:

```rust
use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, Arc, CommandHandler, Context, HandlerResult,
    Update, WebhookConfig,
};

async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    context.reply_text(&update, "Hello from a webhook bot!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();

    let app = ApplicationBuilder::new().token(token).build();

    app.add_handler(CommandHandler::new("start", start), 0).await;

    let config = WebhookConfig {
        listen: "0.0.0.0".into(),
        port: 8443,
        url_path: "/webhook".into(),
        webhook_url: Some("https://your.domain/webhook".into()),
        secret_token: Some("my-secret-token".into()),
        ..Default::default()
    };

    app.run_webhook(config).await.unwrap();
}
```

## WebhookConfig

The `WebhookConfig` struct controls all webhook behaviour:

| Field | Type | Default | Purpose |
|---|---|---|---|
| `listen` | `String` | `"127.0.0.1"` | Address to bind the HTTP server |
| `port` | `u16` | `80` | Port to listen on |
| `url_path` | `String` | `""` | URL path for the webhook endpoint |
| `webhook_url` | `Option<String>` | `None` | Full public URL Telegram will POST to |
| `secret_token` | `Option<String>` | `None` | Token for validating requests from Telegram |
| `bootstrap_retries` | `i32` | `0` | Retries when setting the webhook on Telegram |
| `drop_pending_updates` | `bool` | `false` | Discard updates that arrived while offline |
| `allowed_updates` | `Option<Vec<String>>` | `None` | Filter which update types you receive |
| `max_connections` | `u32` | `40` | Max simultaneous connections from Telegram |

## Secret Token Validation

Telegram sends a `X-Telegram-Bot-Api-Secret-Token` header with every webhook request. When you set `secret_token` in `WebhookConfig`, the built-in server automatically validates this header and rejects requests that do not match.

```rust
let config = WebhookConfig {
    secret_token: Some("a-random-string-only-you-know".into()),
    ..Default::default()
};
```

Generate a strong random secret at deployment time, for example with `openssl rand -hex 32`. Always set a secret token in production to prevent third parties from sending fake updates to your endpoint.

## Custom Webhook with Axum

For bots that need custom HTTP routes alongside the Telegram webhook, bypass the built-in server and run your own axum application. This gives you full control over routing, middleware, and additional endpoints.

```rust
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

use rust_tg_bot::ext::prelude::{
    Application, ApplicationBuilder, Arc, CommandHandler, Context,
    HandlerResult, ParseMode, Update,
};
use rust_tg_bot::raw::types::update::Update as RawUpdate;

async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = update
        .effective_chat()
        .map(|c| c.id)
        .expect("start command must originate from a chat");

    context
        .bot()
        .send_message(chat_id, "Hello from a custom webhook server!")
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}

async fn handle_webhook(
    axum::extract::State(state): axum::extract::State<AppState>,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    let update: RawUpdate = match serde_json::from_slice(&body) {
        Ok(u) => u,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    if let Err(e) = state.update_tx.send(update).await {
        tracing::error!("Failed to enqueue update: {e}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

async fn healthcheck() -> &'static str {
    "The bot is running fine"
}

#[derive(Clone)]
struct AppState {
    update_tx: mpsc::Sender<RawUpdate>,
    app: Arc<Application>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let webhook_url = std::env::var("WEBHOOK_URL").unwrap();

    let app = ApplicationBuilder::new().token(&token).build();

    app.add_handler(CommandHandler::new("start", start), 0).await;

    // Initialize and start the Application manually
    app.initialize().await.expect("Failed to initialize");
    app.start().await.expect("Failed to start");

    // Set the webhook on Telegram's side
    let full_url = format!("{webhook_url}/telegram");
    app.bot()
        .set_webhook(&full_url)
        .await
        .expect("Failed to set webhook");

    // Build the axum router
    let state = AppState {
        update_tx: app.update_sender(),
        app: Arc::clone(&app),
    };

    let router = Router::new()
        .route("/telegram", post(handle_webhook))
        .route("/healthcheck", get(healthcheck))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8443").await.unwrap();

    println!("Custom webhook server listening on 0.0.0.0:8443");

    axum::serve(listener, router).await.unwrap();

    // Cleanup on shutdown
    app.stop().await.ok();
    app.shutdown().await.ok();
}
```

The key steps when running a custom server:

1. Call `app.initialize()` and `app.start()` instead of `run_polling()` or `run_webhook()`.
2. Set the webhook URL on Telegram with `app.bot().set_webhook(url).await`.
3. Use `app.update_sender()` to get the `mpsc::Sender` channel and forward parsed updates into it.
4. Call `app.stop()` and `app.shutdown()` when your server exits.

## Production Setup Behind a Reverse Proxy

In production, you typically run the bot behind nginx or another reverse proxy that handles TLS termination.

### Nginx Configuration

```nginx
server {
    listen 443 ssl http2;
    server_name bot.example.com;

    ssl_certificate     /etc/letsencrypt/live/bot.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/bot.example.com/privkey.pem;

    location /webhook {
        proxy_pass http://127.0.0.1:8080/webhook;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Telegram-Bot-Api-Secret-Token $http_x_rust_tg_bot_api_secret_token;
    }
}
```

Then configure your bot to listen on localhost:

```rust
let config = WebhookConfig {
    listen: "127.0.0.1".into(),
    port: 8080,
    url_path: "/webhook".into(),
    webhook_url: Some("https://bot.example.com/webhook".into()),
    secret_token: Some("my-secret-token".into()),
    ..Default::default()
};
```

### Allowed Ports

Telegram only sends webhook requests to these ports: **443, 80, 88, 8443**. When using a reverse proxy, only the proxy needs to listen on one of these ports. Your bot can listen on any internal port.

## Switching Between Modes

A common pattern is to use polling in development and webhooks in production:

```rust
#[tokio::main]
async fn main() {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let app = ApplicationBuilder::new().token(token).build();

    // ... register handlers ...

    if let Ok(webhook_url) = std::env::var("WEBHOOK_URL") {
        let config = WebhookConfig {
            webhook_url: Some(webhook_url),
            secret_token: std::env::var("WEBHOOK_SECRET").ok(),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8443".into())
                .parse()
                .unwrap(),
            ..Default::default()
        };
        app.run_webhook(config).await.unwrap();
    } else {
        app.run_polling().await.unwrap();
    }
}
```

## Removing a Webhook

To switch back to polling, you must delete the webhook first. The framework does this automatically when you call `run_polling()`, but you can also do it manually:

```rust
app.bot().delete_webhook(false).await?;
```

The boolean argument controls whether to drop pending updates (`true`) or keep them (`false`).

## Next Steps

- [Deployment](../advanced/deployment.md) -- full production deployment strategies including Docker and systemd.
- [Error Handling](../advanced/error-handling.md) -- handle webhook failures gracefully.
