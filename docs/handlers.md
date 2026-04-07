# Handlers

Handlers are the core building block of a bot. Each handler decides whether it is
interested in an update (`check_update`) and then processes it (`handle_update`).

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
}
```

`check_update` is synchronous. Return `None` to skip, `Some(MatchResult)` to handle.

`handle_update` receives the `MatchResult` from `check_update`. This allows handlers to
pass extracted data (arguments, regex captures) without re-computing it.

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

```rust
pub enum HandlerResult {
    Continue,
    Stop,
    Error(Box<dyn std::error::Error + Send + Sync>),
}
```

`Continue` lets processing move to the next handler group. `Stop` ends processing for
this update. `Error` is logged and processing continues (the error handler callback
runs if registered).

---

## Handler Groups and Dispatch Order

Handlers are registered with a group number (any `i32`). Groups are processed in
ascending numeric order. Within a group, the first matching handler wins -- others in the
same group are not called.

```rust
// Group 0: checked first
app.add_handler(start_handler, 0).await;
app.add_handler(help_handler, 0).await;

// Group 1: checked if group 0 produced no match, or if group 0 returned Continue
app.add_handler(echo_handler, 1).await;

// Group 2: always runs unless a handler in an earlier group returned Stop
app.add_handler(logger_handler, 2).await;
```

Typical layout for a medium-complexity bot:
- Group 0: `ConversationHandler` (must intercept updates before other handlers)
- Group 1: `CommandHandler`s
- Group 2: `MessageHandler`s
- Group 3: error handlers or audit loggers

---

## CommandHandler

Matches messages whose first entity is a `bot_command` at offset 0. Commands are
1-32 lowercase alphanumeric or underscore characters (Telegram's requirement).

```rust
use telegram_bot_ext::handlers::command::{CommandHandler, HasArgs};
use telegram_bot_ext::handlers::base::{HandlerResult, MatchResult};
use std::sync::Arc;

let handler = CommandHandler::new(
    vec!["start".into(), "help".into()],
    Arc::new(|update, match_result| Box::pin(async move {
        // match_result is MatchResult::Args(args)
        if let MatchResult::Args(args) = match_result {
            println!("Args: {:?}", args);
        }
        HandlerResult::Continue
    })),
    None,  // HasArgs: None means any number of arguments
    true,  // block: await before next group
);
```

### HasArgs

Controls how many arguments a command must have to match:

| Variant | Behaviour |
|---------|-----------|
| `HasArgs::Any` (default) | Accept any number of arguments, including zero |
| `HasArgs::NonEmpty` | Require at least one argument |
| `HasArgs::None` | Require zero arguments |
| `HasArgs::Exact(n)` | Require exactly `n` arguments |

```rust
// Only match /kick when a user argument is provided
CommandHandler::new(
    vec!["kick".into()],
    callback,
    Some(HasArgs::NonEmpty),
    true,
)
```

### @botname Validation

In group chats, users often write `/start@MyBot`. By default the `@botname` suffix is
silently ignored (backwards compatible). To validate it:

```rust
CommandHandler::new(vec!["start".into()], callback, None, true)
    .with_bot_username("MyBot")
```

Validation is case-insensitive. If the suffix does not match the configured username,
the update is rejected. If no `@suffix` is present, the command is always accepted.

### Custom Update Filter

By default `CommandHandler` accepts updates with a `message` or `edited_message` field
(matching the Python default of `filters.UpdateType.MESSAGES`). Override with a custom
filter:

```rust
CommandHandler::new(vec!["start".into()], callback, None, true)
    .with_filter(Arc::new(|update| {
        // Only match in private chats
        update.message.as_ref()
            .and_then(|m| m.chat.chat_type.as_deref())
            .map_or(false, |t| t == "private")
    }))
```

---

## MessageHandler

Matches updates based on a composable filter. When no filter is provided, every update
matches.

```rust
use telegram_bot_ext::handlers::message::MessageHandler;
use telegram_bot_ext::handlers::base::{HandlerResult, MatchResult};
use telegram_bot_ext::filters::base::{F, All};
use telegram_bot_ext::filters::text::TEXT;
use telegram_bot_ext::filters::command::COMMAND;
use std::sync::Arc;

// Match any text that is not a command
let text_only = F::new(TEXT) & !F::new(COMMAND);

let handler = MessageHandler::new(
    Some(text_only),
    Arc::new(|update, _mr| Box::pin(async move {
        HandlerResult::Continue
    })),
    true,
);
```

If the filter returns `FilterResult::MatchWithData`, the data map is passed through as
`MatchResult::Custom`. Use a downcast to access it in your callback.

---

## CallbackQueryHandler

Handles `callback_query` updates (from inline keyboard buttons).

```rust
use telegram_bot_ext::handlers::callback_query::{CallbackQueryHandler, CallbackPattern};
use telegram_bot_ext::handlers::base::{HandlerResult, MatchResult};
use regex::Regex;
use std::sync::Arc;

