# Handlers

Handlers are the core building block of a bot. Each handler decides whether it is
interested in an update (`check_update`) and then processes it (`handle_update_with_context`).

---

## The Handler Trait

```rust
pub trait Handler: Send + Sync {
    fn check_update(&self, update: &Update) -> Option<MatchResult>;

    fn handle_update(
        &self,
        update: Update,
        match_result: MatchResult,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>>;

    fn block(&self) -> bool { true }

    fn collect_additional_context(
        &self,
        context: &mut CallbackContext,
        match_result: &MatchResult,
    ) {}

    fn handle_update_with_context(
        &self,
        update: Update,
        match_result: MatchResult,
        context: CallbackContext,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> {
        self.handle_update(update, match_result)
    }
}
```

`check_update` is synchronous. Return `None` to skip, `Some(MatchResult)` to handle.

`handle_update_with_context` is the primary dispatch point. Typed handlers override this
to pass the `CallbackContext` to your callback function. The user callback signature is:

```rust
async fn my_handler(update: Update, context: Context) -> HandlerResult
```

`block` controls concurrency. `true` (default) means the application awaits the handler
future before moving on. `false` spawns the future as a background task.

`collect_additional_context` populates `CallbackContext` fields (e.g. `context.args`,
`context.matches`) before the callback is invoked.

---

## MatchResult

```rust
pub enum MatchResult {
    Empty,
    Args(Vec<String>),
    RegexMatch(Vec<String>),
    RegexMatchWithNames {
        positional: Vec<String>,
        named: HashMap<String, String>,
    },
    Custom(Box<dyn Any + Send>),
}
```

Different handlers produce different variants:
- `CommandHandler` produces `Args`.
- `CallbackQueryHandler` with a plain regex produces `RegexMatch`; with named groups
  produces `RegexMatchWithNames`.
- `MessageHandler` produces `Empty` for simple filter matches, or `Custom` wrapping
  `HashMap<String, Vec<String>>` for data filters.

---

## HandlerResult

The type alias `HandlerResult` is `Result<(), HandlerError>`. When a handler returns
`Ok(())`, processing continues to the next group. When it returns an error, the error
handler runs. The internal `HandlerResult` enum maps as follows:

- `Ok(())` maps to `Continue` -- processing moves to the next handler group.
- `Err(HandlerError::HandlerStop)` maps to `Stop` -- processing ends for this update.
- `Err(HandlerError::Other(e))` maps to `Error` -- logged, error handler runs, processing continues.

---

## Handler Groups and Dispatch Order

Handlers are registered with a group number (any `i32`). Groups are processed in
ascending numeric order. Within a group, the first matching handler wins -- others in the
same group are not called.

```rust
use telegram_bot::ext::prelude::*;

// Group -1: always runs first (e.g., user tracking)
app.add_typed_handler(FnHandler::on_any(track_users), -1).await;

// Group 0: command handlers (checked first within this group)
app.add_typed_handler(CommandHandler::new("start", start), 0).await;
app.add_typed_handler(CommandHandler::new("help", help), 0).await;

// Group 1: message handlers (checked if no command matched, or group 0 returned Continue)
app.add_typed_handler(
    MessageHandler::new(TEXT() & !COMMAND(), echo),
    1,
).await;

// Group 2: logging handler (always runs)
app.add_typed_handler(FnHandler::on_any(log_update), 2).await;
```

Typical layout for a medium-complexity bot:
- Group -1: audit/tracking (catch-all, always runs)
- Group 0: `CommandHandler`s and `CallbackQueryHandler`s
- Group 1: `MessageHandler`s with filters
- Group 2: fallback or logging handlers

---

## CommandHandler

Matches messages whose first entity is a `bot_command` at offset 0.

```rust
use telegram_bot::ext::prelude::*;

async fn start(update: Update, context: Context) -> HandlerResult {
    let name = update
        .effective_user()
        .map(|u| u.first_name.as_str())
        .unwrap_or("there");
    context.reply_text(&update, &format!("Hello, {name}!")).await?;
    Ok(())
}

// Register: matches /start
app.add_typed_handler(CommandHandler::new("start", start), 0).await;
```

`CommandHandler::new` takes a command name (without the `/` prefix) and an async callback.

---

## MessageHandler

Matches updates based on a composable filter.

```rust
use telegram_bot::ext::prelude::*;

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

// Match any text that is not a command
app.add_typed_handler(
    MessageHandler::new(TEXT() & !COMMAND(), echo),
    0,
).await;
```

The filter argument is an `F` value. Use the `TEXT()` and `COMMAND()` functions from the
prelude, which return pre-wrapped `F` values ready for composition with `&`, `|`, and `!`.

---

## FnHandler

A lightweight handler that pairs a predicate with an async callback. This is the most
flexible handler type -- it can match any kind of update.

### Convenience constructors

