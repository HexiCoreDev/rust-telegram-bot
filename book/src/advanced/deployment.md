# Deployment

This chapter covers everything you need to deploy a rust-tg-bot application to production: release builds, containerisation, systemd service management, and operational concerns.

## Release Builds

Always deploy release builds. Debug builds are significantly larger and slower.

```sh
cargo build --release -p rust-tg-bot --example my_bot
```

The binary lands in `target/release/examples/my_bot`.

### Binary Size Optimisation

Add these settings to your workspace `Cargo.toml` for smaller binaries:

```toml
[profile.release]
lto = true          # Link-time optimisation
codegen-units = 1   # Single codegen unit for better optimisation
strip = true        # Strip debug symbols
opt-level = "z"     # Optimise for size (use "3" for speed)
panic = "abort"     # Smaller binary, no unwinding overhead
```

With these settings a typical bot binary compiles to approximately 6.2 MB (stripped) — smaller than teloxide at 6.6 MB. See the [benchmarks](../../../benchmarks/README.md) for measured numbers.

### Feature Flags

Only enable the features you use. Each feature pulls in additional dependencies:

```toml
[dependencies]
rust-tg-bot = { version = "1.0.0-beta.4c", features = ["persistence-sqlite", "webhooks"] }
```

Available features:

| Feature | Dependencies Added |
|---|---|
| `webhooks` | axum, hyper |
| `job-queue` | tokio-cron-scheduler |
| `persistence-json` | (minimal -- serde_json already present) |
| `persistence-sqlite` | rusqlite (with bundled SQLite) |
| `rate-limiter` | governor |

## Docker

### Minimal Dockerfile

Use a multi-stage build to keep the final image small:

```dockerfile
# Build stage
FROM rust:1.83-slim AS builder

WORKDIR /app
COPY . .

RUN cargo build --release -p rust-tg-bot --example my_bot \
    --features "persistence-sqlite,webhooks"

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/examples/my_bot /usr/local/bin/my_bot

ENV RUST_LOG=info

ENTRYPOINT ["my_bot"]
```

### Docker Compose

```yaml
version: "3.8"

services:
  bot:
    build: .
    restart: unless-stopped
    environment:
      TELEGRAM_BOT_TOKEN: "${TELEGRAM_BOT_TOKEN}"
      WEBHOOK_URL: "https://bot.example.com"
      WEBHOOK_SECRET: "${WEBHOOK_SECRET}"
      RUST_LOG: "info"
    ports:
      - "8443:8443"
    volumes:
      - bot-data:/data
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8443/healthcheck"]
      interval: 30s
      timeout: 5s
      retries: 3

volumes:
  bot-data:
```

### Persistence in Docker

When using `JsonFilePersistence` or `SqlitePersistence`, mount a volume so data survives container restarts:

```rust
let persistence = SqlitePersistence::open("/data/bot.db")
    .expect("failed to open database");
```

Map `/data` to a Docker volume as shown in the compose file above.

## systemd

For bare-metal or VM deployments, run the bot as a systemd service.

### Service File

Create `/etc/systemd/system/rust-tg-bot.service`:

```ini
[Unit]
Description=Telegram Bot
After=network-online.target
Wants=network-online.target

[Service]
Type=exec
User=bot
Group=bot
WorkingDirectory=/opt/rust-tg-bot
ExecStart=/opt/rust-tg-bot/my_bot
Restart=always
RestartSec=5

# Environment
EnvironmentFile=/opt/rust-tg-bot/.env

# Security hardening
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/opt/rust-tg-bot/data
PrivateTmp=true

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=rust-tg-bot

[Install]
WantedBy=multi-user.target
```

### Environment File

Create `/opt/rust-tg-bot/.env`:

```sh
TELEGRAM_BOT_TOKEN=your-token-here
WEBHOOK_URL=https://bot.example.com
WEBHOOK_SECRET=your-secret-here
RUST_LOG=info
```

### Managing the Service

