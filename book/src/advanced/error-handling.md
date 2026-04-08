# Error Handling

Every handler returns `HandlerResult`, which is `Result<(), HandlerError>`. When a handler returns `Err`, the application dispatches the error to any registered error handlers instead of crashing.

## HandlerError

`HandlerError` wraps any error type. The `Other` variant holds a `Box<dyn std::error::Error + Send + Sync>`:

```rust
use telegram_bot::ext::prelude::HandlerError;

// From any std error type
let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
let e = HandlerError::Other(Box::new(io_err));
```

Inside handlers, the `?` operator on Telegram API calls automatically converts errors to `HandlerError`. For other error types, use `.map_err`:

```rust
use telegram_bot::ext::prelude::{Arc, Context, HandlerError, HandlerResult, Update};

async fn handler(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().map(|c| c.id).unwrap();

    // Telegram errors convert directly with ?
    context.reply_text(&update, "Hello!").await?;

    // Other errors need .map_err
    let _data = std::fs::read_to_string("config.toml")
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}
```

## Deliberately Triggering Errors

Sometimes you want to signal an error condition from a handler:

```rust
async fn bad_command(
    _update: Arc<Update>,
    _context: Context,
) -> HandlerResult {
    Err(HandlerError::Other(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "This is a deliberately triggered error from /bad_command!",
    ))))
}
```

## Registering an Error Handler

`app.add_error_handler` registers a callback that runs whenever any handler returns `Err`:

```rust
use telegram_bot::ext::prelude::{
    ApplicationBuilder, Arc, CallbackContext, Context, HandlerError,
    HandlerResult, CommandHandler, Update,
};

async fn error_handler(
    update: Option<Arc<Update>>,
    context: CallbackContext,
) -> bool {
    let error_text = context
        .error
        .as_ref()
        .map(|e| format!("{e}"))
        .unwrap_or_else(|| "Unknown error".to_string());

    tracing::error!("Exception while handling an update: {error_text}");

    // Build diagnostic info
    let update_str = update
        .as_ref()
        .map(|u| format!("{u:?}"))
        .unwrap_or_else(|| "No update".to_string());

    let chat_data_str = context
        .chat_data()
        .await
        .map(|d| format!("{d:?}"))
        .unwrap_or_else(|| "None".to_string());

    let user_data_str = context
        .user_data()
        .await
        .map(|d| format!("{d:?}"))
        .unwrap_or_else(|| "None".to_string());

    // Truncate to respect the 4096-char Telegram limit
    let message = format!(
        "An exception was raised while handling an update\n\n\
         update = {update_str}\n\n\
         chat_data = {chat_data_str}\n\n\
         user_data = {user_data_str}\n\n\
         error = {error_text}"
    );
    let message = if message.len() > 4000 {
        format!("{}...(truncated)", &message[..4000])
    } else {
        message
    };

    let dev_chat_id: i64 = std::env::var("DEVELOPER_CHAT_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    if dev_chat_id != 0 {
        let _ = context
            .bot()
            .send_message(dev_chat_id, &message)
            .send()
            .await;
    }

    // Return false so other error handlers (if any) can also run
    false
}
```

### Error Handler Signature

The error handler signature is:

```rust
async fn error_handler(
    update: Option<Arc<Update>>,
    context: CallbackContext,
) -> bool
```

- `update` -- the update that caused the error, if available. It is `None` for errors that occur outside of update processing.
- `context` -- a `CallbackContext` that provides access to the bot, user/chat data, and the `error` field.
- Returns `bool` -- return `false` to allow other error handlers to run; `true` to stop the chain.

### Registering the Handler

Error handlers must be wrapped in an `Arc` and pinned:

```rust
app.add_error_handler(
    Arc::new(|update, ctx| Box::pin(error_handler(update, ctx))),
    true,  // block: whether to wait for the handler to complete
).await;
```

## Accessing Error Details

The error is available on `context.error`:

```rust
async fn error_handler(
    update: Option<Arc<Update>>,
    context: CallbackContext,
) -> bool {
    if let Some(ref err) = context.error {
        tracing::error!("Handler error: {err}");
    }

    // You can also access data stores for diagnostics
    if let Some(user_data) = context.user_data().await {
        tracing::debug!("User data at time of error: {user_data:?}");
    }

    false
}
```