```rust
use telegram_bot::ext::prelude::*;

// Match callback queries (inline keyboard button presses)
app.add_typed_handler(FnHandler::on_callback_query(button), 0).await;

// Match inline queries (@bot ... in any chat)
app.add_typed_handler(FnHandler::on_inline_query(inline), 0).await;

// Match every update (catch-all, good for logging or user tracking)
app.add_typed_handler(FnHandler::on_any(track_users), -1).await;

// Match updates with a message
app.add_typed_handler(FnHandler::on_message(handle_msg), 0).await;

// Match polls, poll answers, shipping queries, etc.
app.add_typed_handler(FnHandler::on_poll(handle_poll), 0).await;
app.add_typed_handler(FnHandler::on_poll_answer(handle_answer), 0).await;
app.add_typed_handler(FnHandler::on_shipping_query(handle_shipping), 0).await;
app.add_typed_handler(FnHandler::on_pre_checkout_query(handle_checkout), 0).await;
app.add_typed_handler(FnHandler::on_chat_member(handle_member), 0).await;
app.add_typed_handler(FnHandler::on_my_chat_member(handle_my_member), 0).await;
```

### Custom predicate

```rust
use telegram_bot::ext::prelude::*;

// Match callback queries with specific data
app.add_typed_handler(
    FnHandler::new(
        |u| {
            u.callback_query
                .as_ref()
                .and_then(|cq| cq.data.as_deref())
                .map_or(false, |d| d == "button")
        },
        count_click,
    ),
    0,
).await;
```

### Closures with captured state

When handlers need shared state, clone `Arc` references into the closure:

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use telegram_bot::ext::prelude::*;

let store: Arc<RwLock<HashMap<i64, u64>>> = Arc::new(RwLock::new(HashMap::new()));

{
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
}
```

---

## Callback Query Handling

Callback queries come from inline keyboard buttons. Use `FnHandler::on_callback_query`
for the simplest case:

```rust
use telegram_bot::ext::prelude::*;

async fn button_callback(update: Update, context: Context) -> HandlerResult {
    let cq = update.callback_query.as_ref()
        .expect("handler requires callback_query");

    let data = cq.data.as_deref().unwrap_or("unknown");

    // Answer the callback query (removes the loading indicator)
    context
        .bot()
        .answer_callback_query(&cq.id)
        .send()
        .await?;

    // Edit the original message
    if let Some(ref msg) = cq.message {
        context
            .bot()
            .edit_message_text(&format!("You selected: {data}"))
            .chat_id(msg.chat().id)
            .message_id(msg.message_id())
            .send()
            .await?;
    }

    Ok(())
}

app.add_typed_handler(FnHandler::on_callback_query(button_callback), 0).await;
```

---

## ConversationHandler

Manages a per-user, per-chat state machine. This is the most complex handler.

### Basic Setup

```rust
use telegram_bot::ext::handlers::conversation::*;

#[derive(Clone, Hash, Eq, PartialEq)]
enum State {
    AskName,
    AskAge,
}

let conv = ConversationHandler::builder()
    .entry_point(start_step)
    .state(State::AskName, vec![name_step])
    .state(State::AskAge, vec![age_step])
    .build();
```

### Configuration Options

```rust
ConversationHandler::builder()
    .entry_point(start_step)
    .state(State::AskName, vec![name_step])
    .fallback(cancel_step)               // tried when no state handler matches
    .allow_reentry(true)                 // user can restart while in conversation
    .per_chat(true)                      // include chat_id in key (default: true)
    .per_user(true)                      // include user_id in key (default: true)
    .per_message(false)                  // include message_id (for inline keyboards)
    .conversation_timeout(Duration::from_secs(300))  // idle timeout
    .persistent(true)                    // save state to persistence backend
    .name("registration".to_string())    // required when persistent = true
    .build()
```

### Manual conversation state tracking

For simpler cases, you can track state manually with a shared `HashMap`:

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use telegram_bot::ext::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConvState { AskName, AskAge }

type ConvStore = Arc<RwLock<HashMap<i64, ConvState>>>;

async fn start_command(
    update: Update,
    context: Context,
    conv_store: ConvStore,
) -> HandlerResult {
    let chat_id = update.effective_chat().expect("update must have a chat").id;
    conv_store.write().await.insert(chat_id, ConvState::AskName);
    context
        .bot()
        .send_message(chat_id, "What is your name?")
        .send()
        .await?;
    Ok(())
}

// Register with FnHandler and captured state
let cs = Arc::clone(&conv_store);
app.add_typed_handler(
    FnHandler::new(
        |u| check_command(u, "start"),
        move |update, ctx| {
            let cs = Arc::clone(&cs);
            async move { start_command(update, ctx, cs).await }
        },
    ),
    0,
).await;
```

---

## All Handler Types (via FnHandler)

`FnHandler` provides convenience constructors for every update type. Combined with custom
predicates, it covers all 21+ handler scenarios:

| Constructor | When it fires |
|-------------|---------------|
| `FnHandler::on_callback_query(cb)` | Inline keyboard button press |
| `FnHandler::on_inline_query(cb)` | `@bot query` inline input |
| `FnHandler::on_poll(cb)` | Poll update (new or stopped) |
| `FnHandler::on_poll_answer(cb)` | User answered a poll |
| `FnHandler::on_shipping_query(cb)` | Shipping address provided |
| `FnHandler::on_pre_checkout_query(cb)` | Payment pre-checkout |
| `FnHandler::on_chat_member(cb)` | Chat member status change |
| `FnHandler::on_my_chat_member(cb)` | Bot's own membership change |
| `FnHandler::on_message(cb)` | Any message update |
| `FnHandler::on_any(cb)` | Every update (catch-all) |
| `FnHandler::new(predicate, cb)` | Custom predicate |

For specialized matching (regex on callback data, command argument validation, etc.),
use `FnHandler::new` with a custom predicate closure.