// Match any callback query (no pattern filter)
let any_cq = CallbackQueryHandler::new(
    Arc::new(|_update, _mr| Box::pin(async { HandlerResult::Continue })),
    None,
    true,
);

// Match data against a regex; positional captures in MatchResult::RegexMatch
let regex_cq = CallbackQueryHandler::new(
    Arc::new(|_update, mr| Box::pin(async move {
        if let MatchResult::RegexMatch(caps) = mr {
            println!("Button ID: {}", caps[1]);
        }
        HandlerResult::Continue
    })),
    Some(CallbackPattern::Data(Regex::new(r"^btn_(\d+)$").unwrap())),
    true,
);

// Named capture groups -> MatchResult::RegexMatchWithNames
let named_cq = CallbackQueryHandler::new(
    Arc::new(|_update, mr| Box::pin(async move {
        if let MatchResult::RegexMatchWithNames { named, .. } = mr {
            println!("ID: {}", named.get("id").unwrap());
        }
        HandlerResult::Continue
    })),
    Some(CallbackPattern::Data(Regex::new(r"^btn_(?P<id>\d+)$").unwrap())),
    true,
);

// Predicate-based: replaces Python's callable(data) and isinstance(data, Type) patterns
let predicate_cq = CallbackQueryHandler::new(
    Arc::new(|_update, _mr| Box::pin(async { HandlerResult::Continue })),
    Some(CallbackPattern::Predicate(Arc::new(|data| {
        data.starts_with("action_")
    }))),
    true,
);
```

Context fields populated by `collect_additional_context`:
- `context.matches` -- positional capture groups (index 0 = full match)
- `context.named_matches` -- named capture groups, if the pattern contains any

---

## ConversationHandler

Manages a per-user, per-chat state machine. This is the most complex handler.

### Basic Setup

```rust
use telegram_bot_ext::handlers::conversation::{
    ConversationHandler, ConversationResult,
};
use telegram_bot_ext::handlers::base::{HandlerResult, MatchResult};
use telegram_bot_ext::handlers::command::CommandHandler;
use telegram_bot_ext::handlers::message::MessageHandler;
use telegram_bot_ext::filters::base::{F, All};
use std::sync::Arc;

#[derive(Clone, Hash, Eq, PartialEq)]
enum State {
    AskName,
    AskAge,
}

// Entry point: /register starts the conversation
let start_handler = ConversationStepHandler {
    handler: Box::new(CommandHandler::new(
        vec!["register".into()],
        Arc::new(|_u, _mr| Box::pin(async { HandlerResult::Continue })),
        None,
        true,
    )),
    conv_callback: Arc::new(|_update, _mr| Box::pin(async {
        (HandlerResult::Continue, ConversationResult::NextState(State::AskName))
    })),
};

// State handler: wait for the user's name
let name_handler = ConversationStepHandler {
    handler: Box::new(MessageHandler::new(Some(F::new(All)), noop_callback, true)),
    conv_callback: Arc::new(|_update, _mr| Box::pin(async {
        (HandlerResult::Continue, ConversationResult::NextState(State::AskAge))
    })),
};

// State handler: wait for the user's age
let age_handler = ConversationStepHandler {
    handler: Box::new(MessageHandler::new(Some(F::new(All)), noop_callback, true)),
    conv_callback: Arc::new(|_update, _mr| Box::pin(async {
        // End the conversation
        (HandlerResult::Continue, ConversationResult::End)
    })),
};

let conv = ConversationHandler::builder()
    .entry_point(start_handler)
    .state(State::AskName, vec![name_handler])
    .state(State::AskAge, vec![age_handler])
    .build();
```

### ConversationResult

```rust
pub enum ConversationResult<S> {
    NextState(S),  // transition to state S
    End,           // end the conversation, remove the key
    Stay,          // stay in the current state
}
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

### Conversation Keys

The state machine tracks conversations by a key. With the default settings
(`per_chat = true`, `per_user = true`), the key is `[chat_id, user_id]`. This means
each user has their own independent state within each chat.

Set `per_chat = false` if you want a single conversation state shared across all chats
for a user (rare). Set `per_message = true` when using inline keyboards where multiple
parallel conversations are identified by `callback_query.message.message_id`.

### Timeouts

When `conversation_timeout` is set, the conversation is ended automatically if no update
arrives within the duration. Timeout handlers run before the state is cleared:

