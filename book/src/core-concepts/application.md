# Application

The `Application` is the central orchestrator. It owns the bot, manages handler dispatch, controls the update processing lifecycle, and coordinates persistence.

## Building an Application

Use `ApplicationBuilder` with the typestate pattern -- you cannot call `.build()` without first providing a token:

```rust
use rust_tg_bot::ext::prelude::ApplicationBuilder;

let app = ApplicationBuilder::new()
    .token("your-token")
    .build();
```

The builder returns `Arc<Application>`, which is cheap to clone and share across async tasks.

## Builder Options

| Method | Description |
|---|---|
| `.token(token)` | **Required.** The bot token from @BotFather. |
| `.concurrent_updates(n)` | Process up to `n` updates simultaneously. Default: 1. |
| `.defaults(defaults)` | Set default parse mode, timeouts, etc. |
| `.arbitrary_callback_data(maxsize)` | Enable the callback data cache. |
| `.context_types(ct)` | Custom context types configuration. |
| `.post_init(hook)` | Hook that runs after `initialize()`. |
| `.post_stop(hook)` | Hook that runs after `stop()`. |
| `.post_shutdown(hook)` | Hook that runs after `shutdown()`. |
| `.persistence(backend)` | Persistence backend (requires `persistence` feature). |
| `.job_queue(jq)` | Job queue instance (requires `job-queue` feature). |

### Example: Full Configuration

```rust
use rust_tg_bot::ext::prelude::ApplicationBuilder;
use rust_tg_bot::ext::job_queue::JobQueue;
use rust_tg_bot::ext::persistence::json_file::JsonFilePersistence;

let jq = Arc::new(JobQueue::new());
let persistence = JsonFilePersistence::new("my_bot", true, false);

let app = ApplicationBuilder::new()
    .token(token)
    .concurrent_updates(4)
    .job_queue(Arc::clone(&jq))
    .persistence(Box::new(persistence))
    .build();
```

## Registering Handlers

```rust
app.add_handler(handler, group).await;
```

- `handler` -- any handler type (`CommandHandler`, `MessageHandler`, `FnHandler`, etc.).
- `group` -- an `i32` group number. Lower numbers are checked first.

### Handler Dispatch Flow

```text
Update arrives
    |
    v
Group -1: [handler_a, handler_b]  --> first match wins in this group
    |
    v
Group 0:  [handler_c, handler_d]  --> first match wins in this group
    |
    v
Group 1:  [handler_e]             --> fires if it matches
    |
    v
Done
```

Each group is independent. One handler per group can fire.

## Error Handlers

Register global error handlers that catch any `HandlerError`:

```rust
use rust_tg_bot::ext::prelude::{Arc, CallbackContext, Update};

async fn on_error(update: Option<Arc<Update>>, ctx: CallbackContext) -> bool {
    tracing::error!("Handler error: {:?}", ctx.error);
    false // false = let other error handlers run too
}

app.add_error_handler(
    Arc::new(|update, ctx| Box::pin(on_error(update, ctx))),
    true, // block_other_handlers
).await;
```

See [Error Handling](../advanced/error-handling.md) for a complete guide.

## Lifecycle

### Automatic (Recommended)

```rust
// Handles initialize -> start -> idle -> stop -> shutdown
app.run_polling().await?;
```

### Manual

For custom server setups:

```rust
app.initialize().await?;
app.start().await?;

// Run your own server...
// When ready to stop:

app.stop().await?;
app.shutdown().await?;
```

## Accessing the Bot

```rust
let bot = app.bot();  // Returns Arc<ExtBot>
```

## Update Sender (for Webhooks)

When running a custom webhook server, use the update sender to feed updates into the application:

```rust
let tx = app.update_sender();  // Returns mpsc::Sender<RawUpdate>

// In your webhook handler:
tx.send(raw_update).await?;
```

## Feature Flags Affecting Application

| Feature | What It Enables |
|---|---|
| `job-queue` | `.job_queue()` on the builder |
| `persistence` | `.persistence()` on the builder |
| `persistence-json` | JSON file persistence backend |
| `persistence-sqlite` | SQLite persistence backend |
| `webhooks` | `run_webhook()` method |
| `rate-limiter` | Rate limiting on API calls |

## Next Steps

You now understand all the core concepts. Move on to the practical guides:

- [Command Bots](../guides/command-bots.md) -- handle commands with arguments.
- [Inline Keyboards](../guides/inline-keyboards.md) -- interactive button menus.
- [Conversations](../guides/conversations.md) -- multi-step interactions.
