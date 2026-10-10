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
rust-tg-bot = { version = "1.0.0-rc.3", features = ["webhooks"] }
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

The secret check runs *before* the request body is read, so an unauthenticated caller is rejected on its headers alone and cannot make your server buffer a large body. If you write a custom handler instead (see below), keep that ordering: validating inside the handler body means the body is already in memory by the time you look at the header.

## Getting a Public HTTPS URL

Telegram will only deliver updates to a **publicly reachable HTTPS** endpoint. Your bot almost certainly listens on `127.0.0.1` on your own machine, so something has to forward that port to the internet with a valid certificate.

In production this is just your server's DNS name. For development and testing you need a tunnel. All of the options below produce an `https://…` URL that Telegram accepts.

### ngrok

The most widely used option. Free accounts get a random subdomain that changes on every restart.

```sh
# install: https://ngrok.com/download
ngrok http 8443
# → Forwarding https://a1b2c3d4.ngrok-free.app -> http://localhost:8443
```

Set that URL as `WEBHOOK_URL`. ngrok's free tier shows an interstitial warning page in the browser, but Telegram's POST requests are unaffected. If you hit a rate limit, reserve a static subdomain in the ngrok dashboard.

### zrok

Open source and self-hostable, which makes it a good fit if you would rather not send traffic through a third party.

```sh
zrok share public --headless http://127.0.0.1:8443
# → token:"sja0n87iyyab"  frontendEndpoints:"sja0n87iyyab.shares.zrok.io"
```

The URL is `https://<frontendEndpoint>`. Delete the share when you are done:

```sh
zrok delete share sja0n87iyyab
```

Note that `zrok share public` re-execs itself as a background process, so killing the foreground command does not stop the share — delete it explicitly as above.

### Cloudflare Tunnel

Good if your bot already lives behind Cloudflare. A *quick* tunnel needs no
account at all — just the binary — and hands you a random hostname:

```sh
cloudflared tunnel --url http://127.0.0.1:8443
# → https://random-words.trycloudflare.com
```

One gotcha: `cloudflared` prints the URL **before** the hostname has resolved
in DNS. If you register the webhook the instant it prints, Telegram's
verification request can hit "Name or service not known" and fail. Wait until
the tunnel answers any HTTP status before pointing your bot at it. For
production, create a named tunnel instead of a quick one — quick tunnels have
no uptime guarantee.

### localhost.run

No install and no account — useful for a one-off check.

```sh
ssh -R 80:localhost:8443 nokey@localhost.run
```

### Comparing them

| Tool | Account needed | Stable URL | Self-hostable | Cost |
|---|---|---|---|---|
| ngrok | Yes | Paid tier | No | Free tier / paid |
| zrok | Yes | With reservation | Yes | Free / self-host |
| Cloudflare Tunnel | No (quick) / Yes (named) | Yes (named) | Yes | Free |
| localhost.run | No | No | No | Free |

The harness in the [example index](../examples.md) prefers Cloudflare Tunnel
and falls back to zrok, so a throttled provider does not block a test run.

### Pointing the bot at the tunnel

`WebhookConfig` keeps the public URL and the local mount path in **separate** fields and does not join them. The registered URL must already include the path, or Telegram will probe a path that does not exist and reject the registration with `[setWebhook] Not Found`.

```rust
let path = "/telegram/webhook";
let config = WebhookConfig {
    listen: "127.0.0.1".into(),
    port: 8443,
    url_path: path.into(),
    // base + path, not just the base
    webhook_url: Some(format!("{}{path}", std::env::var("WEBHOOK_URL").unwrap())),
    secret_token: Some(std::env::var("WEBHOOK_SECRET").unwrap()),
    ..Default::default()
};
```

A complete runnable version is in
[`webhook_echo_bot.rs`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/webhook_echo_bot.rs):

```sh
export TELEGRAM_BOT_TOKEN="your-token"
export WEBHOOK_URL="https://your-tunnel-host"
export WEBHOOK_SECRET="$(openssl rand -hex 32)"
export PORT=8443

cargo run -p rust-tg-bot --example webhook_echo_bot --features webhooks
```

Confirm Telegram accepted it at any time:

```sh
curl "https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/getWebhookInfo"
```

`url` should be your tunnel URL and `last_error_message` should be empty. Telegram never echoes the secret back, so this cannot confirm the secret is registered — only that the URL resolved.

When you go back to long polling, remember to remove the webhook, or `getUpdates` will fail with `409 Conflict`:

```sh
curl "https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/deleteWebhook"
```

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
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    // A custom server gets no secret validation for free: without this check
    // anyone who learns the URL can inject arbitrary updates.
    match headers.get("x-telegram-bot-api-secret-token").and_then(|v| v.to_str().ok()) {
        Some(provided) if provided == &*state.secret_token => {}
        _ => return StatusCode::FORBIDDEN,
    }

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
    /// Shared with `set_webhook`, so Telegram echoes it on every request.
    secret_token: Arc<str>,
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

    // A secret token is required: without one the endpoint is unauthenticated.
    let secret_token: Arc<str> = Arc::from(
        std::env::var("WEBHOOK_SECRET").expect("WEBHOOK_SECRET must be set"),
    );

    // Set the webhook on Telegram's side
    let full_url = format!("{webhook_url}/telegram");
    app.bot()
        .set_webhook(&full_url)
        .secret_token(secret_token.to_string())
        .await
        .expect("Failed to set webhook");

    // Build the axum router
    let state = AppState {
        update_tx: app.update_sender(),
        app: Arc::clone(&app),
        secret_token: Arc::clone(&secret_token),
    };

    let router = Router::new()
        .route("/telegram", post(handle_webhook))
        .route("/healthcheck", get(healthcheck))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8443").await.unwrap();

    println!("Custom webhook server listening on 0.0.0.0:8443");

    // Docker and systemd stop containers with SIGTERM, not Ctrl+C. Without this
    // the process is killed mid-request and persistence is never flushed.
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    app.stop().await.ok();
    app.shutdown().await.ok();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut s) => {
                s.recv().await;
            }
            Err(_) => std::future::pending::<()>().await,
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }

    println!("Shutdown signal received");
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
        # nginx lowercases incoming header names, so the variable is
        # $http_x_telegram_bot_api_secret_token -- not $http_x_rust_tg_bot_...,
        # which would be empty and silently blank the header, breaking validation.
        proxy_set_header X-Telegram-Bot-Api-Secret-Token $http_x_telegram_bot_api_secret_token;
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

## Runnable examples

The code above is abridged. These run in full:

| Example | What to look at |
|---|---|
| [`webhook_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/webhook_bot.rs) | the minimal webhook deployment |
| [`webhook_echo_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/webhook_echo_bot.rs) | a secret-token-protected echo bot |
| [`custom_webhook_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/custom_webhook_bot.rs) | serving your own axum routes alongside the webhook |

Every example is listed in the [example index](../examples.md).
