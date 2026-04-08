# Your First Bot

This guide walks through building a complete echo bot step by step. By the end, you will understand the fundamental pattern used by every bot built with this framework.

## The Echo Bot

An echo bot does one thing: it repeats back whatever text the user sends. Despite its simplicity, it demonstrates three core concepts:

1. **Command handlers** -- respond to `/start` and `/help`.
2. **Message handlers with filters** -- catch text messages that are not commands.
3. **The Application lifecycle** -- build, register handlers, and run.

## Full Source

```rust
use telegram_bot::ext::prelude::{
    ApplicationBuilder, Arc, CommandHandler, Context, HandlerResult,
    MessageHandler, Update, COMMAND, TEXT,
};

async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let name = update
        .effective_user()
        .map(|u| u.first_name.as_str())
        .unwrap_or("there");
    context
        .reply_text(
            &update,
            &format!(
                "Hi {name}! I am an echo bot. Send me any message and \
                 I will repeat it back to you.\n\nUse /help to see \
                 available commands."
            ),
        )
        .await?;
    Ok(())
}

async fn help(update: Arc<Update>, context: Context) -> HandlerResult {
    context
        .reply_text(
            &update,
            "Available commands:\n\
             /start - Start the bot\n\
             /help - Show this help message\n\n\
             Send any text message and I will echo it back!",
        )
        .await?;
    Ok(())
}

async fn echo(update: Arc<Update>, context: Context) -> HandlerResult {
    let text = update
        .effective_message()
        .and_then(|m| m.text.as_deref())
        .unwrap_or("");
    if !text.is_empty() {
        context.reply_text(&update, text).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

    let app = ApplicationBuilder::new().token(token).build();

    app.add_typed_handler(CommandHandler::new("start", start), 0).await;
    app.add_typed_handler(CommandHandler::new("help", help), 0).await;
    app.add_typed_handler(
        MessageHandler::new(TEXT() & !COMMAND(), echo), 0,
    ).await;

    println!("Echo bot is running. Press Ctrl+C to stop.");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
```

## Line-by-Line Walkthrough

### Imports

```rust
use telegram_bot::ext::prelude::{
    ApplicationBuilder, Arc, CommandHandler, Context, HandlerResult,
    MessageHandler, Update, COMMAND, TEXT,
};
```

The `prelude` module re-exports everything you need for common bot development. Always import specific items -- avoid wildcard imports for clarity and faster compilation.

### Handler Signature

Every handler function follows this signature:

```rust
async fn handler_name(update: Arc<Update>, context: Context) -> HandlerResult {
    // ...
    Ok(())
}
```

- `update: Arc<Update>` -- the incoming Telegram update, wrapped in `Arc` for cheap cloning across async tasks.
- `context: Context` -- provides access to the bot instance, user/chat data, job queue, and convenience methods like `reply_text`.
- `HandlerResult` -- an alias for `Result<(), HandlerError>`.

### Extracting Data from Updates

```rust
let name = update
    .effective_user()
    .map(|u| u.first_name.as_str())
    .unwrap_or("there");
```

The `Update` type provides typed accessor methods:
- `effective_user()` -- the user who triggered the update.
- `effective_chat()` -- the chat the update originated from.
- `effective_message()` -- the message associated with the update.
- `callback_query()` -- the callback query, if any.

These return `Option` types, so you use standard Rust patterns to handle the `None` case.

### Replying to Messages

```rust
context.reply_text(&update, "Hello!").await?;
```

`context.reply_text()` is a convenience method that:
1. Extracts the chat ID from the update.
2. Calls `bot.send_message(chat_id, text)`.
3. Returns a `Result` you can propagate with `?`.

For more control, use the bot directly:

```rust
context.bot().send_message(chat_id, "Hello!")
    .parse_mode(ParseMode::Html)
    .send()
    .await?;
```

### Building the Application

```rust
let app = ApplicationBuilder::new().token(token).build();
```

`ApplicationBuilder` uses a typestate pattern: you cannot call `.build()` until you have called `.token()`. This is enforced at compile time.

### Registering Handlers

```rust
app.add_typed_handler(CommandHandler::new("start", start), 0).await;
app.add_typed_handler(MessageHandler::new(TEXT() & !COMMAND(), echo), 0).await;
```

- `CommandHandler::new("start", start)` -- matches `/start` and calls the `start` function.
- `MessageHandler::new(TEXT() & !COMMAND(), echo)` -- matches text messages that are NOT commands, then calls `echo`.
- The second argument (`0`) is the handler group. Groups are processed in ascending order. Within a group, the first matching handler wins.

### Running

```rust
app.run_polling().await.unwrap();
```

`run_polling()` starts the bot in long-polling mode: it repeatedly calls Telegram's `getUpdates` endpoint and dispatches incoming updates to your handlers.

## Next Steps

You now understand the fundamental pattern. Every bot you build follows the same structure:

1. Define async handler functions.
2. Build an `Application` with `ApplicationBuilder`.
3. Register handlers.
4. Call `run_polling()` or `run_webhook()`.

Continue to [Running Your Bot](running.md) to learn about environment configuration and the different ways to receive updates.
