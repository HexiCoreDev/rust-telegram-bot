# Troubleshooting

Common issues and their solutions when developing with `rust-telegram-bot`.

---

## Build Issues

### "thread 'main' has overflowed its stack"

**Cause:** The deeply nested async state machines produced by the Bot API call chain require more stack space than the default 2MB per thread.

**Solution:** Use `telegram_bot::run(async { ... })` instead of `#[tokio::main]`. This creates a runtime with 8MB thread stacks:

```rust
fn main() {
    telegram_bot::run(async {
        // your bot code here
    });
}
```

If you need to configure the runtime yourself:

```rust
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .thread_stack_size(8 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // your bot code here
        });
}
```

---

### "the trait bound `... : Future` is not satisfied"

**Cause:** Your handler function signature does not match the expected `async fn(Update, Context) -> HandlerResult`.

**Solution:** Verify your handler matches this exact signature:

```rust
use telegram_bot::ext::prelude::*;

async fn my_handler(update: Update, context: Context) -> HandlerResult {
    // ...
    Ok(())
}
```

Common mistakes:
- Missing the `async` keyword.
- Wrong return type (should be `HandlerResult`, which is `Result<(), HandlerError>`).
- Importing `Update` from the wrong path (use the prelude).
- Extra parameters (the standard signature only takes `Update` and `Context`).

If your handler needs extra state, capture it via closure:

```rust
let shared_state = Arc::new(RwLock::new(HashMap::new()));
let state = Arc::clone(&shared_state);

app.add_typed_handler(
    FnHandler::new(
        |u| u.message.is_some(),
        move |update, ctx| {
            let state = Arc::clone(&state);
            async move { my_handler(update, ctx, state).await }
        },
    ),
    0,
).await;
```

---

### Slow compilation

**Cause:** First build compiles all dependencies from source. The `telegram-bot-raw` crate has many type definitions.

**Solutions:**

1. Use `cargo build` without `--release` for development. Debug builds are significantly faster to compile.

2. Enable incremental compilation (on by default for debug builds).

3. Use `sccache` or `mold` linker for faster link times:

```sh
# Install mold
sudo apt install mold

# Use it
RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=mold" cargo build
```

4. Use `cargo watch -x run` for automatic recompilation on file changes:

```sh
cargo install cargo-watch
cargo watch -x run
```

---

### Feature flag compilation errors

**Cause:** Some features require additional dependencies.

**Solutions:**

- `persistence-sqlite` requires a C compiler for bundled SQLite. Install `build-essential` (Debian/Ubuntu) or `base-devel` (Arch).
- `webhooks` requires `axum` and `tokio-net`. These are pulled in automatically by the feature flag.

---

## Runtime Issues

### Bot does not respond to messages

**Possible causes and solutions:**

1. **Wrong token.** Verify with `curl https://api.telegram.org/bot<TOKEN>/getMe`. If it returns an error, the token is invalid.

2. **Webhook is set.** If the bot was previously running in webhook mode, Telegram will not deliver updates via polling. Delete the webhook:

```sh
curl "https://api.telegram.org/bot<TOKEN>/deleteWebhook"
```

3. **Privacy mode.** In groups, bots with privacy mode enabled (the default) only see:
   - Messages starting with `/`
   - Replies to the bot's messages
   - Messages mentioning the bot

   To see all messages, disable privacy mode via `@BotFather -> /setprivacy -> Disable`.

4. **Handler not matching.** Add a catch-all handler to verify updates are being received:

```rust
app.add_typed_handler(
    FnHandler::on_any(|update, _ctx| async move {
        println!("Received update: {update:?}");
        Ok(())
    }),
    99, // high group number so it runs after your real handlers
).await;
```

5. **Bot blocked by user.** The user may have blocked the bot. The bot cannot send messages to users who have not started it or who have blocked it.

6. **`TELEGRAM_BOT_TOKEN` not set.** Ensure the environment variable is exported in your shell session. On Linux/macOS: `export TELEGRAM_BOT_TOKEN="..."`.

---

### "Network error" or "Connection refused"

**Cause:** The bot cannot reach the Telegram API servers.

**Solutions:**

1. Check your internet connection.
2. Verify DNS resolution: `nslookup api.telegram.org`.
3. Check for firewall rules blocking outbound HTTPS (port 443).
4. If behind a proxy, configure the HTTP client accordingly.
5. In Docker, ensure the container has network access (`--network host` or proper networking).

---

### "Bad Request: message is not modified"

