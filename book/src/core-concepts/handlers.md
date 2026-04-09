# Handlers

Handlers are the core dispatch mechanism. They determine which function runs in response to which update.

## Handler Types

The framework provides several built-in handler types:

| Handler | Use Case |
|---|---|
| `CommandHandler` | Responds to `/command` messages |
| `MessageHandler` | Responds to messages matching a filter |
| `FnHandler` | Custom predicate-based handler for any update type |
| `CallbackQueryHandler` | Responds to inline keyboard button presses |

## CommandHandler

The simplest and most common handler. Matches messages that start with a specific command:

```rust
use rust_tg_bot::ext::prelude::{CommandHandler, Arc, Context, HandlerResult, Update};

async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    context.reply_text(&update, "Welcome!").await?;
    Ok(())
}

// Register it:
app.add_handler(CommandHandler::new("start", start), 0).await;
```

`CommandHandler::new("start", start)` matches `/start` (with or without `@botusername` suffix).

## MessageHandler

Matches messages that pass a filter:

```rust
use rust_tg_bot::ext::prelude::{MessageHandler, TEXT, COMMAND};

async fn echo(update: Arc<Update>, context: Context) -> HandlerResult {
    let text = update.effective_message()
        .and_then(|m| m.text.as_deref())
        .unwrap_or("");
    context.reply_text(&update, text).await?;
    Ok(())
}

// Match text messages that are NOT commands:
app.add_handler(
    MessageHandler::new(TEXT() & !COMMAND(), echo), 0,
).await;
```

See [Filters](filters.md) for all available filters and how to combine them.

## FnHandler

The most flexible handler. You provide a predicate function that inspects the raw `Update` and decides whether to handle it:

```rust
use rust_tg_bot::ext::prelude::FnHandler;

// Match callback queries
app.add_handler(
    FnHandler::on_callback_query(button_callback), 0,
).await;

// Match inline queries
app.add_handler(
    FnHandler::on_inline_query(inline_handler), 0,
).await;

// Match shipping queries
app.add_handler(
    FnHandler::on_shipping_query(shipping_handler), 0,
).await;

// Match pre-checkout queries
app.add_handler(
    FnHandler::on_pre_checkout_query(precheckout_handler), 0,
).await;
```

### Custom Predicates

For full control, pass a predicate closure:

```rust
app.add_handler(
    FnHandler::new(
        |u| {
            // Return true if this handler should fire
            u.effective_message()
                .and_then(|m| m.successful_payment.as_ref())
                .is_some()
        },
        successful_payment_handler,
    ),
    0,
).await;
```

The predicate receives a `&Update` and returns `bool`. The handler function receives `(Arc<Update>, Context)`.

## Handler Groups

The second argument to `add_handler` is the **group number**:

```rust
app.add_handler(handler_a, 0).await;  // Group 0
app.add_handler(handler_b, 0).await;  // Group 0
app.add_handler(handler_c, 1).await;  // Group 1
```

Dispatch rules:

1. Groups are processed in ascending numeric order (0, then 1, then 2, ...).
2. Within a group, the **first** handler whose predicate/filter matches wins. Only that one handler fires.
3. After one handler fires in a group, the dispatcher moves to the next group.
4. A handler in group 1 can fire even if a handler in group 0 already fired.

This means groups let you build layered processing. For example, you might log every update in group -1 and handle commands in group 0:

```rust
// Logging handler -- fires for ALL updates
app.add_handler(
    FnHandler::new(|_| true, log_update), -1,
).await;

// Command handler -- fires only for /start
app.add_handler(
    CommandHandler::new("start", start), 0,
).await;
```

## Handler Function Signatures

All handlers follow the same signature:

```rust
async fn my_handler(update: Arc<Update>, context: Context) -> HandlerResult {
    // Your logic here
    Ok(())
}
```

To pass additional state, use closures that capture shared data:

```rust
let shared_state = Arc::new(RwLock::new(HashMap::new()));

let state = Arc::clone(&shared_state);
app.add_handler(
    FnHandler::new(
        |u| check_command(u, "save"),
        move |update, ctx| {
            let s = Arc::clone(&state);
            async move { save_handler(update, ctx, s).await }
        },
    ),
    0,
).await;
```

This pattern is covered in detail in the [Conversations](../guides/conversations.md) guide.

## Next Steps

Learn how filters control which messages reach your handlers in [Filters](filters.md).
