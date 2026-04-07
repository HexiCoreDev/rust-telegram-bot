# Migration from python-telegram-bot

This guide is for developers who have built bots with python-telegram-bot v20+ and want
to understand the Rust equivalent. The libraries share the same architecture and
vocabulary, so most concepts map directly.

---

## Quick Concept Mapping

| Python concept | Rust equivalent |
|----------------|-----------------|
| `telegram` package | `telegram-bot-raw` crate |
| `telegram.ext` package | `telegram-bot-ext` crate |
| `Application` class | `Arc<Application>` struct |
| `ApplicationBuilder` | `ApplicationBuilder<State>` (typestate) |
| `BaseHandler` | `Handler` trait |
| `handler.check_update(update)` | `handler.check_update(&update) -> Option<MatchResult>` |
| `CallbackContext` dataclass | `CallbackContext` struct |
| `context.bot` | `context.bot()` (returns `Arc<ExtBot>`) |
| `context.user_data` | `context.user_data()` (returns `Arc<RwLock<HashMap<...>>>`) |
| `context.chat_data` | `context.chat_data()` |
| `context.bot_data` | `context.bot_data()` |
| `context.args` | `context.args` (Option<Vec<String>>, set by CommandHandler) |
| `context.matches` | `context.matches` (Option<Vec<String>>, set by regex handlers) |
| `context.job_queue` | `context.job_queue()` |
| `message.reply_text(...)` | `context.bot().inner().build_send_message(chat_id, text).send().await` |
| `filters.TEXT` | `TEXT` (from `filters::text`) |
| `filters.ALL` | `ALL` (from `filters::base`) |
| `f1 & f2` | `F::new(f1) & F::new(f2)` |
| `~f` | `!F::new(f)` |
| `PicklePersistence` | `JsonFilePersistence` or `SqlitePersistence` |
| `APScheduler` jobs | `JobQueue` (tokio timers) |
| `ConversationHandler.END` | `ConversationResult::End` |
| `ConversationHandler.WAITING` | implicit WAITING state (non-blocking callbacks) |

---

## Key Differences

### No bot methods on types

In Python:

```python
async def handle(update, context):
    await update.message.reply_text("Hello!")
    await update.effective_user.send_message("Direct DM")
```

In Rust, `Message`, `User`, and `Chat` are plain data structs with no bot reference.
You always call methods through the context using the builder API:

```rust
async fn handle(update: Value, context: CallbackContext) -> Result<(), HandlerError> {
    let chat_id = update["message"]["chat"]["id"].as_i64().unwrap();
    context
        .bot()
        .inner()
        .build_send_message(chat_id.into(), "Hello!")
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;
    Ok(())
}
```

This is more verbose but keeps type definitions clean and avoids circular references.

### FilterResult instead of bool

Python filters return `True | False | DataDict`. Rust uses an enum:

```rust
pub enum FilterResult {
    NoMatch,
    Match,
    MatchWithData(HashMap<String, Vec<String>>),
}
```

Check if a filter matched with `.is_match()`. For data filters (regex, caption-regex),
the extracted data is in the `MatchWithData` variant.

### Filter wrapping with F

Python filters compose directly:

```python
text_not_command = filters.TEXT & ~filters.COMMAND
```

In Rust, wrap in `F` first:

```rust
use telegram_bot_ext::filters::base::{F, Filter};
use telegram_bot_ext::filters::text::TEXT;
use telegram_bot_ext::filters::command::COMMAND;

let text_not_command = F::new(TEXT) & !F::new(COMMAND);
```

After wrapping, the operator syntax is the same (except `~` becomes `!`).

### Async is explicit

Python async functions are called with `await`:

```python
async def start(update, context):
    await context.bot.send_message(...)
```

Rust async functions are the same, but the function signature and error type are
explicit:

```rust
async fn start(update: Value, context: CallbackContext) -> Result<(), HandlerError> {
    context.bot().inner()
        .build_send_message(chat_id.into(), "Hello!")
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;
    Ok(())
}
```

### Handler registration returns a future

In Python:

```python
app.add_handler(CommandHandler("start", start))
```

In Rust, `add_handler` is async (it acquires a write lock on the handler list):

```rust
app.add_handler(handler, group).await;
```

Call it in an async context (inside `main` or another async function).

### Errors must be boxed

Python handlers can raise any exception. In Rust, the error type in `HandlerResult` is:

```rust
Error(Box<dyn std::error::Error + Send + Sync>)
```

And handler callbacks return:

```rust
type HandlerCallback = Arc<dyn Fn(Value, CallbackContext) -> Pin<Box<dyn Future<Output = Result<(), HandlerError>> + Send>> + Send + Sync>;
```

Where `HandlerError::Other` wraps any error:

```rust
.map_err(|e| HandlerError::Other(Box::new(e)))?
```

---

## Common Patterns Translated

### Echo bot