**Cause:** You called `edit_message_text` with the same text the message already has.

**Solution:** Track the current message text and only edit when it changes. Or catch the error:

```rust
match context.bot().edit_message_text(&new_text).chat_id(cid).message_id(mid).send().await {
    Ok(_) => {}
    Err(e) if e.to_string().contains("not modified") => {} // ignore
    Err(e) => return Err(HandlerError::Other(Box::new(e))),
}
```

---

### "Bad Request: query is too old and response timeout expired"

**Cause:** You took too long to call `answer_callback_query` after receiving a callback query. Telegram expects a response within 30 seconds.

**Solution:** Call `answer_callback_query` as early as possible in your callback handler:

```rust
async fn button(update: Update, context: Context) -> HandlerResult {
    let cq = update.callback_query.as_ref().unwrap();

    // Answer immediately, then do processing
    context.bot().answer_callback_query(&cq.id).send().await?;

    // ... expensive processing ...
    Ok(())
}
```

---

### "Conflict: terminated by other getUpdates request"

**Cause:** Two instances of your bot are running simultaneously with the same token, both trying to poll for updates.

**Solution:** Stop all other instances. Only one process should poll with a given token. In webhook mode, only one server should be registered.

To check: `curl "https://api.telegram.org/bot<TOKEN>/getWebhookInfo"`. If `url` is set, another instance (or a previous run) registered a webhook.

---

### Callback query data is empty or unexpected

**Cause:** The `callback_data` field has a 64-byte limit in the Telegram API. If you are storing complex data, it may be truncated.

**Solution:** Use the arbitrary callback data feature:

```rust
let app = ApplicationBuilder::new()
    .token(token)
    .arbitrary_callback_data(512) // cache up to 512 entries
    .build();
```

This stores the actual data in an in-memory cache and sends a short UUID as the callback_data. The framework transparently resolves it back.

---

### Persistence data not saved

**Possible causes:**

1. **Persistence not configured.** Verify you are passing a persistence backend to the builder:

```rust
let app = ApplicationBuilder::new()
    .token(token)
    .persistence(Box::new(persistence))
    .build();
```

2. **Bot crashed before flush.** The default flush interval is 60 seconds. Data written between flushes is lost on crash. For critical data, consider `SqlitePersistence` with WAL mode, which flushes more frequently.

3. **File permissions.** Ensure the bot process has write access to the persistence file's directory.

4. **Wrong data store.** Make sure you are using `context.set_user_data()` / `context.set_chat_data()` (which are tracked by persistence) rather than a separate data structure that persistence does not know about.

---

### Job queue jobs not firing

**Possible causes:**

1. **JobQueue not registered.** Pass it to the builder:

```rust
let jq = Arc::new(JobQueue::new());
let app = ApplicationBuilder::new()
    .token(token)
    .job_queue(Arc::clone(&jq))
    .build();
```

2. **JobQueue not started.** If you are using manual lifecycle management, ensure `app.initialize().await?` is called. The job queue starts during initialization.

3. **Job removed before firing.** Check that `schedule_removal()` is not being called prematurely.

---

## Webhook Issues

### Webhook not receiving updates

1. **URL must be HTTPS.** Telegram only sends updates to HTTPS endpoints.

2. **Port restrictions.** Telegram webhooks only work on ports 443, 80, 8443, or 88.

3. **Self-signed certificates.** If using a self-signed certificate, you must upload it when setting the webhook. Most cloud platforms provide automatic TLS and do not have this issue.

4. **Verify webhook status:**

```sh
curl "https://api.telegram.org/bot<TOKEN>/getWebhookInfo"
```

Look for:
- `url` should be your webhook URL.
- `last_error_date` and `last_error_message` for recent failures.
- `pending_update_count` for queued updates.

5. **Test with a health check.** Ensure your server is reachable from the internet:

```sh
curl https://your.domain/health
```

---

### "Wrong response from the webhook: 502 Bad Gateway"

**Cause:** Your server is returning an error or is not running.

**Solution:** Ensure your axum server is running and the `/telegram` endpoint returns 200 OK for valid updates.

---

## Getting Help

If your issue is not listed here:

1. Check the [FAQ](FAQ) page.
2. Search existing [GitHub issues](https://github.com/HexiCoreDev/rust-telegram-bot/issues).
3. Open a new issue with:
   - Your Rust version (`rustc --version`)
   - The exact error message
   - A minimal code example that reproduces the issue
   - Your OS and platform