## Combining with Typed Errors

Define a domain error type with `thiserror` for structured error handling:

```rust
use thiserror::Error;
use telegram_bot::ext::prelude::HandlerError;

#[derive(Debug, Error)]
pub enum BotError {
    #[error("user {user_id} is not authorised for {action}")]
    Unauthorised { user_id: i64, action: String },

    #[error("database error: {0}")]
    Database(String),

    #[error("invalid input: {0}")]
    InvalidInput(String),
}

// Convert to HandlerError
impl From<BotError> for HandlerError {
    fn from(e: BotError) -> Self {
        HandlerError::Other(Box::new(e))
    }
}
```

Use it in handlers:

```rust
async fn restricted(update: Arc<Update>, context: Context) -> HandlerResult {
    let user = update
        .effective_user()
        .ok_or_else(|| BotError::Unauthorised {
            user_id: 0,
            action: "restricted".into(),
        })?;

    if user.id != 123456789 {
        return Err(BotError::Unauthorised {
            user_id: user.id,
            action: "restricted".into(),
        }.into());
    }

    context.reply_text(&update, "Access granted.").await?;
    Ok(())
}
```

## Best Practices

### Always Answer Callback Queries

If your handler for a callback query fails before calling `answer_callback_query`, the user sees a perpetual loading spinner. Use the error handler to clean up:

```rust
async fn error_handler(
    update: Option<Arc<Update>>,
    context: CallbackContext,
) -> bool {
    // If the errored update was a callback query, answer it
    if let Some(ref u) = update {
        if let Some(cq) = u.callback_query() {
            let _ = context
                .bot()
                .answer_callback_query(&cq.id)
                .send()
                .await;
        }
    }

    // Log the error
    if let Some(ref err) = context.error {
        tracing::error!("Handler error: {err}");
    }

    false
}
```

### Avoid unwrap in Handlers

Panics inside handlers are caught by the async runtime, but they produce opaque error messages. Prefer `ok_or` and `?`:

```rust
// Avoid:
let msg = update.effective_message().unwrap();

// Prefer:
let msg = update.effective_message().ok_or_else(|| {
    HandlerError::Other("update has no message".into())
})?;
```

Or simply return early:

```rust
let Some(msg) = update.effective_message() else {
    return Ok(());
};
```

## Complete Example

```rust
use telegram_bot::ext::prelude::{
    ApplicationBuilder, Arc, CallbackContext, CommandHandler, Context,
    HandlerError, HandlerResult, Update,
};

async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().map(|c| c.id).unwrap_or(0);
    context
        .reply_text(
            &update,
            &format!(
                "Use /bad_command to cause an error.\nYour chat id is {}.",
                chat_id,
            ),
        )
        .await?;
    Ok(())
}

async fn bad_command(
    _update: Arc<Update>,
    _context: Context,
) -> HandlerResult {
    Err(HandlerError::Other(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "This is a deliberately triggered error!",
    ))))
}

async fn on_error(
    update: Option<Arc<Update>>,
    context: CallbackContext,
) -> bool {
    let error_text = context
        .error
        .as_ref()
        .map(|e| format!("{e}"))
        .unwrap_or_else(|| "Unknown error".to_string());

    tracing::error!("Error: {error_text}");

    let dev_id: i64 = std::env::var("DEVELOPER_CHAT_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    if dev_id != 0 {
        let msg = format!("Error: {error_text}");
        let _ = context.bot().send_message(dev_id, &msg).send().await;
    }

    false
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let app = ApplicationBuilder::new().token(token).build();

    app.add_typed_handler(CommandHandler::new("start", start), 0).await;
    app.add_typed_handler(CommandHandler::new("bad_command", bad_command), 0).await;

    app.add_error_handler(
        Arc::new(|update, ctx| Box::pin(on_error(update, ctx))),
        true,
    ).await;

    println!("Error handler bot is running. Press Ctrl+C to stop.");

    app.run_polling().await.unwrap();
}
```

## Next Steps

- [Custom Filters](custom-filters.md) -- prevent errors by filtering updates before they reach handlers.
- [Testing](testing.md) -- assert that your error handler is invoked correctly.
