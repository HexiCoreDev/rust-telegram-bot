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
| `from telegram.ext import *` | `use telegram_bot::ext::prelude::*` |
| `Application` class | `Arc<Application>` struct |
| `ApplicationBuilder` | `ApplicationBuilder<State>` (typestate) |
| `BaseHandler` | `Handler` trait |
| `handler.check_update(update)` | `handler.check_update(&update) -> Option<MatchResult>` |
| `CallbackContext` dataclass | `Context` (alias for `CallbackContext`) |
| `context.bot` | `context.bot()` (returns `&Arc<ExtBot>`, which `Deref`s to `Bot`) |
| `context.bot_data` | `context.bot_data().await` / `context.bot_data_mut().await` (typed guards) |
| `context.user_data` | `context.user_data().await` (returns cloned snapshot) |
| `context.chat_data` | `context.chat_data().await` (returns cloned snapshot) |
| `context.args` | `context.args` (`Option<Vec<String>>`, set by `CommandHandler`) |
| `context.matches` | `context.matches` (`Option<Vec<String>>`, set by regex handlers) |
| `context.job_queue` | `context.job_queue` (`Option<Arc<JobQueue>>`) |
| `message.reply_text(...)` | `context.reply_text(&update, text).await?` |
| `bot.send_message(chat_id, text)` | `context.bot().send_message(chat_id, text).send().await?` |
| `filters.TEXT` | `TEXT()` (from prelude) |
| `filters.ALL` | `F::new(ALL)` |
| `f1 & f2` | `f1 & f2` (both `F` values) |
| `~f` | `!f` |
| `ParseMode.HTML` | `ParseMode::Html` |
| `MessageEntity.BOT_COMMAND` | `MessageEntityType::BotCommand` |
| `Chat.PRIVATE` | `ChatType::Private` |
| `PicklePersistence` | `JsonFilePersistence` or `SqlitePersistence` |
| `APScheduler` jobs | `JobQueue` (tokio timers, builder pattern) |
| `ConversationHandler.END` | `ConversationResult::End` |

---

## Key Differences

### Typed handler registration

In Python:

```python
app.add_handler(CommandHandler("start", start))
app.add_handler(MessageHandler(filters.TEXT & ~filters.COMMAND, echo))
app.add_handler(CallbackQueryHandler(button))
```

In Rust:

```rust
use telegram_bot::ext::prelude::*;

app.add_typed_handler(CommandHandler::new("start", start), 0).await;
app.add_typed_handler(MessageHandler::new(TEXT() & !COMMAND(), echo), 0).await;
app.add_typed_handler(FnHandler::on_callback_query(button), 0).await;
```

The handler constructors are ergonomic -- `CommandHandler::new` takes the command name
and callback directly. `FnHandler` provides convenience methods like `on_callback_query`,
`on_inline_query`, `on_any`, etc.

### Handler callback signatures

In Python:

```python
async def start(update: Update, context: ContextTypes.DEFAULT_TYPE):
    await update.message.reply_text("Hello!")
```

In Rust:

```rust
async fn start(update: Update, context: Context) -> HandlerResult {
    context.reply_text(&update, "Hello!").await?;
    Ok(())
}
```

The return type is explicit: `HandlerResult` is `Result<(), HandlerError>`. Use `?` for
error propagation.

### Bot API calls use builders

In Python:

```python
await context.bot.send_message(chat_id=chat_id, text="Hello!", parse_mode=ParseMode.HTML)
```

In Rust, every API method returns a builder. Optional parameters are chainable setters.
Builders implement `IntoFuture`, so you can `.await` directly or call `.send().await`:

```rust
// Simple (IntoFuture)
context.bot().send_message(chat_id, "Hello!").await?;

// With optional parameters
context
    .bot()
    .send_message(chat_id, "Hello!")
    .parse_mode(ParseMode::Html)
    .send()
    .await?;
```

No more `None, None, None` for unused parameters.

### Typed constants instead of strings

In Python:

```python
parse_mode=ParseMode.HTML
entity.type == MessageEntity.BOT_COMMAND
chat.type != Chat.PRIVATE
```

In Rust:

```rust
.parse_mode(ParseMode::Html)
entity.entity_type == MessageEntityType::BotCommand
chat.chat_type != ChatType::Private
```

### Typed data access

In Python:

```python
context.bot_data["key"] = "value"
count = context.bot_data.get("count", 0)
```

In Rust, the context provides typed read/write guards with convenience methods:

```rust
// Write
let mut bd = context.bot_data_mut().await;
bd.set_str("key", "value");
bd.set_i64("count", 42);
bd.add_to_id_set("user_ids", user_id);

// Read
let bd = context.bot_data().await;
let key = bd.get_str("key");       // Option<&str>
let count = bd.get_i64("count");   // Option<i64>
let ids = bd.get_id_set("user_ids"); // HashSet<i64>
```

