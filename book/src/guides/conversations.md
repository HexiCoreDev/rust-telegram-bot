# Conversations

A conversation is a multi-step interaction where the bot asks questions and the user provides answers one at a time. Each step transitions to the next state.

## State Management Pattern

In Rust, conversation state is managed with a shared `HashMap` protected by an async `RwLock`:

```rust
use telegram_bot::ext::prelude::{Arc, HashMap, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConvState {
    AskName,
    AskAge,
    AskLocation,
    AskBio,
}

type ConvStore = Arc<RwLock<HashMap<i64, ConvState>>>;
```

The key is the chat ID (or user ID), and the value is the current conversation state for that user.

## Complete Conversation Bot

This bot collects a user profile through four steps: name, age, location, and bio.

```rust
use telegram_bot::ext::prelude::{
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
        .send().await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;
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
        .send().await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;
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
        .send().await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;
    Ok(())
}
```

### State-Checking Predicates

Each state handler needs a predicate that checks whether the user is in the correct state:

```rust
fn is_text_in_state(update: &Update, conv_store: &ConvStore, state: ConvState) -> bool {
    let msg = match update.effective_message() {
        Some(m) => m,
        None => return false,
    };
    if msg.text.is_none() {
        return false;
    }
    // Exclude commands
    let is_cmd = msg.entities.as_ref()
        .and_then(|ents| ents.first())
        .map(|e| e.entity_type == MessageEntityType::BotCommand && e.offset == 0)
        .unwrap_or(false);
    if is_cmd {
        return false;
    }
    let chat_id = msg.chat.id;
    conv_store.try_read()
        .map(|guard| guard.get(&chat_id) == Some(&state))
        .unwrap_or(false)
}
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
        app.add_typed_handler(
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
        app.add_typed_handler(
            FnHandler::new(
                |u| check_command(u, "cancel"),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { cancel(update, ctx, cs).await }
                },
            ), 0,
        ).await;
    }

    // State: AskName (group 1 so commands in group 0 take priority)
    {
        let cs = Arc::clone(&conv_store);
        let ud = Arc::clone(&user_data);
        let cs_check = Arc::clone(&conv_store);
        app.add_typed_handler(
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

### Use Group Numbers for Priority

Register command handlers (like `/cancel`) in group 0 and state handlers in group 1. This ensures commands always take priority over state-based text matching.

### Clone Arc Before Moving Into Closures

Every `Arc::clone()` is cheap (it increments an atomic counter). Always clone before moving into a closure:

```rust
let cs = Arc::clone(&conv_store);
app.add_typed_handler(
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
