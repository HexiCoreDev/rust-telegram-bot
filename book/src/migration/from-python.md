# From python-telegram-bot

This guide helps developers migrating from [python-telegram-bot](https://python-telegram-bot.org/) to rust-tg-bot. The architecture is deliberately similar, so most concepts map directly. The key differences are Rust's type system, ownership model, and async runtime.

## Concept Mapping

| python-telegram-bot | rust-tg-bot | Notes |
|---|---|---|
| `Application.builder().token(...).build()` | `ApplicationBuilder::new().token(...).build()` | Typestate pattern in Rust |
| `update: Update` | `update: Arc<Update>` | `Arc` for cheap cloning across tasks |
| `context: ContextTypes.DEFAULT_TYPE` | `context: Context` | `Context` is an alias for `CallbackContext` |
| `context.bot` | `context.bot()` | Method call, not field access |
| `context.user_data` | `context.user_data().await` | Async, returns `Option<HashMap>` |
| `context.bot_data` | `context.bot_data().await` | Returns `DataReadGuard` |
| `CommandHandler("start", start)` | `CommandHandler::new("start", start)` | Typed constructor |
| `MessageHandler(filters.TEXT & ~filters.COMMAND, echo)` | `MessageHandler::new(TEXT() & !COMMAND(), echo)` | `!` not `~`, functions not constants |
| `ConversationHandler` | `FnHandler` with state store | Manual state machine via `Arc<RwLock<HashMap>>` |
| `CallbackQueryHandler(callback)` | `FnHandler::on_callback_query(callback)` | Factory method on `FnHandler` |
| `InlineQueryHandler(handler)` | `FnHandler::on_inline_query(handler)` | Factory method on `FnHandler` |
| `application.run_polling()` | `app.run_polling().await` | Explicit `.await` |
| `application.run_webhook(...)` | `app.run_webhook(config).await` | `WebhookConfig` struct |
| `PicklePersistence` | `JsonFilePersistence` | JSON instead of pickle |
| `from telegram.ext import *` | `use rust_tg_bot::ext::prelude::{specific, items};` | No wildcards |

## Side-by-Side Comparison

### Python Echo Bot

```python
from telegram import Update
from telegram.ext import (
    ApplicationBuilder,
    CommandHandler,
    ContextTypes,
    MessageHandler,
    filters,
)

async def start(update: Update, context: ContextTypes.DEFAULT_TYPE):
    await update.message.reply_text(
        f"Hi {update.effective_user.first_name}!"
    )

async def echo(update: Update, context: ContextTypes.DEFAULT_TYPE):
    await update.message.reply_text(update.message.text)

def main():
    app = ApplicationBuilder().token("TOKEN").build()
    app.add_handler(CommandHandler("start", start))
    app.add_handler(MessageHandler(
        filters.TEXT & ~filters.COMMAND, echo
    ))
    app.run_polling()

if __name__ == "__main__":
    main()
```

### Rust Echo Bot

```rust
use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, Arc, CommandHandler, Context, HandlerResult,
    MessageHandler, Update, COMMAND, TEXT,
};

async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let name = update
        .effective_user()
        .map(|u| u.first_name.as_str())
        .unwrap_or("there");
    context
        .reply_text(&update, &format!("Hi {name}!"))
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
        .expect("TELEGRAM_BOT_TOKEN must be set");

    let app = ApplicationBuilder::new().token(token).build();

    app.add_handler(CommandHandler::new("start", start), 0).await;
    app.add_handler(
        MessageHandler::new(TEXT() & !COMMAND(), echo), 0,
    ).await;

    app.run_polling().await.unwrap();
}
```

## Key Differences

### 1. Arc\<Update\> Instead of Update

In Python, the `Update` is passed by reference and garbage collected. In Rust, the `Update` is wrapped in `Arc<Update>` (atomic reference counting) so it can be cheaply shared across async tasks without copying.

```rust
// Access fields through Arc transparently:
let user = update.effective_user();       // same as Python
let chat = update.effective_chat();       // same as Python
let msg = update.effective_message();     // same as Python
```

### 2. Explicit Imports

Python encourages wildcard imports. Rust benefits from explicit imports for clarity and compile speed:

```python
# Python
from telegram.ext import *
```

```rust
// Rust -- import exactly what you need
use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, Arc, CommandHandler, Context, HandlerResult,
    MessageHandler, Update, COMMAND, TEXT,
};
```

### 3. Typed Constructors Instead of json

Python often uses dicts or keyword arguments. Rust uses typed constructors:

```python
# Python
InlineKeyboardButton("Click", callback_data="1")
LabeledPrice("Item", 100)
```

```rust
// Rust
InlineKeyboardButton::callback("Click", "1")
LabeledPrice::new("Item", 100)
```

### 4. #[tokio::main] and Explicit .await

Python's `application.run_polling()` manages the event loop internally. In Rust, you declare the async runtime explicitly:

```rust
#[tokio::main]
async fn main() {
    // ...
    app.run_polling().await.unwrap();
}
```

Every async operation requires `.await`. There is no implicit awaiting.

### 5. Handler Return Types

Python handlers return `None` implicitly. Rust handlers return `HandlerResult`:

```python
# Python
async def handler(update, context):
    await update.message.reply_text("Hi")
    # implicitly returns None
```

```rust
// Rust
async fn handler(update: Arc<Update>, context: Context) -> HandlerResult {
    context.reply_text(&update, "Hi").await?;
    Ok(())  // explicit return
}
```

The `?` operator propagates errors. If a Telegram API call fails, the error flows to your error handler instead of crashing.

### 6. Handler Groups

Python uses `add_handler(handler, group=0)`. Rust uses `add_handler(handler, group)`:

```python
# Python
app.add_handler(CommandHandler("start", start), group=0)
```

```rust
// Rust
app.add_handler(CommandHandler::new("start", start), 0).await;
```

### 7. Filter Syntax

Python uses `~` for NOT and `&`/`|` for composition. Rust uses `!` for NOT:

```python
# Python
filters.TEXT & ~filters.COMMAND
filters.PHOTO | filters.VIDEO
```

```rust
// Rust
TEXT() & !COMMAND()
F::new(PHOTO) | F::new(VIDEO)
```

Note that `TEXT()` and `COMMAND()` are functions that return `F` wrappers, not bare constants.

### 8. Persistence

Python uses `PicklePersistence`. Rust uses `JsonFilePersistence` or `SqlitePersistence`:

```python
# Python
persistence = PicklePersistence(filepath="bot_data")
app = ApplicationBuilder().token("TOKEN").persistence(persistence).build()
```

```rust
// Rust
use rust_tg_bot::ext::persistence::json_file::JsonFilePersistence;

let persistence = JsonFilePersistence::new("bot_data", true, false);
let app = ApplicationBuilder::new()
    .token(token)
    .persistence(Box::new(persistence))
    .build();
```

Data access is also different:

```python
# Python
context.user_data["key"] = "value"
value = context.user_data.get("key")
```

```rust
// Rust
context.set_user_data("key".to_string(), JsonValue::String("value".into())).await;
let data = context.user_data().await.unwrap_or_default();
let value = data.get("key").and_then(|v| v.as_str());
```

### 9. Conversation Handlers

Python has a dedicated `ConversationHandler` class. Rust models conversations as manual state machines:

```python
# Python
conv_handler = ConversationHandler(
    entry_points=[CommandHandler("start", start)],
    states={
        CHOOSING: [MessageHandler(filters.TEXT, choice)],
        TYPING: [MessageHandler(filters.TEXT, received)],
    },
    fallbacks=[CommandHandler("cancel", cancel)],
)
```

```rust
// Rust -- use FnHandler with state predicates
type ConvStore = Arc<RwLock<HashMap<i64, ConvState>>>;

let cs = Arc::clone(&conv_store);
let cs_check = Arc::clone(&conv_store);
app.add_handler(
    FnHandler::new(
        move |u| is_in_state(u, &cs_check, ConvState::Choosing),
        move |update, ctx| {
            let cs = Arc::clone(&cs);
            async move { choice(update, ctx, cs).await }
        },
    ),
    1,
).await;
```

### 10. Error Handling

Python uses `application.add_error_handler(callback)`. Rust uses a similar pattern but with a different signature:

```python
# Python
async def error_handler(update, context):
    logger.error("Exception: %s", context.error)

app.add_error_handler(error_handler)
```

```rust
// Rust
async fn error_handler(
    update: Option<Arc<Update>>,
    context: CallbackContext,
) -> bool {
    if let Some(ref err) = context.error {
        tracing::error!("Exception: {err}");
    }
    false  // allow other error handlers to run
}

app.add_error_handler(
    Arc::new(|update, ctx| Box::pin(error_handler(update, ctx))),
    true,
).await;
```

### 11. Callback Queries

```python
# Python
app.add_handler(CallbackQueryHandler(button))
```

```rust
// Rust
app.add_handler(FnHandler::on_callback_query(button), 0).await;
```

### 12. Inline Queries

```python
# Python
app.add_handler(InlineQueryHandler(inline_handler))
```

```rust
// Rust
app.add_handler(FnHandler::on_inline_query(inline_handler), 0).await;
```

## Common Patterns

### Sending Messages with Parse Mode

```python
# Python
await context.bot.send_message(
    chat_id, text, parse_mode=ParseMode.HTML
)
```

```rust
// Rust
context.bot()
    .send_message(chat_id, &text)
    .parse_mode(ParseMode::Html)
    .send()
    .await?;
```

### Editing Messages

```python
# Python
await query.edit_message_text("Updated text")
```

```rust
// Rust
context.bot()
    .edit_message_text("Updated text")
    .chat_id(chat_id)
    .message_id(message_id)
    .send()
    .await?;
```

### Answering Callback Queries

```python
# Python
await query.answer(text="Done!", show_alert=True)
```

```rust
// Rust
context.bot()
    .answer_callback_query(&cq.id)
    .text("Done!")
    .show_alert(true)
    .send()
    .await?;
```

## What You Gain

- **Compile-time safety.** Wrong types, missing fields, and invalid filter combinations are caught before you run the bot.
- **Performance.** Async Rust on tokio is significantly faster than Python's asyncio.
- **Memory safety.** No null pointer exceptions, data races, or use-after-free.
- **Single binary deployment.** No virtual environment, no pip install, no Python version management.
- **Predictable resource usage.** No garbage collector pauses.

## Next Steps

- [Your First Bot](../getting-started/first-bot.md) -- build a complete bot from scratch.
- [Core Concepts](../core-concepts/handlers.md) -- deep dive into handlers, filters, and context.