### Entry point

In Python:

```python
if __name__ == "__main__":
    app = ApplicationBuilder().token(TOKEN).build()
    app.run_polling()
```

In Rust:

```rust
fn main() {
    telegram_bot::run(async {
        let app = ApplicationBuilder::new().token(token).build();
        app.add_typed_handler(CommandHandler::new("start", start), 0).await;
        app.run_polling().await.unwrap();
    });
}
```

`telegram_bot::run()` sets up a multi-threaded tokio runtime with proper stack sizing.
No `#[tokio::main]` needed.

### Imports

In Python:

```python
from telegram import Update
from telegram.ext import ApplicationBuilder, CommandHandler, MessageHandler, filters, ContextTypes
```

In Rust, a single prelude import covers everything:

```rust
use telegram_bot::ext::prelude::*;
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
async fn echo(update: Update, context: Context) -> HandlerResult {
    let text = update
        .effective_message()
        .and_then(|m| m.text.as_deref())
        .unwrap_or("");
    if !text.is_empty() {
        context.reply_text(&update, text).await?;
    }
    Ok(())
}

app.add_typed_handler(
    MessageHandler::new(TEXT() & !COMMAND(), echo),
    0,
).await;
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
async fn ban(update: Update, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().expect("must have a chat").id;
    let text = update
        .effective_message()
        .and_then(|m| m.text.as_deref())
        .unwrap_or("");

    let args: Vec<&str> = text.split_whitespace().skip(1).collect();

    if args.is_empty() {
        context.reply_text(&update, "Usage: /ban <username>").await?;
        return Ok(());
    }

    let target = args[0];
    println!("Banning: {target}");
    Ok(())
}

app.add_typed_handler(CommandHandler::new("ban", ban), 0).await;
```

### Inline keyboard with callback query

Python:
```python
async def start(update, context):
    keyboard = [[InlineKeyboardButton("Option 1", callback_data="1")]]
    await update.message.reply_text("Choose:", reply_markup=InlineKeyboardMarkup(keyboard))

async def button(update, context):
    query = update.callback_query
    await query.answer()
    await query.edit_message_text(f"Selected: {query.data}")
```

Rust:
```rust
use serde_json::json;

async fn start(update: Update, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().expect("must have a chat").id;
    let keyboard = json!({
        "inline_keyboard": [[
            {"text": "Option 1", "callback_data": "1"}
        ]]
    });

    context
        .bot()
        .send_message(chat_id, "Choose:")
        .reply_markup(keyboard)
        .send()
        .await?;
    Ok(())
}

async fn button(update: Update, context: Context) -> HandlerResult {
    let cq = update.callback_query.as_ref().expect("must have callback_query");
    let data = cq.data.as_deref().unwrap_or("?");

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

app.add_typed_handler(CommandHandler::new("start", start), 0).await;
app.add_typed_handler(FnHandler::on_callback_query(button), 0).await;
```

### Storing bot-wide data

Python:
```python
async def count(update, context):
    context.bot_data["count"] = context.bot_data.get("count", 0) + 1
    await update.message.reply_text(f"Total messages: {context.bot_data['count']}")
```

Rust:
```rust
async fn count(update: Update, context: Context) -> HandlerResult {
    let current = context.bot_data().await.get_i64("count").unwrap_or(0);

    {
        let mut bd = context.bot_data_mut().await;
        bd.set_i64("count", current + 1);
    }

    context
        .reply_text(&update, &format!("Total messages: {}", current + 1))
        .await?;
    Ok(())
}
```

### User tracking

Python:
```python
async def track_users(update, context):
    if update.effective_user:
        context.bot_data.setdefault("user_ids", set()).add(update.effective_user.id)
```

Rust:
```rust
async fn track_users(update: Update, context: Context) -> HandlerResult {
    if let Some(user) = update.effective_user() {
        let mut bd = context.bot_data_mut().await;
        bd.add_to_id_set("user_ids", user.id);
    }
    Ok(())
}

// Register in group -1 so it runs before other handlers
app.add_typed_handler(FnHandler::on_any(track_users), -1).await;
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
use telegram_bot::ext::job_queue::{JobCallbackFn, JobContext};

async fn set_timer(update: Update, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().expect("must have a chat").id;

    let jq = context.job_queue.as_ref().expect("job_queue should be set");

    let bot = Arc::clone(context.bot());
    let callback: JobCallbackFn = Arc::new(move |ctx: JobContext| {
        let bot = Arc::clone(&bot);
        Box::pin(async move {
            let target = ctx.chat_id.unwrap_or(0);
            if target != 0 {
                bot.send_message(target, "Reminder!").send().await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            }
            Ok(())
        })
    });

    jq.once(callback, Duration::from_secs(30))
        .name("reminder")
        .chat_id(chat_id)
        .start()
        .await;

    context.reply_text(&update, "Timer set for 30 seconds!").await?;
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

Rust (using manual state tracking, the most common pattern):
```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use telegram_bot::ext::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConvState { AskingName, AskingAge }

