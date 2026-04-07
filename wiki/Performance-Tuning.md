# Performance Tuning

This page covers strategies for minimizing memory usage, maximizing throughput, and optimizing your bot for production workloads.

---

## Baseline Numbers

Measured on a typical x86_64 Linux system with the echo_bot example:

| Metric | Debug Build | Release Build | Release + LTO + Strip |
|--------|------------|---------------|----------------------|
| Binary size | ~80 MB | ~20 MB | ~10 MB |
| RSS at idle | ~40 MB | ~15 MB | ~15 MB |
| RSS under load | ~60 MB | ~20-27 MB | ~20-27 MB |
| Cold start | ~2 s | ~500 ms | ~400 ms |
| Updates/second | ~200 | ~1,000+ | ~1,000+ |

Always deploy release builds. Debug builds are 4-8x slower and use 2-3x more memory.

---

## Binary Size Optimization

### Release Profile

Add to your `Cargo.toml`:

```toml
[profile.release]
lto = true          # Link-time optimization (merges all code into one compilation unit)
codegen-units = 1   # Single codegen unit for maximum optimization
strip = true        # Strip debug symbols from the binary
opt-level = "z"     # Optimize for size instead of speed (optional, use "3" for speed)
panic = "abort"     # Removes unwind tables (saves ~10% binary size)
```

| Setting | Impact | Trade-off |
|---------|--------|-----------|
| `lto = true` | -20-40% binary size | Longer compile times |
| `codegen-units = 1` | -5-10% binary size | Longer compile times |
| `strip = true` | -30-50% binary size | No debug info in crash dumps |
| `opt-level = "z"` | -5-15% binary size | Slightly slower runtime |
| `panic = "abort"` | -5-10% binary size | No catch_unwind support |

### Post-build stripping

If you did not set `strip = true` in the profile:

```sh
strip target/release/my_bot
```

### UPX compression (optional)

For minimal deployment images:

```sh
upx --best target/release/my_bot
```

This reduces binary size by another 50-70% at the cost of slower startup (decompression).

---

## Memory Optimization

### Understand RSS components

The bot's RSS is composed of:

| Component | Typical Size | Notes |
|-----------|-------------|-------|
| Binary code + static data | 5-8 MB | Fixed; determined by binary size |
| tokio runtime | 3-5 MB | Thread stacks (8 MB each, but RSS only counts used pages) |
| HTTP client (reqwest) | 2-3 MB | TLS buffers, connection pools |
| Application state | 1-5 MB | Handlers, data stores, pending updates |
| Per-user/chat data | Variable | Depends on your data model |

### Reduce concurrent updates

The `concurrent_updates` setting controls how many updates are processed simultaneously:

```rust
let app = ApplicationBuilder::new()
    .token(token)
    .concurrent_updates(4) // default is 1
    .build();
```

Each concurrent update requires its own task stack and context. For memory-constrained environments, keep this low (1-4). For high-throughput bots on machines with ample RAM, increase it (8-32).

### Control data store growth

User data and chat data grow with the number of unique users/chats. For bots with many users:

1. **Use SQLite persistence** instead of JSON files. SQLite handles large datasets more efficiently.

2. **Clean up stale data.** Periodically remove data for inactive users:

```rust
// In a scheduled job or lifecycle hook
let mut store = context.bot_data_mut().await;
// Remove entries older than 30 days, etc.
```

3. **Store minimal data.** Keep only what you need. Avoid storing full message history.

### Drop guards promptly

`DataReadGuard` and `DataWriteGuard` hold async locks. Long-held locks block other handlers:

```rust
// Bad: holding the lock across an await point
let guard = context.bot_data().await;
let value = guard.get_str("key").unwrap().to_string();
context.bot().send_message(chat_id, &value).send().await?;  // lock held during network I/O!

// Good: drop the guard before doing I/O
let value = {
    let guard = context.bot_data().await;
    guard.get_str("key").unwrap_or("").to_string()
};
context.bot().send_message(chat_id, &value).send().await?;  // lock released
```

---

## Throughput Optimization

### Concurrent update processing

For high-traffic bots, increase concurrent updates:

```rust
let app = ApplicationBuilder::new()
    .token(token)
    .concurrent_updates(16)
    .build();
```

This allows up to 16 updates to be processed in parallel. The optimal value depends on your bot's workload:

- **I/O-bound bots** (making API calls, database queries): Higher values (8-32) are beneficial.
- **CPU-bound bots** (heavy computation): Match the number of CPU cores.
- **Memory-constrained environments**: Keep low (1-4).