```rust
ConversationHandler::builder()
    .entry_point(start_step)
    .state(State::AskName, vec![name_step])
    .timeout_handler(timeout_notify_step)   // runs on timeout
    .conversation_timeout(Duration::from_secs(120))
    .build()
```

### Nested Conversations (map_to_parent)

When a child `ConversationHandler` ends, it can optionally transition the parent
conversation to a specific state:

```rust
child_conv_builder
    .map_to_parent(HashMap::from([
        (ChildState::Done, ParentState::NextStep),
    ]))
    .build()
```

### Persistence Integration

Set `persistent(true)` and give the handler a `name`. When the `Application` has a
persistence backend, conversation states are saved and reloaded across restarts:

```rust
ConversationHandler::builder()
    .entry_point(start_step)
    .state(State::AskName, vec![name_step])
    .persistent(true)
    .name("registration".to_string())
    .build()
```

---

## InlineQueryHandler

Matches `inline_query` updates (when users type `@YourBot ...` in any chat).

```rust
use telegram_bot_ext::handlers::inline_query::InlineQueryHandler;
use std::sync::Arc;

let handler = InlineQueryHandler::new(
    Arc::new(|update, _mr| Box::pin(async move {
        // update["inline_query"]["query"] contains the search string
        HandlerResult::Continue
    })),
    true,
);
```

---

## Other Handler Types

All 21 handlers follow the same pattern: implement `Handler`, accept a callback and
optionally a filter/pattern, expose `check_update` and `handle_update`.

| Handler | Module | When it fires |
|---------|--------|---------------|
| `CommandHandler` | `handlers::command` | `/command` messages |
| `MessageHandler` | `handlers::message` | Any update passing a filter |
| `CallbackQueryHandler` | `handlers::callback_query` | Inline keyboard button press |
| `ConversationHandler` | `handlers::conversation` | Multi-step state machine |
| `InlineQueryHandler` | `handlers::inline_query` | `@bot query` inline input |
| `ChosenInlineResultHandler` | `handlers::chosen_inline_result` | User chose an inline result |
| `PollHandler` | `handlers::poll` | Poll update (new or stopped) |
| `PollAnswerHandler` | `handlers::poll_answer` | User answered a poll |
| `PreCheckoutQueryHandler` | `handlers::pre_checkout_query` | Payment pre-checkout |
| `ShippingQueryHandler` | `handlers::shipping_query` | Shipping address provided |
| `ChatMemberHandler` | `handlers::chat_member` | Chat member status change |
| `ChatJoinRequestHandler` | `handlers::chat_join_request` | Join request to a chat |
| `ChatBoostHandler` | `handlers::chat_boost` | Chat boost added or removed |
| `MessageReactionHandler` | `handlers::message_reaction` | Message reaction update |
| `BusinessConnectionHandler` | `handlers::business_connection` | Business connection event |
| `BusinessMessagesDeletedHandler` | `handlers::business_messages_deleted` | Business messages deleted |
| `PaidMediaPurchasedHandler` | `handlers::paid_media_purchased` | Paid media purchased |
| `PrefixHandler` | `handlers::prefix` | Messages starting with a given prefix |
| `StringCommandHandler` | `handlers::string_command` | Exact string command match |
| `StringRegexHandler` | `handlers::string_regex` | Regex on message text |
| `TypeHandler` | `handlers::type_handler` | Any update of a specific JSON shape |

### PrefixHandler

Matches messages that start with a specified prefix string (instead of the `/` slash).
Useful for bots that use custom command triggers.

```rust
use telegram_bot_ext::handlers::prefix::PrefixHandler;

let handler = PrefixHandler::new(
    "!",    // prefix
    vec!["ban".into(), "kick".into()],
    callback,
    None,   // HasArgs
    true,
);
// Matches: "!ban @user", "!kick @user reason"
```

### StringCommandHandler

Exact-string match on message text. Useful for bots that respond to keyboard buttons
(which send plain text, not commands).

```rust
use telegram_bot_ext::handlers::string_command::StringCommandHandler;

let handler = StringCommandHandler::new(
    vec!["Yes".into(), "No".into()],
    callback,
    true,
);
```

### StringRegexHandler

Applies a regex to the message text. Capture groups are available in `context.matches`.

```rust
use telegram_bot_ext::handlers::string_regex::StringRegexHandler;
use regex::Regex;

let handler = StringRegexHandler::new(
    Regex::new(r"order #(\d+)").unwrap(),
    callback,
    true,
);
```

### TypeHandler

Matches updates that have a specific top-level JSON key. The most general handler;
useful for update types not covered by a dedicated handler.

```rust
use telegram_bot_ext::handlers::type_handler::TypeHandler;

// Match any update with a "my_chat_member" field
let handler = TypeHandler::new("my_chat_member", callback, true);
```