```sh
# Enable on boot
sudo systemctl enable rust-tg-bot

# Start
sudo systemctl start rust-tg-bot

# Check status
sudo systemctl status rust-tg-bot

# View logs
sudo journalctl -u rust-tg-bot -f

# Restart after deploy
sudo systemctl restart rust-tg-bot
```

## Webhook vs Polling in Production

### When to Use Polling

- Simple bots with low traffic.
- Development and staging environments.
- Deployments without a public IP or domain.
- When you cannot provision TLS certificates.

### When to Use Webhooks

- Production bots handling more than a few messages per second.
- Serverless or container environments where idle resource usage matters.
- When you need the lowest possible latency.
- When you already run a web server alongside the bot.

### Switching Modes by Environment

```rust
#[tokio::main]
async fn main() {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let app = ApplicationBuilder::new().token(token).build();

    // ... register handlers ...

    if let Ok(webhook_url) = std::env::var("WEBHOOK_URL") {
        let config = WebhookConfig {
            listen: "0.0.0.0".into(),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8443".into())
                .parse()
                .unwrap(),
            webhook_url: Some(webhook_url),
            secret_token: std::env::var("WEBHOOK_SECRET").ok(),
            ..Default::default()
        };
        app.run_webhook(config).await.unwrap();
    } else {
        println!("No WEBHOOK_URL set, falling back to polling.");
        app.run_polling().await.unwrap();
    }
}
```

## Monitoring with tracing

The framework uses the `tracing` crate internally. Configure a subscriber in `main` to capture structured logs:

```rust
#[tokio::main]
async fn main() {
    // Basic stderr logging
    tracing_subscriber::fmt::init();

    // Or with environment filter
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env(),
        )
        .json()  // JSON output for log aggregation
        .init();

    // ...
}
```

Set the log level via the `RUST_LOG` environment variable:

```sh
RUST_LOG=info                    # Application info and above
RUST_LOG=rust_tg_bot=debug      # Debug logs from the framework
RUST_LOG=my_bot=trace,info       # Trace for your code, info for everything else
```

### Key Events to Monitor

| Event | What to Watch |
|---|---|
| Handler errors | Spikes indicate bugs or API issues |
| Update processing time | Latency degradation |
| Webhook delivery failures | Network or TLS problems |
| Persistence flush failures | Disk space or database issues |
| Rate limit responses (HTTP 429) | Too many API calls |

## Graceful Shutdown

The framework handles `SIGTERM` and `SIGINT` (Ctrl+C) automatically when using `run_polling()` or `run_webhook()`. For custom servers, handle shutdown explicitly:

```rust
use std::sync::Arc;
use tokio::sync::Notify;

let stop = Arc::new(Notify::new());
let stop_signal = Arc::clone(&stop);

tokio::spawn(async move {
    tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    tracing::info!("Received shutdown signal");
    stop_signal.notify_waiters();
});

// In your server loop:
let shutdown = Arc::clone(&stop);
axum::serve(listener, router)
    .with_graceful_shutdown(async move {
        shutdown.notified().await;
    })
    .await
    .unwrap();

// Teardown
app.stop().await.ok();
app.shutdown().await.ok();
```

## Production Checklist

1. **Build in release mode** with LTO and stripping enabled.
2. **Set a webhook secret** to prevent spoofed updates.
3. **Use persistence** (`SqlitePersistence` or `JsonFilePersistence`) so data survives restarts.
4. **Configure `tracing`** with structured logging and an appropriate log level.
5. **Run behind a reverse proxy** (nginx, Caddy) for TLS termination and certificate renewal.
6. **Set up monitoring** -- alerting on handler errors and latency spikes.
7. **Use systemd or Docker** for process management and automatic restarts.
8. **Mount a persistent volume** if using file-based persistence in Docker.
9. **Enable only the features you need** to minimise binary size and attack surface.
10. **Test your shutdown path** -- verify that persistence is flushed on `SIGTERM`.

## Next Steps

- [Webhooks](../guides/webhooks.md) -- detailed webhook configuration.
- [Error Handling](error-handling.md) -- production error handling strategies.
- [Testing](testing.md) -- test before you deploy.