Python:
```python
async def echo(update, context):
    await update.message.reply_text(update.message.text)

app.add_handler(MessageHandler(filters.TEXT & ~filters.COMMAND, echo))
```

Rust:
```rust
async fn echo(update: Value, context: CallbackContext) -> Result<(), HandlerError> {
    let chat_id = update["message"]["chat"]["id"].as_i64().unwrap();
    let text = update["message"]["text"].as_str().unwrap_or("");
    context.bot().inner()
        .build_send_message(chat_id.into(), text)
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;
    Ok(())
}

let text_not_cmd = F::new(TEXT) & !F::new(COMMAND);

app.add_handler(MessageHandler::new(
    Some(text_not_cmd),
    Arc::new(|u, ctx| Box::pin(echo(u, ctx))),
    true,
), 0).await;
```

### Command with arguments

Python:
```python
async def ban(update, context):
    if not context.args:
        await update.message.reply_text("Usage: /ban <username>")
        return
    target = context.args[0]
    ...

app.add_handler(CommandHandler("ban", ban))
```

Rust:
```rust
async fn ban(update: Value, context: CallbackContext) -> Result<(), HandlerError> {
    let chat_id = update["message"]["chat"]["id"].as_i64().unwrap();
    match context.args.as_deref() {
        None | Some([]) => {
            context.bot().inner()
                .build_send_message(chat_id.into(), "Usage: /ban <username>")
                .send()
                .await
                .map_err(|e| HandlerError::Other(Box::new(e)))?;
        }
        Some([target, ..]) => {
            println!("Banning: {}", target);
        }
    }
    Ok(())
}

app.add_handler(CommandHandler::new(
    vec!["ban".into()],
    Arc::new(|u, ctx| Box::pin(ban(u, ctx))),
    None,
    true,
), 0).await;
```

### Inline keyboard with callback query

Python:
```python
async def button(update, context):
    query = update.callback_query
    await query.answer()
    await query.edit_message_text(f"Selected: {query.data}")
```

Rust:
```rust
async fn button(update: Value, context: CallbackContext) -> Result<(), HandlerError> {
    let cq_id = update["callback_query"]["id"].as_str().unwrap();
    let data = update["callback_query"]["data"].as_str().unwrap_or("?");
    let message_id = update["callback_query"]["message"]["message_id"].as_i64().unwrap();
    let chat_id = update["callback_query"]["message"]["chat"]["id"].as_i64().unwrap();

    context.bot().inner()
        .build_answer_callback_query(cq_id)
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    context.bot().inner()
        .build_edit_message_text(chat_id.into(), message_id, &format!("Selected: {}", data))
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

app.add_handler(CallbackQueryHandler::new(
    Arc::new(|u, mr| Box::pin(button(u, mr))),
    None,
    true,
), 0).await;
```

### Storing user data

Python:
```python
async def count(update, context):
    context.user_data["count"] = context.user_data.get("count", 0) + 1
    await update.message.reply_text(f"You sent {context.user_data['count']} messages.")
```

Rust:
```rust
async fn count(update: Value, context: CallbackContext) -> Result<(), HandlerError> {
    let user_id = update["message"]["from"]["id"].as_i64().unwrap();
    let chat_id = update["message"]["chat"]["id"].as_i64().unwrap();

    let current = {
        let user_data = context.user_data().read().await;
        user_data.get(&user_id)
            .and_then(|d| d.get("count"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0)
    };

    {
        let mut user_data = context.user_data().write().await;
        user_data.entry(user_id).or_default()
            .insert("count".into(), serde_json::json!(current + 1));
    }

    context.bot().inner()
        .build_send_message(chat_id.into(), &format!("You sent {} messages.", current + 1))
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;
    Ok(())
}
```

### Scheduling a reminder

Python:
```python
async def remind(context):
    await context.bot.send_message(context.job.chat_id, "Reminder!")

async def set_timer(update, context):
    context.job_queue.run_once(remind, 30, chat_id=update.effective_chat.id)
```

Rust:
```rust
async fn set_timer(update: Value, context: CallbackContext) -> Result<(), HandlerError> {
    let chat_id = update["message"]["chat"]["id"].as_i64().unwrap();

    let jq = context.job_queue()
        .ok_or_else(|| HandlerError::Other("no job queue configured".into()))?;

    let callback: JobCallbackFn = Arc::new(|ctx| Box::pin(async move {
        // In a real bot, you'd hold a reference to the bot here
        println!("Reminder for chat_id {}", ctx.chat_id.unwrap_or(0));
    }));

    jq.once(callback, Duration::from_secs(30))
        .name("reminder")
        .chat_id(chat_id)
        .start()
        .await;

    Ok(())
}
```

### ConversationHandler

