# Context

The `Context` (aliased as `CallbackContext`) is passed to every handler alongside the `Update`. It provides access to the bot, user/chat data storage, the job queue, and convenience methods.

## Core Methods

### bot()

Access the bot instance to call Telegram API methods:

```rust
async fn my_handler(update: Arc<Update>, context: Context) -> HandlerResult {
    let bot = context.bot();
    bot.send_message(chat_id, "Hello!").send().await?;
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
context.bot().send_message(chat_id, "Got it!").send().await?;
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

The data methods return `DataReadGuard` and `DataWriteGuard` types that provide typed accessors:

```rust
if let Some(guard) = context.user_data_guard().await {
    let name: Option<&str> = guard.get_str("name");
    let age: Option<i64> = guard.get_i64("age");
    let score: Option<f64> = guard.get_f64("score");
    let active: Option<bool> = guard.get_bool("active");
    let ids: HashSet<i64> = guard.get_id_set("friend_ids");
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
| `user_data()` | `Option<HashMap<String, JsonValue>>` | Per-user key-value storage |
| `set_user_data(key, value)` | `()` | Store a value for the current user |
| `chat_data()` | `Option<String>` | Per-chat data (debug representation) |
| `job_queue` | `Option<Arc<JobQueue>>` | Scheduled task system |
| `error` | `Option<HandlerError>` | Error details (in error handlers) |
| `args` | `Option<Vec<String>>` | Command arguments |

## Next Steps

Understand how all these pieces fit together in the [Application](application.md).
