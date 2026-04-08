# Filters

Filters determine which updates reach a handler. They are composable, type-safe predicates that inspect an `Update` and return whether it matches.

## Built-in Filters

### TEXT

Matches any message that contains text:

```rust
use telegram_bot::ext::prelude::TEXT;

MessageHandler::new(TEXT(), my_handler)
```

### COMMAND

Matches messages that start with a bot command (`/something`):

```rust
use telegram_bot::ext::prelude::COMMAND;

MessageHandler::new(COMMAND(), my_handler)
```

### Combining Filters

Filters support bitwise operators for composition:

```rust
// Text messages that are NOT commands
TEXT() & !COMMAND()

// Text OR photo messages
// (when photo filter is available)
```

The operators:
- `&` -- AND: both filters must match.
- `|` -- OR: at least one filter must match.
- `^` -- XOR: exactly one filter must match.
- `!` -- NOT: inverts the filter.

## The F Wrapper

All filters are wrapped in the `F` type, which provides the operator overloads:

```rust
use telegram_bot::ext::prelude::F;
```

The `TEXT()` and `COMMAND()` functions return `F` values directly, so you can combine them immediately.

## Filter Results

Filters return a `FilterResult` enum:

```rust
pub enum FilterResult {
    NoMatch,
    Match,
    MatchWithData(HashMap<String, Vec<String>>),
}
```

- `NoMatch` -- the update does not match.
- `Match` -- the update matches (no additional data).
- `MatchWithData` -- the update matches and carries extracted data (e.g., regex capture groups).

## Available Filter Modules

The framework includes filters for many update properties:

| Module | What It Matches |
|---|---|
| `text` | Text content presence |
| `command` | Bot commands (`/start`, etc.) |
| `chat` | Chat type (private, group, supergroup, channel) |
| `user` | User properties |
| `document` | Document/file messages |
| `photo` | Photo messages |
| `entity` | Message entity types (mentions, URLs, etc.) |
| `forwarded` | Forwarded messages |
| `regex` | Text matching a regular expression |
| `status_update` | Chat status changes (member joined, etc.) |
| `via_bot` | Messages sent via inline bots |

## Using Filters with MessageHandler

```rust
use telegram_bot::ext::prelude::{MessageHandler, TEXT, COMMAND};

// Echo non-command text
app.add_typed_handler(
    MessageHandler::new(TEXT() & !COMMAND(), echo), 0,
).await;
```

## Using Predicates with FnHandler

When built-in filters are not enough, `FnHandler` lets you write arbitrary predicates:

```rust
use telegram_bot::ext::prelude::FnHandler;

// Match updates that have a callback query with specific data
app.add_typed_handler(
    FnHandler::new(
        |u| {
            u.callback_query()
                .and_then(|cq| cq.data.as_deref())
                == Some("my_button")
        },
        my_callback_handler,
    ),
    0,
).await;
```

## Filter Composition in Practice

Here is a real-world example that matches text messages in a specific conversation state:

```rust
fn is_text_in_state(update: &Update, conv_store: &ConvStore, state: ConvState) -> bool {
    let msg = match update.effective_message() {
        Some(m) => m,
        None => return false,
    };
    // Must have text
    if msg.text.is_none() {
        return false;
    }
    // Must NOT be a command
    let is_cmd = msg.entities.as_ref()
        .and_then(|ents| ents.first())
        .map(|e| e.entity_type == MessageEntityType::BotCommand && e.offset == 0)
        .unwrap_or(false);
    if is_cmd {
        return false;
    }
    // Must be in the expected state
    let chat_id = msg.chat.id;
    conv_store.try_read()
        .map(|guard| guard.get(&chat_id) == Some(&state))
        .unwrap_or(false)
}
```

Then use it with `FnHandler`:

```rust
let cs_check = Arc::clone(&conv_store);
app.add_typed_handler(
    FnHandler::new(
        move |u| is_text_in_state(u, &cs_check, ConvState::AskName),
        move |update, ctx| {
            let cs = Arc::clone(&cs);
            async move { receive_name(update, ctx, cs).await }
        },
    ),
    1,
).await;
```

## Next Steps

Learn about the `Context` object that handlers receive in [Context](context.md).