Python:
```python
ASKING_NAME, ASKING_AGE = range(2)

async def start(update, context):
    await update.message.reply_text("What's your name?")
    return ASKING_NAME

async def got_name(update, context):
    context.user_data["name"] = update.message.text
    await update.message.reply_text("How old are you?")
    return ASKING_AGE

async def got_age(update, context):
    await update.message.reply_text(f"Nice to meet you, {context.user_data['name']}!")
    return ConversationHandler.END

conv_handler = ConversationHandler(
    entry_points=[CommandHandler("start", start)],
    states={
        ASKING_NAME: [MessageHandler(filters.TEXT, got_name)],
        ASKING_AGE:  [MessageHandler(filters.TEXT, got_age)],
    },
    fallbacks=[],
)
```

Rust:
```rust
#[derive(Clone, Hash, Eq, PartialEq)]
enum State { AskingName, AskingAge }

// start: called by /start command, transitions to AskingName
// got_name: called when in AskingName state, transitions to AskingAge
// got_age: called when in AskingAge state, ends the conversation

// Each handler is wrapped in ConversationStepHandler:
//   handler: the underlying Handler (for check_update)
//   conv_callback: returns (HandlerResult, ConversationResult<State>)

let conv = ConversationHandler::builder()
    .entry_point(ConversationStepHandler {
        handler: Box::new(CommandHandler::new(vec!["start".into()], noop_cb, None, true)),
        conv_callback: Arc::new(|_u, _mr| Box::pin(async {
            (HandlerResult::Continue, ConversationResult::NextState(State::AskingName))
        })),
    })
    .state(State::AskingName, vec![ConversationStepHandler {
        handler: Box::new(MessageHandler::new(Some(F::new(TEXT)), noop_cb, true)),
        conv_callback: Arc::new(|_u, _mr| Box::pin(async {
            (HandlerResult::Continue, ConversationResult::NextState(State::AskingAge))
        })),
    }])
    .state(State::AskingAge, vec![ConversationStepHandler {
        handler: Box::new(MessageHandler::new(Some(F::new(TEXT)), noop_cb, true)),
        conv_callback: Arc::new(|_u, _mr| Box::pin(async {
            (HandlerResult::Continue, ConversationResult::End)
        })),
    }])
    .build();
```

---

## What Is the Same

- Handler group numbers and dispatch order (ascending, first match wins)
- `ConversationHandler` state machine: entry points, states, fallbacks, per_chat/per_user keys
- Filter operators: `&` (AND), `|` (OR), `^` (XOR), `!` (NOT, was `~`)
- Persistence categories: user_data, chat_data, bot_data, callback_data
- Job queue API: `once`, `repeating`, `daily`, `monthly` (builder pattern)
- `context.args` for command arguments
- `context.matches` for regex captures
- Handler `block` flag for concurrent execution

---

## What Is Different

- **Bot on types**: Python -- `message.reply_text(...)`. Rust -- `context.bot().inner().build_send_message(chat_id, text).send().await`.
- **Filter wrapping**: Python -- compose directly. Rust -- wrap in `F(...)` first.
- **FilterResult vs bool**: Python returns `bool | DataDict`. Rust returns an enum variant.
- **Error handling**: Python raises exceptions. Rust returns `Result<(), HandlerError>`.
- **Persistence backend**: Python has `PicklePersistence`. Rust provides JSON file and SQLite.
- **Job context**: Python job callbacks receive `ContextTypes`. Rust callbacks receive `JobContext` (a lightweight struct with name, chat_id, user_id, data).
- **ConversationResult**: Python callbacks return an integer state or `END`. Rust callbacks return `ConversationResult<S>` with your state type.
- **check_update return**: Python returns `bool`. Rust returns `Option<MatchResult>` so handlers can pass extracted data to callbacks without recomputing.

---

## Tips for Python Developers New to Rust

**Ownership means no shared mutable state by default.** Use `Arc<RwLock<T>>` or
`Arc<Mutex<T>>` for data shared across handler closures and job callbacks. The
`context.user_data()` method returns an `Arc<RwLock<...>>` you can clone.

**Closures must own their captures** when passed across async tasks. If a callback
captures a variable, you may need `move` and `Arc::clone`:

```rust
let chat_id = update["message"]["chat"]["id"].as_i64().unwrap();
let callback: JobCallbackFn = Arc::new(move |_ctx| Box::pin(async move {
    println!("chat_id = {}", chat_id); // chat_id moved into closure
}));
```

**`?` does not work in closures that return `()`.** Job callbacks return `()`, so use
`if let Err(e) = ... { ... }` or `.unwrap_or_else(|e| ...)` for error handling inside
them.

**Trait objects require `Send + Sync`.** All callbacks, filters, and persistence backends
must be `Send + Sync` because they are used in async tasks. If you store non-`Send` data,
wrap it in `Arc<Mutex<T>>`.

**`async fn` in traits requires Rust 1.75+.** The `BasePersistence` trait uses native
`async fn`. Make sure your toolchain is up to date.