### Use webhooks for lower latency

Polling introduces a baseline latency (the interval between poll requests). Webhooks deliver updates instantly as they arrive.

For latency-sensitive bots, always use webhook mode in production.

### Batch operations

When sending messages to many users, avoid blocking the handler pipeline:

```rust
// Spawn a background task for bulk operations
let bot = Arc::clone(context.bot());
let subscribers: Vec<i64> = {
    let guard = context.bot_data().await;
    guard.get_id_set("subscribers").into_iter().collect()
};

tokio::spawn(async move {
    for chat_id in subscribers {
        let _ = bot.send_message(chat_id, "Broadcast message").send().await;
        // Respect rate limits: ~30 messages/second
        tokio::time::sleep(std::time::Duration::from_millis(35)).await;
    }
});
```

### Pre-compute expensive values

If your handler performs expensive computations that do not depend on the update, compute them once and share via `bot_data`:

```rust
// At startup (in a post_init hook or before run_polling)
{
    let mut guard = context.bot_data_mut().await;
    guard.set_str("cached_result", &expensive_computation());
}

// In handlers (fast read)
let guard = context.bot_data().await;
let result = guard.get_str("cached_result").unwrap();
```

---

## Compile-Time Optimization

### Minimize feature flags

Only enable the features you actually use. Each feature pulls in additional dependencies:

```toml
# Only JSON persistence
telegram-bot = { git = "...", features = ["persistence-json"] }

# Only webhooks
telegram-bot = { git = "...", features = ["webhooks"] }

# Everything (larger binary, slower compilation)
telegram-bot = { git = "...", features = ["persistence-json", "persistence-sqlite", "webhooks"] }
```

### Use cargo-chef for Docker caching

In Docker builds, separate dependency compilation from application compilation:

```dockerfile
FROM rust:1.85-slim AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin my_bot

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/my_bot /usr/local/bin/my_bot
CMD ["my_bot"]
```

This caches the dependency compilation layer. Only the final `cargo build` reruns when your code changes.

---

## Monitoring Performance

### Enable tracing metrics

Use `tracing` to instrument your handlers:

```rust
use tracing::instrument;
use std::time::Instant;

async fn my_handler(update: Update, context: Context) -> HandlerResult {
    let start = Instant::now();

    // ... handler logic ...

    let elapsed = start.elapsed();
    tracing::info!(duration_ms = elapsed.as_millis(), "Handler completed");
    Ok(())
}
```

### Monitor memory with /proc

On Linux, monitor your bot's memory usage:

```sh
# RSS in kilobytes
cat /proc/$(pidof my_bot)/status | grep VmRSS

# Or use ps
ps -o rss,vsz -p $(pidof my_bot)
```

### Prometheus metrics (advanced)

For production monitoring, expose Prometheus metrics:

```rust
use std::sync::atomic::{AtomicU64, Ordering};

static UPDATES_PROCESSED: AtomicU64 = AtomicU64::new(0);
static ERRORS_TOTAL: AtomicU64 = AtomicU64::new(0);

async fn my_handler(update: Update, context: Context) -> HandlerResult {
    UPDATES_PROCESSED.fetch_add(1, Ordering::Relaxed);
    // ...
    Ok(())
}
```

Expose these via an HTTP endpoint in your webhook server.

---

## Resource Limits and Rate Limiting

### Telegram API rate limits

| Limit | Value |
|-------|-------|
| Messages per second per chat | 1 |
| Messages per second per bot | 30 |
| Messages per minute to a group | 20 |
| Inline query results | 50 per query |
| File upload | 50 MB |
| File download | 20 MB |

Exceeding these returns HTTP 429 with `retry_after`. For bulk operations, throttle your requests.

### OS-level limits

For high-traffic bots, increase file descriptor limits:

```sh
# /etc/security/limits.conf
bot soft nofile 65536
bot hard nofile 65536
```

Or in systemd:

```ini
[Service]
LimitNOFILE=65536
```

---

## Summary: Production Checklist

- [ ] Build with `--release`
- [ ] Enable LTO, single codegen unit, and stripping in `Cargo.toml`
- [ ] Use webhook mode for production
- [ ] Set `concurrent_updates` based on workload
- [ ] Use SQLite persistence for bots with many users
- [ ] Drop data guards before network I/O
- [ ] Set appropriate `RUST_LOG` level (info for production)
- [ ] Monitor RSS and update throughput
- [ ] Implement rate limiting for bulk operations
- [ ] Set OS file descriptor limits for high-traffic bots
