# Conversations

A conversation is a multi-step interaction where the bot asks questions and the user provides answers one at a time. Each step transitions to the next state.

## Use `ConversationHandler` First

If you are coming from python-telegram-bot, this is the API you want. It is a
direct port: entry points, per-state handlers, fallbacks, timeouts and
persistence all behave the way you expect, and you do not write any state
plumbing yourself.

```rust
use std::hash::Hash;
use std::time::Duration;

use rust_tg_bot::ext::handlers::base::{FnHandler, Handler, HandlerResult};
use rust_tg_bot::ext::handlers::command::CommandHandler;
use rust_tg_bot::ext::handlers::conversation::{
    ConversationHandler, ConversationResult, ConversationStepHandler,
};
use rust_tg_bot::ext::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    AskName,
    AskAge,
}

// Each step pairs a normal handler (used for `check_update`) with the state
// transition to perform when that handler matches.
//
// Two different `HandlerResult` types are in play, which is worth knowing up
// front: the prelude's `HandlerResult` is an alias for
// `Result<(), HandlerError>`, and the `Continue` here comes from the *trait*
// enum in `handlers::base`, which also has `Stop` and `Error(_)`. The
// conversation layer consumes that inner value and turns it into the outer
// Result; you only need the trait variant when writing a `step` by hand.
fn step<S, H>(handler: H, next: ConversationResult<S>) -> ConversationStepHandler<S>
where
    H: Handler + 'static,
    S: Hash + Eq + Clone + Send + Sync + 'static,
{
    ConversationStepHandler {
        handler: Box::new(handler),
        conv_callback: Arc::new(move |_update, _match| {
            let next = next.clone();
            Box::pin(async move { (HandlerResult::Continue, next) })
        }),
    }
}

// A step that matches any update and just moves the conversation along.
fn advance(next: ConversationResult<State>) -> ConversationStepHandler<State> {
    step(FnHandler::new(|_update| true, |_update, _ctx| async { Ok(()) }), next)
}

async fn register(app: Arc<Application>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let conversation = ConversationHandler::builder()
        .entry_point(step(
            CommandHandler::new("signup", |_update, _ctx| async { Ok(()) }),
            ConversationResult::NextState(State::AskName),
        ))
        .state(State::AskName, vec![advance(ConversationResult::NextState(State::AskAge))])
        .state(State::AskAge, vec![advance(ConversationResult::End)])
        .per_chat(true)
        .per_user(true)
        .conversation_timeout(Duration::from_secs(300))
        .build();

    app.add_handler(conversation, 0).await;
    Ok(())
}
```

Key builder methods:

| Method | Purpose |
|---|---|
| `entry_point(step)` / `entry_points(vec![..])` | Handlers that start a conversation |
| `state(state, vec![..])` | Handlers active while in that state |
| `fallbacks(vec![..])` | Handlers active in any state (e.g. `/cancel`) |
| `conversation_timeout(Duration)` | End the conversation after inactivity |
| `per_chat` / `per_user` / `per_message` | Which ids make up the conversation key |
| `map_to_parent(HashMap<S, S>)` | Nested conversations — see [Nested Conversations](../advanced/nested-conversations.md) |
| `persistent(bool)` | Survive restarts via the configured persistence backend |

`ConversationResult` has three variants: `NextState(S)` transitions,
`End` finishes, and `Stay` leaves the current state unchanged.

## The Manual Pattern

The rest of this page builds the same thing by hand with a shared `HashMap`
protected by an async `RwLock`. You need this only when you want state that
`ConversationHandler` does not model — but prefer the builder above unless you
have a specific reason.

```rust
use rust_tg_bot::ext::prelude::{Arc, HashMap, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConvState {
    AskName,
    AskAge,
    AskLocation,
    AskBio,
}

type ConvStore = Arc<RwLock<HashMap<i64, ConvState>>>;
```

> This page trims the runnable example down to the states each section is
> teaching, so the enum declares only the variants the surrounding prose uses.
> The full
> [`conversation_bot.rs`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/conversation_bot.rs)
> names the same states `Name`, `Age`, `Location`, and `Bio` -- same shape,
> different labels. Either version compiles on its own; only mix the spellings
> if you rename one side.

The key is the chat ID (or user ID), and the value is the current conversation state for that user.

## Complete Conversation Bot

This bot collects a user profile through four steps: name, age, location, and bio.

```rust
use rust_tg_bot::ext::prelude::{
    Application, ApplicationBuilder, Arc, Context, FnHandler,
    HandlerError, HandlerResult, HashMap, MessageEntityType, RwLock, Update,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConvState {
    AskName,
    AskAge,
    AskLocation,
    AskBio,
}

type ConvStore = Arc<RwLock<HashMap<i64, ConvState>>>;
type UserDataStore = Arc<RwLock<HashMap<i64, UserProfile>>>;

#[derive(Debug, Clone, Default)]
struct UserProfile {
    name: Option<String>,
    age: Option<String>,
    location: Option<String>,
    bio: Option<String>,
}
```

