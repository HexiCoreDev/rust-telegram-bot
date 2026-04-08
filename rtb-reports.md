# RTB Performance & DX Improvement Plan

Concrete changes to close the memory gap with teloxide while keeping PTB-flavored DX.

Current state (from benchmarks):

| Framework | Idle RSS | Under Load | Binary |
|-----------|:--------:|:----------:|:------:|
| teloxide 0.17 | **15 MB** | **17 MB** | **6.6 MB** |
| RTB 1.0.0-beta | 20 MB | 32 MB | 12 MB |

---

## 1. Cache `to_value()` per update

Every filter calls `to_value(update)` independently. A single update passing through `TEXT() & !COMMAND()` serializes the entire `Update` struct **3 times** (once for `TextAny`, once for `CommandFilter`, once for `NotFilter`'s inner). Across the benchmark's 4 handler groups, that's 8-12 full serializations per update.

Fix: compute once in `process_update()`, pass a reference.

```rust
// filters/base.rs — change the trait
pub trait Filter: Send + Sync {
    fn check_update(&self, update: &Update, cached: &Value) -> FilterResult;
    fn name(&self) -> &str;
}
```

```rust
// application.rs — compute once
pub async fn process_update(&self, update: Update) -> Result<(), ApplicationError> {
    let cached = serde_json::to_value(&update).unwrap_or_default();
    // ...
    if !(handler.check_update)(&update, &cached) { continue; }
```

```rust
// filters/text.rs — use the cached value
impl Filter for TextAny {
    fn check_update(&self, _update: &Update, cached: &Value) -> FilterResult {
        if effective_message_val(cached)
            .and_then(|m| m.get("text"))
            .and_then(|v| v.as_str())
            .is_some()
        { FilterResult::Match } else { FilterResult::NoMatch }
    }
}
```

Composed filters (`AndFilter`, `OrFilter`, etc.) pass the same `&Value` down. Per-update allocations drop by 70-80%.

---

## 2. `Arc<Update>` instead of `.clone()` in dispatch

`application.rs:460` and `application.rs:913` clone the full update per dispatch cycle. An `Update` contains nested `Message` structs with `String` fields — these clones are expensive.

```rust
pub async fn process_update(&self, update: Update) -> Result<(), ApplicationError> {
    let update = Arc::new(update);
    let cached = serde_json::to_value(update.as_ref()).unwrap_or_default();
    // ...
    let uc = Arc::clone(&update);
    // handler signature becomes (Arc<Update>, Context) -> HandlerResult
```

Breaking change to handler signatures, but correct. PTB passes update by reference too (Python objects are refcounted).

---

## 3. Actually feature-gate modules in `lib.rs`

Currently `lib.rs` compiles everything unconditionally. The feature flags are decorative.

```rust
// lib.rs — gate what's optional
pub mod application;
pub mod builder;
pub mod context;
pub mod context_types;
pub mod defaults;
pub mod ext_bot;
pub mod filters;
pub mod handlers;
pub mod prelude;
pub mod update_processor;
pub mod utils;

#[cfg(feature = "job-queue")]
pub mod job_queue;
#[cfg(feature = "rate-limiter")]
pub mod rate_limiter;
#[cfg(feature = "persistence-json")]
pub mod persistence;
pub mod callback_data_cache;  // only if arbitrary_callback_data is used
```

Then in `application.rs`, gate the field:

```rust
pub struct Application {
    // ...
    #[cfg(feature = "job-queue")]
    job_queue: Option<Arc<JobQueue>>,
    // ...
}
```

Remove `updater.rs` entirely — `Application` manages its own channels.

The benchmark running with just `default` features won't compile job queue, persistence, rate limiter, or the standalone updater. Smaller binary, fewer baseline allocations.

---

## 4. Shrink the reqwest connection pool

RTB defaults to 256 connections (matching PTB's httpx). A Telegram bot talks to one host (`api.telegram.org`).

```rust
// In Bot or ExtBot construction
reqwest::Client::builder()
    .pool_max_idle_per_host(8)   // was effectively 256
    .pool_idle_timeout(Duration::from_secs(30))
    .build()
```

teloxide's lower memory under load is partly this — fewer pre-allocated connection slots.

---

## 5. Rewrite the benchmark to use `run_webhook()`

The benchmark manually wires axum, mpsc channels, `Arc<Notify>`, and 4 lifecycle calls. But `Application::run_webhook()` already exists at `application.rs:677` and does all of this internally. The benchmark should be:

```rust
fn main() {
    telegram_bot::run(async {
        tracing_subscriber::fmt().with_max_level(tracing::Level::WARN).init();

        let token = std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN required");
        let webhook_url = std::env::var("WEBHOOK_URL").expect("WEBHOOK_URL required");

        let app = ApplicationBuilder::new().token(&token).build();

        app.add_typed_handler(CommandHandler::new("start", start), 0).await;
        app.add_typed_handler(CommandHandler::new("help", help_cmd), 0).await;
        app.add_typed_handler(FnHandler::on_callback_query(button_callback), 0).await;
        app.add_typed_handler(MessageHandler::new(TEXT() & !COMMAND(), echo), 0).await;

        app.bot().set_webhook(&format!("{webhook_url}/telegram")).await.unwrap();

        let config = WebhookConfig {
            listen: "127.0.0.1".into(),
            port: 8000,
            url_path: "/telegram".into(),
            secret_token: None,
        };

        println!("RTB benchmark bot running on port 8000.");
        app.run_webhook(config).await.unwrap();
    });
}
```

149 lines to ~30 lines. Also eliminates the duplicate axum instance — the benchmark was spinning up its own axum alongside the framework's, inflating memory numbers.

---

## 6. Migrate hot-path filters to typed access

Keep the `to_value()` cache from change #1 as the default path, but the two most-used filters (`TEXT` and `COMMAND`) should bypass JSON entirely:

```rust
impl Filter for TextAny {
    fn check_update(&self, update: &Update, _cached: &Value) -> FilterResult {
        update.effective_message()
            .and_then(|m| m.text.as_deref())
            .map(|_| FilterResult::Match)
            .unwrap_or(FilterResult::NoMatch)
    }
}
```

The hot path (every text message) never touches `serde_json`. The `_cached` parameter stays for filters that still need it.

---

## Expected impact

| Change | Memory | Binary | DX |
|---|---|---|---|
| Cache `to_value()` | -5-8 MB under load | -- | -- |
| `Arc<Update>` | -2-3 MB under load | -- | -- |
| Feature-gate modules | -2-3 MB idle | -3-4 MB | -- |
| Shrink connection pool | -3-5 MB under load | -- | -- |
| Use `run_webhook()` | -2 MB (no duplicate axum) | -- | 149 to 30 lines |
| Typed hot-path filters | -1-2 MB under load | -- | -- |

Target: close to teloxide's 15-17 MB range. Benchmark code reads almost as clean as PTB.

**Priority order:** changes 1, 4, and 5 are highest ROI — do those first, re-benchmark, then tackle the rest.
