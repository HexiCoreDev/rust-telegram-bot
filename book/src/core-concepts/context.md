# Context

The `Context` (aliased as `CallbackContext`) is passed to every handler alongside the `Update`. It provides access to the bot, user/chat data storage, the job queue, and convenience methods.

## Core Methods

### bot()

Access the bot instance to call Telegram API methods:

```rust
async fn my_handler(update: Arc<Update>, context: Context) -> HandlerResult {
    let bot = context.bot();
    bot.send_message(chat_id, "Hello!").await?;
    Ok(())
}
```

### reply_text()

A shortcut for sending a text reply to the chat that sent the update:

```rust
context.reply_text(&update, "Got it!").await?;
```

This is equivalent to:

```rust
let chat_id = update.effective_chat().map(|c| c.id).unwrap();
context.bot().send_message(chat_id, "Got it!").await?;
```

## User Data

Store and retrieve per-user data that persists across updates. When paired with a persistence backend, this data survives bot restarts.

### Reading User Data

```rust
let user_data = context.user_data().await.unwrap_or_default();
let name = user_data.get("name")
    .and_then(|v| v.as_str())
    .unwrap_or("unknown");
```

### Writing User Data

```rust
use serde_json::Value as JsonValue;

context.set_user_data(
    "name".to_string(),
    JsonValue::String("Alice".to_string()),
).await;
```

### Typed Data Guards

Bot-wide data is accessed through `DataReadGuard` / `DataWriteGuard`, which
provide typed accessors:

```rust
// bot_data() -> DataReadGuard<'_>, bot_data_mut() -> DataWriteGuard<'_>
let guard = context.bot_data().await;
let name: Option<&str> = guard.get_str("name");
let age: Option<i64> = guard.get_i64("age");
let active: Option<bool> = guard.get_bool("active");
```

Per-user and per-chat data are plain `HashMap<String, serde_json::Value>`
(optionally absent), not guards:

```rust
// user_data() -> Option<DefaultData>, where DefaultData = HashMap<String, Value>
if let Some(data) = context.user_data().await {
    let name = data.get("name").and_then(|v| v.as_str());
}
```

## Chat Data

Similar to user data, but keyed per chat:

```rust
let chat_data = context.chat_data().await;
```

## Bot Data

Global data shared across all handlers:

```rust
let bot_data = context.bot_data().await;
```

## Job Queue

Access the scheduled job system (requires the `job-queue` feature):

```rust
let jq = context.job_queue.as_ref()
    .expect("job_queue should be set");

jq.once(callback, Duration::from_secs(60))
    .name("reminder")
    .chat_id(chat_id)
    .start()
    .await;
```

See the [Job Queue guide](../guides/job-queue.md) for full details.

## Error Information

In error handlers, the context carries information about what went wrong:

```rust
async fn error_handler(
    update: Option<Arc<Update>>,
    context: CallbackContext,
) -> bool {
    let error_text = context.error
        .as_ref()
        .map(|e| format!("{e}"))
        .unwrap_or_else(|| "Unknown error".to_string());

    tracing::error!("Error: {error_text}");

    let chat_data = context.chat_data().await;
    let user_data = context.user_data().await;

    false // Return false to allow other error handlers to run
}
```

## Handler Arguments

The context also provides parsed arguments for command handlers:

```rust
async fn set_timer(update: Arc<Update>, context: Context) -> HandlerResult {
    // For a command like "/set 30", context.args contains ["30"]
    if let Some(args) = &context.args {
        if let Some(seconds) = args.first().and_then(|s| s.parse::<u64>().ok()) {
            // Use seconds
        }
    }
    Ok(())
}
```

## Summary

| Method | Returns | Purpose |
|---|---|---|
| `bot()` | `Arc<ExtBot>` | Access the Telegram Bot API |
| `reply_text(&update, text)` | `Result<...>` | Quick reply to the current chat |
| `bot_data()` | `DataReadGuard<'_>` | Bot-wide data, with typed accessors |
| `bot_data_mut()` | `DataWriteGuard<'_>` | Bot-wide data, mutable |
| `user_data()` | `Option<DefaultData>` | Per-user key-value storage (`HashMap<String, Value>`) |
| `set_user_data(key, value)` | `bool` | Store a value for the current user |
| `chat_data()` | `Option<DefaultData>` | Per-chat key-value storage |
| `set_chat_data(key, value)` | `bool` | Store a value for the current chat |
| `job_queue` | `Option<Arc<JobQueue>>` | Scheduled task system |
| `error` | `Option<Arc<dyn Error + Send + Sync>>` | Error details (in error handlers) |
| `args` | `Option<Vec<String>>` | Command arguments |

## Next Steps

Understand how all these pieces fit together in the [Application](application.md).
