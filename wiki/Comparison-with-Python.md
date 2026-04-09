# Comparison with python-telegram-bot

This page provides a detailed side-by-side comparison for developers migrating from [python-telegram-bot](https://github.com/python-telegram-bot/python-telegram-bot) (PTB) to `rust-tg-bot`. The Rust framework deliberately mirrors PTB's architecture, so the transition is straightforward.

---

## Architecture Mapping

| PTB Concept | Rust Equivalent | Notes |
|------------|-----------------|-------|
| `Application` | `Application` | Nearly identical API |
| `ApplicationBuilder` | `ApplicationBuilder` | Typestate pattern (token required at compile time) |
| `CallbackContext` | `Context` (alias for `CallbackContext`) | Same fields: `bot`, `args`, `matches`, `error`, `job_queue` |
| `Update` | `Update` | Typed struct instead of JSON dict |
| `CommandHandler` | `CommandHandler` | Same constructor: `CommandHandler::new("start", fn)` |
| `MessageHandler` | `MessageHandler` | Same pattern: `MessageHandler::new(filter, fn)` |
| `CallbackQueryHandler` | `FnHandler::on_callback_query(fn)` | Convenience constructor |
| `InlineQueryHandler` | `FnHandler::on_inline_query(fn)` | Convenience constructor |
| `ConversationHandler` | `ConversationHandler` | Same state machine model |
| `JobQueue` | `JobQueue` | Same API: `once`, `repeating`, cancellation |
| `PicklePersistence` | `JsonFilePersistence` | JSON instead of pickle (human-readable) |
| `BasePersistence` | `BasePersistence` trait | Same method signatures |

---

## Side-by-Side Code Examples

### Hello World

**Python:**

```python
from telegram import Update
from telegram.ext import Application, CommandHandler, ContextTypes

async def start(update: Update, context: ContextTypes.DefaultType):
    await update.message.reply_text("Hello!")

app = Application.builder().token("TOKEN").build()
app.add_handler(CommandHandler("start", start))
app.run_polling()
```

**Rust:**

```rust
use rust_tg_bot::ext::prelude::*;

async fn start(update: Update, context: Context) -> HandlerResult {
    context.reply_text(&update, "Hello!").await?;
    Ok(())
}

fn main() {
    rust_tg_bot::run(async {
        let app = ApplicationBuilder::new().token("TOKEN").build();
        app.add_handler(CommandHandler::new("start", start), 0).await;
        app.run_polling().await.unwrap();
    });
}
```

**Key differences:**
- Rust uses `context.reply_text(&update, text)` instead of `update.message.reply_text(text)`.
- Handlers return `HandlerResult` (a `Result` type) instead of `None`.
- `add_handler` takes a group number as the second argument.
- No `async def` -- Rust uses `async fn`.

---

### Echo Bot with Filters

**Python:**

```python
from telegram.ext import MessageHandler, filters

async def echo(update: Update, context: ContextTypes.DefaultType):
    await update.message.reply_text(update.message.text)

app.add_handler(MessageHandler(filters.TEXT & ~filters.COMMAND, echo))
```

**Rust:**

```rust
async fn echo(update: Update, context: Context) -> HandlerResult {
    let text = update.effective_message().and_then(|m| m.text.as_deref()).unwrap_or("");
    if !text.is_empty() {
        context.reply_text(&update, text).await?;
    }
    Ok(())
}

app.add_handler(MessageHandler::new(TEXT() & !COMMAND(), echo), 0).await;
```

**Key differences:**
- Python's `filters.TEXT` becomes `TEXT()` (a function call that returns an `F` wrapper).
- Python's `~` (bitwise NOT) becomes `!` in Rust.
- Python's `filters.COMMAND` becomes `COMMAND()`.
- Text extraction is explicit in Rust: `update.effective_message().and_then(|m| m.text.as_deref())`.

---

### Inline Keyboard

**Python:**

```python
from telegram import InlineKeyboardButton, InlineKeyboardMarkup

keyboard = [[InlineKeyboardButton("Click", callback_data="1")]]
reply_markup = InlineKeyboardMarkup(keyboard)
await update.message.reply_text("Choose:", reply_markup=reply_markup)
```

**Rust:**

```rust
use serde_json::json;

let keyboard = json!({
    "inline_keyboard": [[{"text": "Click", "callback_data": "1"}]]
});

context
    .bot()
    .send_message(chat_id, "Choose:")
    .reply_markup(keyboard)
    .send()
    .await?;
```

**Key differences:**
- Rust uses `serde_json::json!` macro to build keyboard JSON instead of typed `InlineKeyboardButton` structs.
- The builder pattern: chain `.reply_markup(keyboard).send().await?`.

---

### Callback Query Handler

**Python:**

```python
from telegram.ext import CallbackQueryHandler

async def button(update: Update, context: ContextTypes.DefaultType):
    query = update.callback_query
    await query.answer()
    await query.edit_message_text(f"Selected: {query.data}")

app.add_handler(CallbackQueryHandler(button))
```

**Rust:**

```rust
async fn button(update: Update, context: Context) -> HandlerResult {
    let cq = update.callback_query.as_ref().unwrap();
    let data = cq.data.as_deref().unwrap_or("unknown");

    context.bot().answer_callback_query(&cq.id).send().await?;

    if let Some(ref msg) = cq.message {
        context
            .bot()
            .edit_message_text(&format!("Selected: {data}"))
            .chat_id(msg.chat().id)
            .message_id(msg.message_id())
            .send()
            .await?;
    }
    Ok(())
}

app.add_handler(FnHandler::on_callback_query(button), 0).await;
```

**Key differences:**
- `CallbackQueryHandler` becomes `FnHandler::on_callback_query(fn)`.
- Answering and editing are separate API calls with explicit chat_id and message_id.
- No `query.answer()` shortcut -- use `bot.answer_callback_query(&id)`.

---

### Command Arguments

**Python:**

```python
async def set_timer(update, context):
    seconds = int(context.args[0])
```

**Rust:**

```rust
async fn set_timer(update: Update, context: Context) -> HandlerResult {
    let seconds: u64 = context.args.as_ref()
        .and_then(|a| a.first())
        .and_then(|s| s.parse().ok())
        .unwrap_or(60);
    // ...
    Ok(())
}
```

**Key differences:**
- `context.args` is `Option<Vec<String>>` in Rust (always check for `None`).
- Parsing is explicit: `.parse::<u64>()` instead of `int()`.

---

### Bot Data

**Python:**

```python
context.bot_data["key"] = "value"
value = context.bot_data.get("key")
```

**Rust:**

```rust
// Write
{
    let mut guard = context.bot_data_mut().await;
    guard.set_str("key", "value");
}

// Read
{
    let guard = context.bot_data().await;
    let value = guard.get_str("key");
}
```

**Key differences:**
- Rust uses async locks: `bot_data().await` returns a `DataReadGuard`, `bot_data_mut().await` returns a `DataWriteGuard`.
- Typed setters (`set_str`, `set_i64`, `set_bool`) instead of Python's dynamic dict.
- The guard must be dropped (goes out of scope) before acquiring another lock.

---

### Error Handler

**Python:**

```python
async def error_handler(update, context):
    logging.error(f"Error: {context.error}")

app.add_error_handler(error_handler)
```

**Rust:**

```rust
async fn error_handler(update: Option<Update>, context: CallbackContext) -> bool {
    let err = context.error.as_ref().map(|e| format!("{e}")).unwrap_or_default();
    tracing::error!("Error: {err}");
    false  // false = let other error handlers run too
}

app.add_error_handler(
    Arc::new(|update, ctx| Box::pin(error_handler(update, ctx))),
    true,
).await;
```

**Key differences:**
- Error handler returns `bool` (true = stop, false = continue to next handler).
- Registration requires wrapping in `Arc::new` with a pinned future.
- `update` is `Option<Update>` because errors can occur without an associated update.

---

### Persistence

**Python:**

```python
persistence = PicklePersistence(filepath="bot_data")
app = Application.builder().token("TOKEN").persistence(persistence).build()
```

**Rust:**

```rust
use rust_tg_bot::ext::persistence::json_file::JsonFilePersistence;

let persistence = JsonFilePersistence::new("bot_data", true, false);
let app = ApplicationBuilder::new()
    .token("TOKEN")
    .persistence(Box::new(persistence))
    .build();
```

**Key differences:**
- JSON format instead of pickle (human-readable, portable).
- `Box::new(persistence)` for dynamic dispatch.
- Same conceptual API: user_data, chat_data, bot_data all persist automatically.

---

## Concept Mapping Table

| Python | Rust | Notes |
|--------|------|-------|
| `update.message.reply_text("hi")` | `context.reply_text(&update, "hi").await?` | Context holds the bot reference |
| `update.message.text` | `update.effective_message().and_then(\|m\| m.text.as_deref())` | Option chaining |
| `update.effective_user.id` | `update.effective_user().map(\|u\| u.id)` | Returns Option |
| `context.bot.send_message(...)` | `context.bot().send_message(...).send().await?` | Builder + send |
| `filters.TEXT & ~filters.COMMAND` | `TEXT() & !COMMAND()` | Same operators, different syntax |
| `handler_group` param in `add_handler` | Second arg to `add_handler` | `0` is default group |
| `context.args[0]` | `context.args.as_ref().and_then(\|a\| a.first())` | Safe access |
| `context.bot_data["k"]` | `context.bot_data().await.get_str("k")` | Async lock + typed accessor |
| `context.job_queue` | `context.job_queue.as_ref()` | Optional, configured at build time |
| `ConversationHandler.END` | `ConversationResult::End` | Enum variant |
| `await` | `.await?` | Rust propagates errors with `?` |

---

## Performance Comparison

| Metric | Python (PTB) | Rust |
|--------|-------------|------|
| Startup memory | ~50 MB | ~15 MB |
| Per-update overhead | ~1 ms | ~10 us |
| Binary size | N/A (interpreted) | ~10 MB (stripped) |
| Cold start | 2-5 s | < 500 ms |
| Concurrency model | asyncio (single-threaded) | tokio (multi-threaded) |
| Type safety | Runtime errors | Compile-time errors |

The Rust version uses approximately 3-4x less memory and processes updates 10-100x faster, depending on the workload.

---

## Migration Checklist

- [ ] Replace `Application.builder()` with `ApplicationBuilder::new()`
- [ ] Change handler signatures to `async fn(Update, Context) -> HandlerResult`
- [ ] Replace `update.message.reply_text()` with `context.reply_text(&update, text).await?`
- [ ] Replace `filters.TEXT` with `TEXT()`, `filters.COMMAND` with `COMMAND()`
- [ ] Replace `~` with `!`, `&` stays `&`, `|` stays `|`
- [ ] Replace `add_handler(h)` with `add_handler(h, group).await`
- [ ] Replace `CallbackQueryHandler` with `FnHandler::on_callback_query`
- [ ] Replace `InlineQueryHandler` with `FnHandler::on_inline_query`
- [ ] Replace `context.args[0]` with `context.args.as_ref().and_then(|a| a.first())`
- [ ] Replace `context.bot_data["k"]` with `context.bot_data().await.get_str("k")`
- [ ] Replace `PicklePersistence` with `JsonFilePersistence` or `SqlitePersistence`
- [ ] Wrap main in `rust_tg_bot::run(async { ... })`
- [ ] Add `?` after every `.await` that can fail