type ConvStore = Arc<RwLock<HashMap<i64, ConvState>>>;
type UserDataStore = Arc<RwLock<HashMap<i64, String>>>;

async fn start_conv(
    update: Update,
    context: Context,
    conv_store: ConvStore,
) -> HandlerResult {
    let chat_id = update.effective_chat().expect("must have a chat").id;
    conv_store.write().await.insert(chat_id, ConvState::AskingName);
    context
        .bot()
        .send_message(chat_id, "What's your name?")
        .send()
        .await?;
    Ok(())
}

async fn got_name(
    update: Update,
    context: Context,
    conv_store: ConvStore,
    user_data: UserDataStore,
) -> HandlerResult {
    let chat_id = update.effective_chat().expect("must have a chat").id;
    let text = update
        .effective_message()
        .and_then(|m| m.text.as_deref())
        .unwrap_or_default()
        .to_string();

    user_data.write().await.insert(chat_id, text.clone());
    conv_store.write().await.insert(chat_id, ConvState::AskingAge);
    context
        .bot()
        .send_message(chat_id, &format!("Nice to meet you, {text}! How old are you?"))
        .send()
        .await?;
    Ok(())
}
```

Register the state handlers with `FnHandler` and predicates that check the current state:

```rust
let cs = Arc::clone(&conv_store);
app.add_typed_handler(
    FnHandler::new(
        move |u| is_text_in_state(u, &cs, ConvState::AskingName),
        move |update, ctx| {
            let cs = Arc::clone(&conv_store);
            let ud = Arc::clone(&user_data);
            async move { got_name(update, ctx, cs, ud).await }
        },
    ),
    1,
).await;
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

- **Bot API calls**: Python -- `bot.send_message(chat_id, text, parse_mode="HTML")`. Rust -- `bot.send_message(chat_id, text).parse_mode(ParseMode::Html).send().await?`. Builder pattern eliminates positional arguments.
- **Reply shorthand**: Python -- `message.reply_text(...)`. Rust -- `context.reply_text(&update, ...)`.
- **Filter composition**: Python -- compose directly. Rust -- `TEXT()` and `COMMAND()` from prelude are pre-wrapped; other filters need `F::new(...)`.
- **FilterResult vs bool**: Python returns `bool | DataDict`. Rust returns an enum variant.
- **Error handling**: Python raises exceptions. Rust returns `Result<(), HandlerError>` and uses `?` for propagation.
- **Persistence backend**: Python has `PicklePersistence`. Rust provides JSON file and SQLite.
- **Data access**: Python -- `context.bot_data["key"]`. Rust -- `context.bot_data().await.get_str("key")` with typed guards.
- **Constants**: Python -- `ParseMode.HTML` (string). Rust -- `ParseMode::Html` (enum).
- **Job context**: Python job callbacks receive `ContextTypes`. Rust callbacks receive `JobContext` (a lightweight struct with name, chat_id, user_id, data).
- **ConversationResult**: Python callbacks return an integer state or `END`. Rust callbacks return `ConversationResult<S>` with your state type.
- **check_update return**: Python returns `bool`. Rust returns `Option<MatchResult>` so handlers can pass extracted data to callbacks without recomputing.

---

## Tips for Python Developers New to Rust

**Ownership means no shared mutable state by default.** Use `Arc<RwLock<T>>` or
`Arc<Mutex<T>>` for data shared across handler closures and job callbacks. The
`context.bot_data_mut()` method provides a typed write guard.

**Closures must own their captures** when passed across async tasks. If a callback
captures a variable, you need `move` and `Arc::clone`:

```rust
let store = Arc::clone(&store);
app.add_typed_handler(
    FnHandler::new(
        |u| check_command(u, "set"),
        move |update, ctx| {
            let s = Arc::clone(&store);
            async move { set_timer(update, ctx, s).await }
        },
    ),
    0,
).await;
```

**`?` does not work in closures that return `()`.** Job callbacks return `Result`, so use
`?` there. But if a closure returns `()`, use `if let Err(e) = ... { ... }` instead.

**Trait objects require `Send + Sync`.** All callbacks, filters, and persistence backends
must be `Send + Sync` because they are used in async tasks. If you store non-`Send` data,
wrap it in `Arc<Mutex<T>>`.

**`async fn` in traits requires Rust 1.75+.** The `BasePersistence` trait uses native
`async fn`. Make sure your toolchain is up to date.
