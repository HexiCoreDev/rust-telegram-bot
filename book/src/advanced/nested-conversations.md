# Nested Conversations

Nested conversations let you build multi-level state machines where selecting an option in one conversation enters a child conversation, and finishing the child returns control to the parent. This is useful for complex data collection flows.

## Architecture

A nested conversation is modelled as a state machine with multiple levels:

```
Level 1 (Top):     [Add member] [Add self] [Show data] [Done]
                          |
Level 2 (Member):  [Add parent] [Add child] [Show data] [Back]
                          |
Level 3 (Features): [Name] [Age] [Done]
```

Each level has its own set of states. Transitioning "down" enters a child level. Transitioning "up" returns to the parent via state restoration.

## Defining States

Use enums to model each level:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TopState {
    SelectingAction,
    AddingMember,
    DescribingSelf,
    ShowingData,
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MemberState {
    SelectingLevel,
    SelectingGender,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FeatureState {
    SelectingFeature,
    Typing,
}
```

Combine them into a single discriminated state:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
enum ConvState {
    Top(TopState),
    Member(MemberState),
    Feature(FeatureState),
    End,
}

impl Default for ConvState {
    fn default() -> Self {
        ConvState::End
    }
}
```

## Shared State Store

Store per-user conversation state in a thread-safe map:

```rust
use telegram_bot::ext::prelude::{Arc, HashMap, RwLock};

#[derive(Debug, Clone, Default)]
struct PersonInfo {
    gender: Option<String>,
    name: Option<String>,
    age: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct UserState {
    conv: ConvState,
    current_level: String,
    current_feature: String,
    current_person: PersonInfo,
    family: HashMap<String, Vec<PersonInfo>>,
}

type StateStore = Arc<RwLock<HashMap<i64, UserState>>>;
```

## State-Based Predicates

Each handler fires only when the user is in the correct state. Write predicate functions that check the store:

```rust
fn is_in_state(store: &StateStore, user_id: i64, expected: &ConvState) -> bool {
    store
        .try_read()
        .map(|guard| {
            guard
                .get(&user_id)
                .map(|us| &us.conv == expected)
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

fn is_callback_in_top_state(update: &Update, store: &StateStore) -> bool {
    if update.callback_query().is_none() {
        return false;
    }
    let user_id = match update.effective_user() {
        Some(u) => u.id,
        None => return false,
    };
    is_in_state(store, user_id, &ConvState::Top(TopState::SelectingAction))
        || is_in_state(store, user_id, &ConvState::Top(TopState::ShowingData))
}
```

Use `try_read()` instead of `.read().await` because predicates are synchronous -- they run inside the `FnHandler`'s filter closure, which cannot be async.

## Entering a Child Level

When the user selects "Add member" at the top level, transition to the member level:

```rust
use telegram_bot::ext::prelude::{
    Arc, Context, HandlerError, HandlerResult, InlineKeyboardButton,
    InlineKeyboardMarkup, Update,
};

async fn handle_top_action(
    update: Arc<Update>,
    context: Context,
    store: StateStore,
) -> HandlerResult {
    let user_id = update.effective_user().unwrap().id;
    let cq = update.callback_query().expect("must have callback_query");
    let data = cq.data.as_deref().unwrap_or("");

    context
        .bot()
        .answer_callback_query(&cq.id)
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    let msg = cq.message.as_deref().expect("must have message");
    let chat_id = msg.chat().id;

    match data {
        "add_member" => {
            // Transition DOWN to Member level
            let mut s = store.write().await;
            let us = s.entry(user_id).or_default();
            us.conv = ConvState::Member(MemberState::SelectingLevel);

            let keyboard = serde_json::to_value(InlineKeyboardMarkup::new(vec![
                vec![
                    InlineKeyboardButton::callback("Add parent", "parents"),
                    InlineKeyboardButton::callback("Add child", "children"),
                ],
                vec![
                    InlineKeyboardButton::callback("Show data", "show"),
                    InlineKeyboardButton::callback("Back", "back"),
                ],
            ])).unwrap();

            context
                .bot()
                .edit_message_text("Choose a member type or go back.")
                .chat_id(chat_id)
                .message_id(msg.message_id())
                .reply_markup(keyboard)
                .await
                .map_err(|e| HandlerError::Other(Box::new(e)))?;
        }
        // ... handle other top-level actions ...
        _ => {}
    }

    Ok(())
}
```

## Returning to a Parent Level

When the user presses "Back" or "Done" at a child level, restore the parent state:

```rust
// Inside the member-level handler:
"back" => {
    // Transition UP to Top level
    let mut s = store.write().await;
    let us = s.entry(user_id).or_default();
    us.conv = ConvState::Top(TopState::SelectingAction);
    drop(s);

    context
        .bot()
        .edit_message_text("Choose an action.")
        .chat_id(chat_id)
        .message_id(msg.message_id())
        .reply_markup(top_menu_keyboard())
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;
}
```

When "Done" is pressed in the feature level, save the collected data and return to either the member level (for family members) or the top level (for self):

```rust
"done" => {
    let mut s = store.write().await;
    let us = s.entry(user_id).or_default();
    let level = us.current_level.clone();
    let person = us.current_person.clone();
    us.family.entry(level.clone()).or_default().push(person);
    us.current_person = PersonInfo::default();

    if level == "self" {
        // Return to top level
        us.conv = ConvState::Top(TopState::SelectingAction);
    } else {
        // Return to member level
        us.conv = ConvState::Member(MemberState::SelectingLevel);
    }
}
```

## Handling Free-Text Input

When the user is in a "typing" state, text messages should be captured as feature values instead of being treated as commands:

```rust
use telegram_bot::ext::prelude::MessageEntityType;

fn is_text_in_typing_state(update: &Update, store: &StateStore) -> bool {
    let msg = match update.effective_message() {
        Some(m) => m,
        None => return false,
    };
    if msg.text.is_none() {
        return false;
    }
    // Exclude commands
    let is_cmd = msg
        .entities
        .as_ref()
        .and_then(|ents| ents.first())
        .map(|e| {
            e.entity_type == MessageEntityType::BotCommand && e.offset == 0
        })
        .unwrap_or(false);
    if is_cmd {
        return false;
    }
    let user_id = match update.effective_user() {
        Some(u) => u.id,
        None => return false,
    };
    is_in_state(store, user_id, &ConvState::Feature(FeatureState::Typing))
}

async fn handle_text_input(
    update: Arc<Update>,
    context: Context,
    store: StateStore,
) -> HandlerResult {
    let user_id = update.effective_user().unwrap().id;
    let chat_id = update.effective_chat().unwrap().id;
    let text = update
        .effective_message()
        .and_then(|m| m.text.as_deref())
        .unwrap_or("")
        .to_string();

    let mut s = store.write().await;
    let us = s.entry(user_id).or_default();

    match us.current_feature.as_str() {
        "name" => us.current_person.name = Some(text),
        "age" => us.current_person.age = Some(text),
        _ => {}
    }
    us.conv = ConvState::Feature(FeatureState::SelectingFeature);
    drop(s);

    context
        .bot()
        .send_message(chat_id, "Got it! Please select a feature to update.")
        .reply_markup(feature_keyboard())
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}
```

## Registering Handlers with Shared State

Each handler needs its own clones of the store for both the predicate and the handler body:

```rust
use telegram_bot::ext::prelude::{
    Application, ApplicationBuilder, Arc, FnHandler, HashMap, RwLock,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let app: Arc<Application> = ApplicationBuilder::new().token(token).build();
    let store: StateStore = Arc::new(RwLock::new(HashMap::new()));

    // Top-level callback handler
    {
        let s = Arc::clone(&store);
        let s_check = Arc::clone(&store);
        app.add_typed_handler(
            FnHandler::new(
                move |u| is_callback_in_top_state(u, &s_check),
                move |update, ctx| {
                    let s = Arc::clone(&s);
                    async move { handle_top_action(update, ctx, s).await }
                },
            ),
            1,
        ).await;
    }

    // Text input handler
    {
        let s = Arc::clone(&store);
        let s_check = Arc::clone(&store);
        app.add_typed_handler(
            FnHandler::new(
                move |u| is_text_in_typing_state(u, &s_check),
                move |update, ctx| {
                    let s = Arc::clone(&s);
                    async move { handle_text_input(update, ctx, s).await }
                },
            ),
            1,
        ).await;
    }

    // ... register other level handlers in group 1 ...

    app.run_polling().await.unwrap();
}
```

The pattern for each handler is:

1. Clone the store twice -- once for the predicate closure, once for the handler closure.
2. The predicate closure captures `s_check` and calls `is_in_state`.
3. The handler closure captures `s`, clones it into the async block, and passes it to the handler function.
4. Register all conversation handlers in group 1 (or higher) so command handlers in group 0 get priority.

## Timeout Handling

Add a `/stop` command that resets the conversation from any level:

```rust
async fn stop_command(
    update: Arc<Update>,
    context: Context,
    store: StateStore,
) -> HandlerResult {
    let chat_id = update.effective_chat().unwrap().id;
    let user_id = update.effective_user().unwrap().id;

    {
        let mut s = store.write().await;
        let us = s.entry(user_id).or_default();
        us.conv = ConvState::End;
    }

    context
        .bot()
        .send_message(chat_id, "Okay, bye.")
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}
```

Register it in group 0 so it takes priority over conversation handlers:

```rust
{
    let s = Arc::clone(&store);
    app.add_typed_handler(
        FnHandler::new(
            |u| check_command(u, "stop"),
            move |update, ctx| {
                let s = Arc::clone(&s);
                async move { stop_command(update, ctx, s).await }
            },
        ),
        0,
    ).await;
}
```

## Design Tips

- **Group ordering matters.** Register `/start` and `/stop` in group 0 so they always fire. Register conversation handlers in group 1.
- **Use `try_read` in predicates.** Predicates are synchronous. Using `.read().await` would require an async context that is not available during filter evaluation.
- **Drop write guards early.** Call `drop(s)` after modifying state and before making async API calls to avoid holding the lock across `await` points.
- **Store per-user, not per-chat.** In group chats, multiple users might have independent conversations. Key the store by user ID.
- **Combine with persistence.** For conversations that should survive restarts, store the conversation state in `user_data` via `context.set_user_data()` instead of an in-memory `HashMap`.

## Next Steps

- [Conversations](../guides/conversations.md) -- simpler single-level conversations.
- [Persistence](../guides/persistence.md) -- persist conversation state across restarts.