### Handler Functions

Each handler function receives the shared stores through closure captures:

```rust
async fn start_command(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
) -> HandlerResult {
    let chat_id = update.effective_chat().unwrap().id;
    conv_store.write().await.insert(chat_id, ConvState::AskName);

    context.bot()
        .send_message(chat_id, "Hi! What is your name? (Send /cancel to stop.)")
        .await?;
    Ok(())
}

async fn receive_name(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
    user_data: UserDataStore,
) -> HandlerResult {
    let chat_id = update.effective_chat().unwrap().id;
    let text = update.effective_message()
        .and_then(|m| m.text.as_deref())
        .unwrap_or_default();

    user_data.write().await.entry(chat_id).or_default().name = Some(text.to_string());
    conv_store.write().await.insert(chat_id, ConvState::AskAge);

    context.bot()
        .send_message(chat_id, &format!("Nice to meet you, {text}! How old are you?"))
        .await?;
    Ok(())
}

async fn cancel(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
) -> HandlerResult {
    let chat_id = update.effective_chat().unwrap().id;
    conv_store.write().await.remove(&chat_id);

    context.bot()
        .send_message(chat_id, "Conversation cancelled. Send /start to begin again.")
        .await?;
    Ok(())
}
```

### State-Checking Predicates

Each state handler needs a predicate that checks whether the user is in the correct state:

The predicate is the same one used in the runnable examples, included verbatim:

```rust,ignore
{{#include ../../../crates/telegram-bot/examples/conversation_bot2.rs:is_text_in_state}}
```

### Wiring It Together

```rust
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let app: Arc<Application> = ApplicationBuilder::new().token(token).build();

    let conv_store: ConvStore = Arc::new(RwLock::new(HashMap::new()));
    let user_data: UserDataStore = Arc::new(RwLock::new(HashMap::new()));

    // /start entry point
    {
        let cs = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                |u| check_command(u, "start"),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { start_command(update, ctx, cs).await }
                },
            ), 0,
        ).await;
    }

    // /cancel fallback
    {
        let cs = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                |u| check_command(u, "cancel"),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { cancel(update, ctx, cs).await }
                },
            ), 0,
        ).await;
    }

    // State: AskName (group 1; correctness comes from the predicate excluding
    // commands, not from the group number -- see "Group Numbers Are Not Priority")
    {
        let cs = Arc::clone(&conv_store);
        let ud = Arc::clone(&user_data);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| is_text_in_state(u, &cs_check, ConvState::AskName),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    let ud = Arc::clone(&ud);
                    async move { receive_name(update, ctx, cs, ud).await }
                },
            ), 1,
        ).await;
    }

    // Repeat for AskAge, AskLocation, AskBio...

    app.run_polling().await.unwrap();
}
```

## Key Patterns

### Group Numbers Are Not Priority

Handlers in group `0` are matched before group `1`, but **every group is
evaluated independently** — a lower-numbered group does not "win" over an
earlier match in a higher one, and a handler that matches in group 1 does not
stop group 0 from also matching.

Putting commands in group 0 and state handlers in group 1 only works because
the state predicates themselves exclude commands:

```rust
// This is what makes the ordering correct -- the `& !COMMAND()` part.
fn is_text_in_state(update: &Update, store: &ConvStore, state: ConvState) -> bool {
    text(update) && !is_command(update) && current_state(update, store) == state
}
```

Without that exclusion, a `/cancel` typed mid-conversation would match both the
command handler and the state handler.

Negative group numbers (e.g. `-1`) are always evaluated last, which makes them
a good home for bookkeeping that must run after everything else.

### Clone Arc Before Moving Into Closures

Every `Arc::clone()` is cheap (it increments an atomic counter). Always clone before moving into a closure:

```rust
let cs = Arc::clone(&conv_store);
app.add_handler(
    FnHandler::new(
        |u| check_command(u, "start"),
        move |update, ctx| {
            let cs = Arc::clone(&cs);
            async move { start_command(update, ctx, cs).await }
        },
    ), 0,
).await;
```

### Use try_read() in Predicates

Predicates run synchronously, so use `try_read()` instead of `.read().await`:

```rust
conv_store.try_read()
    .map(|guard| guard.get(&chat_id) == Some(&state))
    .unwrap_or(false)
```

## Next Steps

- [Persistence](persistence.md) -- save conversation state across bot restarts.
- [Nested Conversations](../advanced/nested-conversations.md) -- multi-level state machines.

## Runnable examples

The code above is abridged. These run in full:

| Example | What to look at |
|---|---|
| [`conversation_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/conversation_bot.rs) | the smallest complete state machine |
| [`conversation_bot2`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/conversation_bot2.rs) | branching states and fallbacks |
| [`nested_conversation_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/nested_conversation_bot.rs) | a conversation nested inside another |

Every example is listed in the [example index](../examples.md).
