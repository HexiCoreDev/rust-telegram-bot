# Zero to Hero: Building Telegram Bots in Rust

**A comprehensive guide to the `rust-telegram-bot` framework**

*Author: Jude Etuka*
*Repository: [https://github.com/HexiCoreDev/rust-telegram-bot](https://github.com/HexiCoreDev/rust-telegram-bot)*

---

## Table of Contents

- [Part 1: Getting Started](#part-1-getting-started)
  - [What is a Telegram Bot?](#what-is-a-telegram-bot)
  - [Creating a Bot with BotFather](#creating-a-bot-with-botfather)
  - [Setting Up Your Rust Project](#setting-up-your-rust-project)
  - [Your First Echo Bot](#your-first-echo-bot)
  - [Running and Testing It](#running-and-testing-it)
  - [Troubleshooting Common Issues](#troubleshooting-common-issues)
- [Part 2: Core Concepts](#part-2-core-concepts)
  - [The Update Object](#the-update-object)
  - [The Bot](#the-bot)
  - [Handlers](#handlers)
  - [Filters](#filters)
  - [Context](#context)
  - [The Application Lifecycle](#the-application-lifecycle)
- [Part 3: Building Real Bots](#part-3-building-real-bots)
  - [Command Bot with Argument Parsing](#command-bot-with-argument-parsing)
  - [Interactive Bot with Inline Keyboards](#interactive-bot-with-inline-keyboards)
  - [Conversation Bot with Multi-Step State Machine](#conversation-bot-with-multi-step-state-machine)
  - [Timer/Scheduler Bot with JobQueue](#timerscheduler-bot-with-jobqueue)
  - [Webhook Bot for Production Deployment](#webhook-bot-for-production-deployment)
  - [Inline Mode Bot](#inline-mode-bot)
- [Part 4: Advanced Topics](#part-4-advanced-topics)
  - [Persistence](#persistence)
  - [Error Handling](#error-handling)
  - [Group and Channel Bots](#group-and-channel-bots)
  - [Payments and Invoices](#payments-and-invoices)
  - [Custom Filters](#custom-filters)
  - [Nested Conversations](#nested-conversations)
  - [Typed Data Access](#typed-data-access)
- [Part 5: Production Deployment](#part-5-production-deployment)
  - [Release Build](#release-build)
  - [Docker Deployment](#docker-deployment)
  - [Webhook vs Polling](#webhook-vs-polling)
  - [Environment Variables and Secrets](#environment-variables-and-secrets)
  - [Monitoring with Tracing](#monitoring-with-tracing)
  - [Security Best Practices](#security-best-practices)

---

## Part 1: Getting Started

### What is a Telegram Bot?

A Telegram Bot is a special account operated by software rather than a person. Bots can receive messages, respond to commands, send media, process payments, and participate in groups and channels. Telegram provides a comprehensive [Bot API](https://core.telegram.org/bots/api) that your code communicates with over HTTPS. Every interaction is stateless from Telegram's perspective: your bot receives an "update" (a JSON payload describing what happened), processes it, and optionally calls the Bot API to respond.

`rust-telegram-bot` is a Rust framework that provides the same developer experience as Python's `python-telegram-bot` (PTB) library, but with Rust's performance characteristics: a stripped release binary of around 10MB, runtime memory usage of 15-27MB RSS, and zero-cost abstractions over the async Bot API. If you have used PTB before, you will find the handler/filter/context architecture immediately familiar. If Rust is new to you, this guide will walk you through every step.

### Creating a Bot with BotFather

Before writing any code, you need a bot token from Telegram. Open Telegram and search for `@BotFather`.

1. Start a conversation with `@BotFather` by sending `/start`.
2. Send `/newbot` to begin bot creation.
3. Choose a **display name** for your bot (e.g., "My First Rust Bot").
4. Choose a **username** that ends in `bot` (e.g., `my_first_rust_bot`). This must be globally unique.
5. BotFather replies with an HTTP API token that looks like `123456789:ABCdefGHIjklMNO-pqrSTUvwxYZ`. Copy this token.
6. **Keep this token secret.** Anyone who has it can control your bot.

Optional but recommended:

- Send `/setdescription` to BotFather to set what users see before they start your bot.
- Send `/setcommands` to register your bot's commands so they appear in the Telegram command menu.
- Send `/setprivacy` and set it to "Disable" if your bot needs to read all messages in groups (not just commands).

### Setting Up Your Rust Project

You need Rust 1.75 or later (for `async fn` in trait support). If you do not have Rust installed, visit [https://rustup.rs](https://rustup.rs).

Create a new project and add the dependency:

```sh
cargo new my_bot
cd my_bot
```

Add `rust-telegram-bot` to your `Cargo.toml`. Since the crate lives in the repository as a workspace, you can depend on it via git:

```toml
[dependencies]
telegram-bot = { git = "https://github.com/HexiCoreDev/rust-telegram-bot", package = "telegram-bot" }
tracing-subscriber = "0.3"
```

If you need persistence features, add the appropriate feature flags:

```toml
[dependencies]
telegram-bot = { git = "https://github.com/HexiCoreDev/rust-telegram-bot", package = "telegram-bot", features = ["persistence-json"] }
```

Available features:
- `persistence-json` -- JSON file persistence
- `persistence-sqlite` -- SQLite persistence
- `webhooks` -- Webhook support via axum

### Your First Echo Bot

Create `src/main.rs` with the following code. This bot responds to `/start` and `/help` commands and echoes back any text message.

```rust
use telegram_bot::ext::prelude::*;

// ---------------------------------------------------------------------------
// Handler functions
// ---------------------------------------------------------------------------

/// Respond to `/start` with a greeting.
async fn start(update: Update, context: Context) -> HandlerResult {
    let name = update
        .effective_user()
        .map(|u| u.first_name.as_str())
        .unwrap_or("there");

    context
        .reply_text(
            &update,
            &format!(
                "Hi {name}! I am an echo bot. Send me any message and \
                 I will repeat it back to you.\n\n\
                 Use /help to see available commands."
            ),
        )
        .await?;

    Ok(())
}

/// Respond to `/help` with usage instructions.
async fn help(update: Update, context: Context) -> HandlerResult {
    context
        .reply_text(
            &update,
            "Available commands:\n\
             /start - Start the bot\n\
             /help - Show this help message\n\n\
             Send any text message and I will echo it back!",
        )
        .await?;

    Ok(())
}

/// Echo back whatever text the user sends.
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

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    telegram_bot::run(async {
        tracing_subscriber::fmt::init();

        let token = std::env::var("TELEGRAM_BOT_TOKEN")
            .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

        let app = ApplicationBuilder::new().token(token).build();

        app.add_typed_handler(CommandHandler::new("start", start), 0).await;
        app.add_typed_handler(CommandHandler::new("help", help), 0).await;
        app.add_typed_handler(MessageHandler::new(TEXT() & !COMMAND(), echo), 0).await;

        println!("Echo bot is running. Press Ctrl+C to stop.");

        if let Err(e) = app.run_polling().await {
            eprintln!("Error running bot: {e}");
        }
    });
}
```

Let us walk through this line by line.

**The import:**

```rust
use telegram_bot::ext::prelude::*;
```

This single import brings in everything you need: `Update`, `Context`, `HandlerResult`, `ApplicationBuilder`, `CommandHandler`, `MessageHandler`, the `TEXT()` and `COMMAND()` filter functions, and all typed constants like `ParseMode::Html`.

**Handler functions:**

Every handler has the same signature:

```rust
async fn handler(update: Update, context: Context) -> HandlerResult
```

- `update` is the Telegram update (a message, callback query, inline query, etc.) with typed accessors.
- `context` provides the bot instance, shared data stores, and convenience methods.
- `HandlerResult` is `Result<(), HandlerError>`. Return `Ok(())` on success.

In the `start` handler:
- `update.effective_user()` returns `Option<&User>`, giving typed access to the sender's name and ID.
- `context.reply_text(&update, text)` is a convenience that extracts the chat ID from the update and calls `send_message`.

In the `echo` handler:
- `update.effective_message()` returns the `Message` struct with typed fields like `text`, `chat`, `entities`.

**The main function:**

```rust
fn main() {
    telegram_bot::run(async {
        // ...
    });
}
```

`telegram_bot::run` creates a multi-threaded tokio runtime with 8MB thread stacks. The increased stack size handles the deeply nested async state machines produced by the Bot API call chain. You do not need to annotate `main` with `#[tokio::main]`.

**Building the application:**

```rust
let app = ApplicationBuilder::new().token(token).build();
```

`ApplicationBuilder` uses a typestate pattern: you cannot call `.build()` until you have provided a `.token()`. The result is an `Arc<Application>` that can be shared across threads.

**Registering handlers:**

```rust
app.add_typed_handler(CommandHandler::new("start", start), 0).await;
app.add_typed_handler(CommandHandler::new("help", help), 0).await;
app.add_typed_handler(MessageHandler::new(TEXT() & !COMMAND(), echo), 0).await;
```

- `CommandHandler::new("start", start)` matches messages beginning with `/start`.
- `MessageHandler::new(TEXT() & !COMMAND(), echo)` matches text messages that are not commands. The `&` and `!` operators compose filters naturally.
- The second argument (`0`) is the handler group. All three handlers are in group 0, processed in registration order. Within a group, the first matching handler wins.

**Polling:**

```rust
app.run_polling().await.unwrap();
```

This calls `initialize()`, `start()`, begins long-polling the Telegram servers for updates, and blocks until Ctrl+C is received. On shutdown it calls `stop()` and `shutdown()` automatically.

### Running and Testing It

Set the environment variable and run:

```sh
export TELEGRAM_BOT_TOKEN="your-token-here"
cargo run
```

Open Telegram, find your bot by its username, and send `/start`. The bot should greet you by name. Send any text message and it echoes it back.

For faster iteration during development, use `cargo watch`:

```sh
cargo install cargo-watch
cargo watch -x run
```

**Try it:** Modify the `start` handler to include the user's ID in the greeting. Access it via `update.effective_user().map(|u| u.id)`.

### Troubleshooting Common Issues

**"TELEGRAM_BOT_TOKEN environment variable must be set"**

You forgot to export the token. On Linux/macOS: `export TELEGRAM_BOT_TOKEN="..."`. On Windows: `set TELEGRAM_BOT_TOKEN=...`.

**The bot does not respond to messages**

- Make sure you are messaging the bot in a private chat, not a group.
- Check that the token is correct (no extra spaces or newlines).
- If the bot was previously running with a webhook, Telegram will not send updates via polling. Call the `deleteWebhook` API first: `curl "https://api.telegram.org/bot<TOKEN>/deleteWebhook"`.

**Compilation error: "trait bound not satisfied"**

Ensure your handler function signature is exactly `async fn(Update, Context) -> HandlerResult`. The types come from the prelude; do not import `Update` from a different path.

**"thread has overflowed its stack"**

Use `telegram_bot::run(async { ... })` instead of `#[tokio::main]`. The framework creates a runtime with 8MB thread stacks specifically to handle this.

**Slow compile times**

The first build downloads and compiles all dependencies. Subsequent builds are incremental and much faster. For development, `cargo run` (debug mode) is fine. For deployment, always use `cargo build --release`.

---

## Part 2: Core Concepts

### The Update Object

Every interaction with your bot begins with an `Update`. When a user sends a message, taps an inline button, or submits a payment, Telegram wraps the event in an Update and sends it to your bot. The `Update` struct provides typed access to every field in the [Telegram Bot API Update object](https://core.telegram.org/bots/api#update).

Key fields accessed directly:

| Field | Type | Description |
|-------|------|-------------|
| `update.message` | `Option<Message>` | A new incoming message |
| `update.edited_message` | `Option<Message>` | An edited message |
| `update.callback_query` | `Option<CallbackQuery>` | Button press on inline keyboard |
| `update.inline_query` | `Option<InlineQuery>` | User types `@botname query` |
| `update.shipping_query` | `Option<ShippingQuery>` | Payment shipping step |
| `update.pre_checkout_query` | `Option<PreCheckoutQuery>` | Payment confirmation step |
| `update.poll` | `Option<Poll>` | Poll state change |
| `update.poll_answer` | `Option<PollAnswer>` | User answered a poll |
| `update.chat_member` | `Option<ChatMemberUpdated>` | Member status change |
| `update.my_chat_member` | `Option<ChatMemberUpdated>` | Bot's own status change |

Convenience accessors that search across all update types:

```rust
// Get the user who triggered this update (works for messages, callbacks, inline queries, etc.)
let user: Option<&User> = update.effective_user();
let name: &str = user.map(|u| u.first_name.as_str()).unwrap_or("unknown");
let user_id: i64 = user.map(|u| u.id).unwrap_or(0);

// Get the chat this update belongs to
let chat: Option<&Chat> = update.effective_chat();
let chat_id: i64 = chat.map(|c| c.id).unwrap_or(0);

// Get the message (works for message, edited_message, channel_post, etc.)
let message: Option<&Message> = update.effective_message();
let text: Option<&str> = message.and_then(|m| m.text.as_deref());
```

A `Message` has many fields: `text`, `chat` (with `chat.id`), `from` (the sender), `entities` (bold, links, commands), `photo`, `document`, `location`, `successful_payment`, and dozens more matching the full Telegram API.

**Try it:** Print `{update:?}` inside a handler to see the full debug representation of an update. Notice how different interaction types populate different fields.

### The Bot

The `Bot` (accessed via `context.bot()`) is your gateway to the Telegram Bot API. Every API method is available as a builder:

```rust
// Simple message
context.bot().send_message(chat_id, "Hello!").send().await?;

// Message with formatting and reply keyboard
context
    .bot()
    .send_message(chat_id, "<b>Bold text</b>")
    .parse_mode(ParseMode::Html)
    .reply_markup(keyboard_json)
    .send()
    .await?;

// Edit an existing message
context
    .bot()
    .edit_message_text("Updated text")
    .chat_id(chat_id)
    .message_id(msg_id)
    .send()
    .await?;

// Answer a callback query (dismiss the loading spinner)
context
    .bot()
    .answer_callback_query(&callback_query_id)
    .send()
    .await?;

// Answer an inline query with results
context
    .bot()
    .answer_inline_query(&inline_query_id, results_vec)
    .send()
    .await?;

// Send an invoice
context
    .bot()
    .send_invoice(chat_id, "Title", "Description", "payload", "USD", prices)
    .provider_token(&token)
    .send()
    .await?;
```

Every builder follows the same pattern: call the method, chain optional parameters, and finish with `.send().await?`.

The convenience method `context.reply_text(&update, text)` is a shortcut for extracting the chat ID from an update and calling `send_message`. Use it when you just want to reply with plain text. Use `context.bot()` directly when you need formatting, keyboards, or any other option.

**Typed constants** are available for common values:

```rust
ParseMode::Html
ParseMode::MarkdownV2
ChatType::Private
ChatType::Group
ChatType::Supergroup
ChatType::Channel
MessageEntityType::BotCommand
MessageEntityType::Bold
MessageEntityType::Url
ChatAction::Typing
ChatMemberStatus::Administrator
```

**Try it:** Send a message with `ParseMode::Html` and use `<b>`, `<i>`, `<code>`, and `<a href="...">` tags.

### Handlers

Handlers are the routing layer. They decide which updates your code processes and how. The framework provides several handler types:

**CommandHandler** -- matches bot commands (`/command`):

```rust
// Matches /start (case-insensitive, handles @botname suffix)
app.add_typed_handler(CommandHandler::new("start", start_fn), 0).await;
```

When a command has arguments (e.g., `/set 30`), they are parsed and available via `context.args`:

```rust
async fn set(update: Update, context: Context) -> HandlerResult {
    if let Some(args) = &context.args {
        let seconds: u64 = args[0].parse().unwrap_or(60);
        // use seconds...
    }
    Ok(())
}
```

**MessageHandler** -- matches updates that pass a composable filter:

```rust
// Match text messages that are not commands
app.add_typed_handler(
    MessageHandler::new(TEXT() & !COMMAND(), echo_fn),
    0,
).await;
```

**FnHandler** -- the most flexible handler, matches by predicate:

```rust
// Match callback queries (button presses)
app.add_typed_handler(
    FnHandler::on_callback_query(button_fn),
    0,
).await;

// Match inline queries
app.add_typed_handler(
    FnHandler::on_inline_query(inline_fn),
    0,
).await;

// Match with a custom predicate
app.add_typed_handler(
    FnHandler::new(|u| u.message.is_some(), any_message_fn),
    0,
).await;

// Catch-all handler
app.add_typed_handler(FnHandler::on_any(fallback_fn), 1).await;
```

`FnHandler` provides convenience constructors for common update types:

| Constructor | Matches |
|-------------|---------|
| `FnHandler::on_callback_query(f)` | Callback query updates |
| `FnHandler::on_inline_query(f)` | Inline query updates |
| `FnHandler::on_message(f)` | Message updates |
| `FnHandler::on_poll(f)` | Poll updates |
| `FnHandler::on_poll_answer(f)` | Poll answer updates |
| `FnHandler::on_shipping_query(f)` | Shipping query updates |
| `FnHandler::on_pre_checkout_query(f)` | Pre-checkout query updates |
| `FnHandler::on_chat_member(f)` | Chat member updates |
| `FnHandler::on_my_chat_member(f)` | Bot's own member status updates |
| `FnHandler::on_any(f)` | Every update (catch-all) |

**Handler groups:**

The second argument to `add_typed_handler` is the group number. Groups are processed in ascending numeric order. Within each group, the first handler whose predicate matches wins -- only one handler per group fires.

```rust
// Group 0: commands
app.add_typed_handler(CommandHandler::new("start", start_fn), 0).await;

// Group 1: conversation-state handlers (checked after commands)
app.add_typed_handler(FnHandler::new(pred, state_fn), 1).await;

// Group 2: fallback
app.add_typed_handler(FnHandler::on_any(fallback_fn), 2).await;
```

**Try it:** Register two handlers in the same group that both match text messages. Observe that only the first one fires. Then move the second to group 1 and observe that both fire.

### Filters

Filters are composable predicates used with `MessageHandler`. The framework provides built-in filter functions that return the `F` wrapper type, which supports bitwise operators for composition:

```rust
// Built-in filters
TEXT()       // matches any message with text
COMMAND()    // matches bot commands (/start, /help, etc.)

// Composition operators
TEXT() & !COMMAND()      // text that is NOT a command (AND + NOT)
TEXT() | COMMAND()       // text OR command (OR)
TEXT() ^ COMMAND()       // text XOR command (exclusive or)
!TEXT()                  // NOT text (negation)
```

You can create custom filters by implementing the `Filter` trait:

```rust
use telegram_bot::ext::prelude::*;

struct PhotoFilter;

impl Filter for PhotoFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        if update.effective_message().and_then(|m| m.photo.as_ref()).is_some() {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "PhotoFilter"
    }
}

// Use it
let photo_handler = MessageHandler::new(F::new(PhotoFilter), handle_photo);
```

For quick one-off predicates, `FnHandler::new(predicate, callback)` is often more convenient than creating a custom filter.

**Try it:** Create a filter that matches messages containing a URL entity. Compose it with `TEXT()` using the `&` operator.

### Context

The `Context` (aliased from `CallbackContext`) is your second essential parameter. It provides:

**The Bot instance:**

```rust
context.bot()  // -> &Arc<ExtBot>
```

**Quick replies:**

```rust
context.reply_text(&update, "Hello!").await?;
```

**Bot-wide data** (shared across all handlers, all users):

```rust
// Read
let guard = context.bot_data().await;         // DataReadGuard
let name = guard.get_str("admin_name");       // Option<&str>
let count = guard.get_i64("request_count");   // Option<i64>
let active = guard.get_bool("maintenance");   // Option<bool>
let ids = guard.get_id_set("admin_ids");      // HashSet<i64>

// Write
let mut guard = context.bot_data_mut().await; // DataWriteGuard
guard.set_str("admin_name", "Alice");
guard.set_i64("request_count", 42);
guard.set_bool("maintenance", false);
guard.add_to_id_set("admin_ids", 12345);
guard.remove_from_id_set("admin_ids", 12345);
```

**Per-user and per-chat data:**

```rust
// Read snapshots (returns Option<HashMap<String, Value>>)
let user_data = context.user_data().await;
let chat_data = context.chat_data().await;

// Write individual keys
context.set_user_data("score".to_string(), serde_json::json!(100)).await;
context.set_chat_data("topic".to_string(), serde_json::json!("Rust")).await;
```

**Command arguments** (populated by `CommandHandler`):

```rust
async fn set_timer(update: Update, context: Context) -> HandlerResult {
    if let Some(args) = &context.args {
        // /set 30 -> args = ["30"]
        let seconds: u64 = args.first()
            .and_then(|s| s.parse().ok())
            .unwrap_or(60);
    }
    Ok(())
}
```

**Regex matches** (populated by regex-based handlers):

```rust
// context.matches -> Option<Vec<String>>   (positional captures)
// context.named_matches -> Option<HashMap<String, String>> (named captures)
```

**Error info** (in error handlers only):

```rust
// context.error -> Option<Arc<dyn Error + Send + Sync>>
```

**Job queue:**

```rust
// context.job_queue -> Option<Arc<JobQueue>>
```

**Try it:** Store a counter in `bot_data` that increments on every message. Display it when the user sends `/count`.

### The Application Lifecycle

The `Application` follows a strict lifecycle:

```
initialize  ->  start  ->  idle (run_polling / run_webhook)  ->  stop  ->  shutdown
```

**`initialize`**: Initializes the bot (fetches bot info from Telegram), initializes the update processor, loads data from persistence (if configured), and starts the job queue.

**`start`**: Marks the application as running and begins the update dispatch loop.

**`idle`**: The application listens for updates via polling or webhook and dispatches them to handlers. This is where your bot spends most of its time.

**`stop`**: Stops the update dispatch loop, waits for pending tasks to complete, and flushes persistence.

**`shutdown`**: Flushes persistence, shuts down the bot and update processor, and marks the application as uninitialized.

For most bots, `run_polling()` handles the entire lifecycle automatically:

```rust
app.run_polling().await?;
// Equivalent to: initialize -> start -> poll until Ctrl+C -> stop -> shutdown
```

For webhook bots or advanced use cases, you can control each phase manually:

```rust
app.initialize().await?;
app.start().await?;

// Run your custom server here...

app.stop().await?;
app.shutdown().await?;
```

Lifecycle hooks let you run custom code at specific points:

```rust
let hook: LifecycleHook = Arc::new(|_app| Box::pin(async {
    println!("Application initialized!");
}));

let app = ApplicationBuilder::new()
    .token(token)
    .post_init(hook)
    .build();
```

**Try it:** Add a `post_init` hook that logs the bot's username. Access it via `app.bot().get_me().await`.

---

## Part 3: Building Real Bots

### Command Bot with Argument Parsing

This bot demonstrates command handling with arguments. It implements `/greet <name>`, `/add <a> <b>`, and `/uppercase <text>`.

```rust
use telegram_bot::ext::prelude::*;

async fn start(update: Update, context: Context) -> HandlerResult {
    context
        .reply_text(
            &update,
            "Command bot! Try:\n\
             /greet Alice\n\
             /add 3 5\n\
             /uppercase hello world",
        )
        .await?;
    Ok(())
}

async fn greet(update: Update, context: Context) -> HandlerResult {
    let name = context
        .args
        .as_ref()
        .and_then(|a| a.first())
        .map(String::as_str)
        .unwrap_or("stranger");

    context
        .reply_text(&update, &format!("Hello, {name}!"))
        .await?;
    Ok(())
}

async fn add(update: Update, context: Context) -> HandlerResult {
    let args = context.args.as_ref();
    let a: f64 = args
        .and_then(|a| a.get(0))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);
    let b: f64 = args
        .and_then(|a| a.get(1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);

    context
        .reply_text(&update, &format!("{a} + {b} = {}", a + b))
        .await?;
    Ok(())
}

async fn uppercase(update: Update, context: Context) -> HandlerResult {
    let text = context
        .args
        .as_ref()
        .map(|a| a.join(" "))
        .unwrap_or_default();

    context
        .reply_text(&update, &text.to_uppercase())
        .await?;
    Ok(())
}

fn main() {
    telegram_bot::run(async {
        tracing_subscriber::fmt::init();

        let token = std::env::var("TELEGRAM_BOT_TOKEN")
            .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

        let app = ApplicationBuilder::new().token(token).build();

        app.add_typed_handler(CommandHandler::new("start", start), 0).await;
        app.add_typed_handler(CommandHandler::new("greet", greet), 0).await;
        app.add_typed_handler(CommandHandler::new("add", add), 0).await;
        app.add_typed_handler(CommandHandler::new("uppercase", uppercase), 0).await;

        println!("Command bot running. Press Ctrl+C to stop.");
        app.run_polling().await.unwrap();
    });
}
```

When a user sends `/add 10 20`, the `CommandHandler` parses the text after the command into `context.args = Some(vec!["10", "20"])`. The handler then parses these strings into numbers.

**Try it:** Add a `/roll <sides>` command that generates a random number between 1 and `sides` using `rand::thread_rng()`.

### Interactive Bot with Inline Keyboards

Inline keyboards let you add buttons directly below messages. When a user presses a button, your bot receives a callback query.

```rust
use serde_json::json;
use telegram_bot::ext::prelude::*;

fn build_keyboard() -> serde_json::Value {
    json!({
        "inline_keyboard": [
            [
                {"text": "Option 1", "callback_data": "1"},
                {"text": "Option 2", "callback_data": "2"},
            ],
            [
                {"text": "Option 3", "callback_data": "3"},
            ],
        ]
    })
}

async fn start(update: Update, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().map(|c| c.id).unwrap_or(0);
    let keyboard = build_keyboard();

    context
        .bot()
        .send_message(chat_id, "Please choose an option:")
        .reply_markup(keyboard)
        .send()
        .await?;

    Ok(())
}

async fn button_callback(update: Update, context: Context) -> HandlerResult {
    let cq = update
        .callback_query
        .as_ref()
        .expect("callback query handler requires callback_query");

    let data = cq.data.as_deref().unwrap_or("unknown");

    // Dismiss the loading spinner on the client
    context
        .bot()
        .answer_callback_query(&cq.id)
        .send()
        .await?;

    // Edit the original message to show the selection
    if let Some(ref msg) = cq.message {
        context
            .bot()
            .edit_message_text(&format!("You selected: Option {data}"))
            .chat_id(msg.chat().id)
            .message_id(msg.message_id())
            .send()
            .await?;
    }

    Ok(())
}

fn main() {
    telegram_bot::run(async {
        tracing_subscriber::fmt::init();

        let token = std::env::var("TELEGRAM_BOT_TOKEN")
            .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

        let app = ApplicationBuilder::new().token(token).build();

        app.add_typed_handler(CommandHandler::new("start", start), 0).await;
        app.add_typed_handler(FnHandler::on_callback_query(button_callback), 0).await;

        println!("Inline keyboard bot running. Press Ctrl+C to stop.");
        app.run_polling().await.unwrap();
    });
}
```

Key concepts:

- The keyboard is built as a JSON value matching the Telegram `InlineKeyboardMarkup` schema.
- `FnHandler::on_callback_query(f)` matches only callback query updates.
- Always call `answer_callback_query` to dismiss the loading indicator. Failing to do so leaves a spinning icon on the button.
- Use `edit_message_text` to modify the message the button is attached to.
- `cq.message` gives you the message the button belongs to, from which you can extract `chat().id` and `message_id()`.

**Try it:** Add a "Refresh" button that regenerates the keyboard. After the user selects an option, edit the message to show their choice and a single "Back" button that restores the original keyboard.

### Conversation Bot with Multi-Step State Machine

Conversations collect information over multiple steps. The bot asks a question, waits for the answer, transitions to the next state, and repeats until done.

This pattern uses a shared `HashMap` protected by a `RwLock` to track which conversation state each chat is in. Different handler groups check the current state before processing.

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use telegram_bot::ext::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConvState {
    AskName,
    AskAge,
    AskBio,
}

type ConvStore = Arc<RwLock<HashMap<i64, ConvState>>>;

/// Check if the update is a non-command text message and the chat is in the given state.
fn is_text_in_state(update: &Update, conv_store: &ConvStore, state: ConvState) -> bool {
    let msg = match update.effective_message() {
        Some(m) => m,
        None => return false,
    };
    if msg.text.is_none() {
        return false;
    }
    // Reject commands
    let is_cmd = msg
        .entities
        .as_ref()
        .and_then(|ents| ents.first())
        .map(|e| e.entity_type == MessageEntityType::BotCommand && e.offset == 0)
        .unwrap_or(false);
    if is_cmd {
        return false;
    }
    let chat_id = msg.chat.id;
    conv_store
        .try_read()
        .map(|guard| guard.get(&chat_id) == Some(&state))
        .unwrap_or(false)
}

async fn start_conv(update: Update, context: Context, cs: ConvStore) -> HandlerResult {
    let chat_id = update.effective_chat().unwrap().id;
    cs.write().await.insert(chat_id, ConvState::AskName);
    context
        .bot()
        .send_message(chat_id, "What is your name? (Send /cancel to stop.)")
        .send()
        .await?;
    Ok(())
}

async fn receive_name(update: Update, context: Context, cs: ConvStore) -> HandlerResult {
    let chat_id = update.effective_chat().unwrap().id;
    let name = update.effective_message().and_then(|m| m.text.as_deref()).unwrap_or("");
    cs.write().await.insert(chat_id, ConvState::AskAge);
    context
        .bot()
        .send_message(chat_id, &format!("Nice to meet you, {name}! How old are you?"))
        .send()
        .await?;
    Ok(())
}

async fn receive_age(update: Update, context: Context, cs: ConvStore) -> HandlerResult {
    let chat_id = update.effective_chat().unwrap().id;
    cs.write().await.insert(chat_id, ConvState::AskBio);
    context
        .bot()
        .send_message(chat_id, "Tell me a little about yourself.")
        .send()
        .await?;
    Ok(())
}

async fn receive_bio(update: Update, context: Context, cs: ConvStore) -> HandlerResult {
    let chat_id = update.effective_chat().unwrap().id;
    let bio = update.effective_message().and_then(|m| m.text.as_deref()).unwrap_or("");
    cs.write().await.remove(&chat_id);
    context
        .bot()
        .send_message(chat_id, &format!("Profile complete! Bio: {bio}"))
        .send()
        .await?;
    Ok(())
}

async fn cancel(update: Update, context: Context, cs: ConvStore) -> HandlerResult {
    let chat_id = update.effective_chat().unwrap().id;
    cs.write().await.remove(&chat_id);
    context
        .bot()
        .send_message(chat_id, "Conversation cancelled. Send /start to begin again.")
        .send()
        .await?;
    Ok(())
}

fn check_command(update: &Update, expected: &str) -> bool {
    let msg = match update.effective_message() {
        Some(m) => m,
        None => return false,
    };
    let text = match msg.text.as_deref() {
        Some(t) => t,
        None => return false,
    };
    let entities = match msg.entities.as_ref() {
        Some(e) => e,
        None => return false,
    };
    entities.first().map_or(false, |e| {
        if e.entity_type == MessageEntityType::BotCommand && e.offset == 0 {
            let length = e.length as usize;
            if length <= text.len() {
                text[1..length].split('@').next().unwrap_or("")
                    .eq_ignore_ascii_case(expected)
            } else {
                false
            }
        } else {
            false
        }
    })
}

fn main() {
    telegram_bot::run(async {
        tracing_subscriber::fmt::init();
        let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
        let app = ApplicationBuilder::new().token(token).build();
        let cs: ConvStore = Arc::new(RwLock::new(HashMap::new()));

        // Group 0: entry point and cancel (always checked first)
        {
            let cs = Arc::clone(&cs);
            app.add_typed_handler(
                FnHandler::new(|u| check_command(u, "start"), move |u, c| {
                    let cs = Arc::clone(&cs);
                    async move { start_conv(u, c, cs).await }
                }),
                0,
            ).await;
        }
        {
            let cs = Arc::clone(&cs);
            app.add_typed_handler(
                FnHandler::new(|u| check_command(u, "cancel"), move |u, c| {
                    let cs = Arc::clone(&cs);
                    async move { cancel(u, c, cs).await }
                }),
                0,
            ).await;
        }

        // Group 1: state-specific handlers
        {
            let cs = Arc::clone(&cs);
            let cs_check = Arc::clone(&cs);
            app.add_typed_handler(
                FnHandler::new(
                    move |u| is_text_in_state(u, &cs_check, ConvState::AskName),
                    move |u, c| { let cs = Arc::clone(&cs); async move { receive_name(u, c, cs).await } },
                ),
                1,
            ).await;
        }
        {
            let cs = Arc::clone(&cs);
            let cs_check = Arc::clone(&cs);
            app.add_typed_handler(
                FnHandler::new(
                    move |u| is_text_in_state(u, &cs_check, ConvState::AskAge),
                    move |u, c| { let cs = Arc::clone(&cs); async move { receive_age(u, c, cs).await } },
                ),
                1,
            ).await;
        }
        {
            let cs = Arc::clone(&cs);
            let cs_check = Arc::clone(&cs);
            app.add_typed_handler(
                FnHandler::new(
                    move |u| is_text_in_state(u, &cs_check, ConvState::AskBio),
                    move |u, c| { let cs = Arc::clone(&cs); async move { receive_bio(u, c, cs).await } },
                ),
                1,
            ).await;
        }

        println!("Conversation bot running. Commands: /start, /cancel");
        app.run_polling().await.unwrap();
    });
}
```

Key patterns:

- **State store**: A `RwLock<HashMap<i64, ConvState>>` maps each chat ID to its current conversation state.
- **Handler groups**: Commands (group 0) are checked before state handlers (group 1). This ensures `/cancel` always works regardless of state.
- **`try_read()`**: The predicate in `FnHandler::new` uses `try_read()` instead of `.read().await` because predicates are synchronous. `try_read()` returns immediately, failing only under extreme lock contention.
- **`Arc::clone` dance**: Each handler closure captures its own `Arc` clone of the shared state. This is idiomatic Rust for sharing data across async closures.

**Try it:** Add a `/skip` command that advances to the next state without storing the current answer.

### Timer/Scheduler Bot with JobQueue

The `JobQueue` lets you schedule one-shot or repeating tasks. This bot implements `/set <seconds>` and `/unset`.

```rust
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use telegram_bot::ext::prelude::*;
use telegram_bot::ext::job_queue::{JobCallbackFn, JobContext, JobQueue};

type TimerStore = Arc<RwLock<std::collections::HashMap<i64, u64>>>;

async fn set_timer(
    update: Update,
    context: Context,
    timer_store: TimerStore,
) -> HandlerResult {
    let chat_id = update.effective_chat().unwrap().id;
    let text = update.effective_message().and_then(|m| m.text.as_deref()).unwrap_or("");
    let args: Vec<&str> = text.split_whitespace().skip(1).collect();

    let seconds: u64 = match args.first().and_then(|s| s.parse().ok()) {
        Some(n) if n > 0 => n,
        _ => {
            context.bot().send_message(chat_id, "Usage: /set <seconds>").send().await?;
            return Ok(());
        }
    };

    let bot = Arc::clone(context.bot());
    let alarm_callback: JobCallbackFn = Arc::new(move |ctx: JobContext| {
        let bot = Arc::clone(&bot);
        Box::pin(async move {
            let target = ctx.chat_id.unwrap_or(0);
            if target != 0 {
                bot.send_message(target, "BEEP! Timer is done!")
                    .send()
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            }
            Ok(())
        })
    });

    let jq = context.job_queue.as_ref().unwrap();

    // Cancel existing timer for this chat
    {
        let store = timer_store.read().await;
        if let Some(&old_id) = store.get(&chat_id) {
            for job in jq.jobs().await {
                if job.id == old_id {
                    job.schedule_removal();
                    break;
                }
            }
        }
    }

    let job = jq
        .once(alarm_callback, Duration::from_secs(seconds))
        .name(format!("timer_{chat_id}"))
        .chat_id(chat_id)
        .start()
        .await;

    timer_store.write().await.insert(chat_id, job.id);

    context
        .bot()
        .send_message(chat_id, &format!("Timer set for {seconds} seconds!"))
        .send()
        .await?;

    Ok(())
}

fn main() {
    telegram_bot::run(async {
        tracing_subscriber::fmt::init();
        let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();

        let jq = Arc::new(JobQueue::new());

        let app = ApplicationBuilder::new()
            .token(token)
            .job_queue(Arc::clone(&jq))
            .build();

        let timer_store: TimerStore = Arc::new(RwLock::new(std::collections::HashMap::new()));

        // Register /set handler
        {
            let store = Arc::clone(&timer_store);
            app.add_typed_handler(
                FnHandler::new(
                    |u| {
                        u.effective_message()
                            .and_then(|m| m.text.as_deref())
                            .map(|t| t.starts_with("/set"))
                            .unwrap_or(false)
                    },
                    move |update, ctx| {
                        let s = Arc::clone(&store);
                        async move { set_timer(update, ctx, s).await }
                    },
                ),
                0,
            ).await;
        }

        println!("Timer bot running. Use /set <seconds>");
        app.run_polling().await.unwrap();
    });
}
```

Key concepts:

- Create a `JobQueue` and pass it to `ApplicationBuilder::job_queue()`.
- Access the queue inside handlers via `context.job_queue.as_ref()`.
- `jq.once(callback, duration)` schedules a one-shot job. Use `.name()` and `.chat_id()` for metadata.
- `job.schedule_removal()` cancels a scheduled job.
- `jq.jobs().await` returns all currently scheduled jobs.

**Try it:** Add a `/repeat <seconds> <message>` command that sends the given message every N seconds until `/unset` is called.

### Webhook Bot for Production Deployment

For production, webhooks are more efficient than polling. This example shows a custom axum server alongside the Telegram bot.

```rust
use std::sync::Arc;
use axum::routing::{get, post};
use axum::Router;
use tokio::net::TcpListener;
use telegram_bot::ext::prelude::*;

async fn start(update: Update, context: Context) -> HandlerResult {
    context.reply_text(&update, "Hello from webhook mode!").await?;
    Ok(())
}

async fn handle_telegram_webhook(
    axum::extract::State(tx): axum::extract::State<tokio::sync::mpsc::UnboundedSender<Update>>,
    body: axum::body::Bytes,
) -> axum::http::StatusCode {
    match serde_json::from_slice::<Update>(&body) {
        Ok(update) => {
            let _ = tx.send(update);
            axum::http::StatusCode::OK
        }
        Err(_) => axum::http::StatusCode::BAD_REQUEST,
    }
}

async fn healthcheck() -> &'static str {
    "OK"
}

fn main() {
    telegram_bot::run(async {
        tracing_subscriber::fmt::init();

        let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
        let webhook_url = std::env::var("WEBHOOK_URL").unwrap();
        let port: u16 = std::env::var("PORT").unwrap_or_else(|_| "8000".into()).parse().unwrap();

        let app = ApplicationBuilder::new().token(&token).build();

        app.add_typed_handler(CommandHandler::new("start", start), 0).await;

        // Manual lifecycle control
        app.initialize().await.unwrap();
        app.start().await.unwrap();

        // Set the webhook on Telegram's side
        let full_url = format!("{webhook_url}/telegram");
        app.bot().set_webhook(&full_url).send().await.unwrap();

        // Build the axum router
        let router = Router::new()
            .route("/telegram", post(handle_telegram_webhook))
            .route("/health", get(healthcheck))
            .with_state(app.update_sender());

        let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

        println!("Webhook bot listening on port {port}");

        // Run until Ctrl+C
        let shutdown = Arc::new(tokio::sync::Notify::new());
        let shutdown_clone = shutdown.clone();
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.unwrap();
            shutdown_clone.notify_waiters();
        });

        axum::serve(listener, router)
            .with_graceful_shutdown(async move { shutdown.notified().await })
            .await
            .unwrap();

        // Clean shutdown
        app.stop().await.unwrap();
        app.shutdown().await.unwrap();
    });
}
```

Key differences from polling:

- Call `initialize()` and `start()` manually instead of `run_polling()`.
- Use `app.update_sender()` to get a channel sender that feeds updates into the Application's dispatch loop.
- Your axum endpoint deserializes the raw JSON into an `Update` and sends it through the channel.
- Set the webhook URL with `app.bot().set_webhook(&url).send().await`.
- On shutdown, call `stop()` and `shutdown()` explicitly.

**Try it:** Add a `/health` endpoint that returns the bot's uptime and the number of updates processed.

### Inline Mode Bot

Inline mode lets users invoke your bot from any chat by typing `@botname query`. The bot responds with a list of results displayed in a popup.

**Important:** Enable inline mode with `@BotFather` by sending `/setinline` before running this bot.

```rust
use serde_json::json;
use telegram_bot::ext::prelude::*;

async fn start(update: Update, context: Context) -> HandlerResult {
    context
        .reply_text(&update, "Use me inline! Type @botusername <text> in any chat.")
        .await?;
    Ok(())
}

async fn inline_query_handler(update: Update, context: Context) -> HandlerResult {
    let iq = match &update.inline_query {
        Some(q) => q,
        None => return Ok(()),
    };

    if iq.query.is_empty() {
        return Ok(());
    }

    let query = &iq.query;
    let escaped = query.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");

    let results = vec![
        json!({
            "type": "article",
            "id": format!("caps-{}", iq.id),
            "title": "CAPS",
            "input_message_content": {
                "message_text": query.to_uppercase(),
            },
        }),
        json!({
            "type": "article",
            "id": format!("bold-{}", iq.id),
            "title": "Bold",
            "input_message_content": {
                "message_text": format!("<b>{escaped}</b>"),
                "parse_mode": "HTML",
            },
        }),
        json!({
            "type": "article",
            "id": format!("italic-{}", iq.id),
            "title": "Italic",
            "input_message_content": {
                "message_text": format!("<i>{escaped}</i>"),
                "parse_mode": "HTML",
            },
        }),
    ];

    context
        .bot()
        .answer_inline_query(&iq.id, results)
        .send()
        .await?;

    Ok(())
}

fn main() {
    telegram_bot::run(async {
        tracing_subscriber::fmt::init();
        let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
        let app = ApplicationBuilder::new().token(token).build();

        app.add_typed_handler(CommandHandler::new("start", start), 0).await;
        app.add_typed_handler(FnHandler::on_inline_query(inline_query_handler), 0).await;

        println!("Inline bot running. Enable inline mode with @BotFather!");
        app.run_polling().await.unwrap();
    });
}
```

Key concepts:

- `FnHandler::on_inline_query(f)` matches inline query updates.
- `update.inline_query` contains the query text and a unique ID.
- Results are `InlineQueryResultArticle` JSON objects with a unique `id`, a `title`, and `input_message_content`.
- `answer_inline_query` sends the results back to Telegram for display.
- Always escape HTML entities when using `ParseMode::Html` with user input.

**Try it:** Add a "Reverse" option that sends the query text reversed. Add a "Code" option that wraps it in `<code>` tags.

---

## Part 4: Advanced Topics

### Persistence

By default, all data (bot_data, user_data, chat_data) lives in memory and is lost when the bot restarts. Persistence backends save this data to disk.

**JSON file persistence:**

```rust
use telegram_bot::ext::persistence::json_file::JsonFilePersistence;

let persistence = JsonFilePersistence::new(
    "my_bot",   // file prefix (creates my_bot.json or my_bot_*.json)
    true,       // single_file mode (all data in one file)
    false,      // update_on_flush (batch writes, not per-update)
);

let app = ApplicationBuilder::new()
    .token(token)
    .persistence(Box::new(persistence))
    .build();
```

Once persistence is configured, `context.user_data()`, `context.chat_data()`, and `context.bot_data()` automatically load from and save to disk. The Application handles the flush cycle (default: every 60 seconds, plus on shutdown).

**SQLite persistence** (requires the `persistence-sqlite` feature):

```rust
use telegram_bot::ext::persistence::sqlite::SqlitePersistence;

let persistence = SqlitePersistence::open("my_bot.db")?;

let app = ApplicationBuilder::new()
    .token(token)
    .persistence(Box::new(persistence))
    .build();
```

SQLite is recommended for bots with many users because it handles concurrent access better than a single JSON file and does not need to rewrite the entire file on each flush.

**Custom persistence:**

Implement the `BasePersistence` trait to store data in any backend (Redis, PostgreSQL, S3, etc.):

```rust
use telegram_bot::ext::persistence::base::{BasePersistence, PersistenceResult};

#[derive(Debug)]
struct MyPersistence { /* ... */ }

impl BasePersistence for MyPersistence {
    async fn get_user_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> { /* ... */ }
    async fn get_chat_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> { /* ... */ }
    async fn get_bot_data(&self) -> PersistenceResult<JsonMap> { /* ... */ }
    async fn update_user_data(&self, user_id: i64, data: &JsonMap) -> PersistenceResult<()> { /* ... */ }
    async fn update_chat_data(&self, chat_id: i64, data: &JsonMap) -> PersistenceResult<()> { /* ... */ }
    async fn update_bot_data(&self, data: &JsonMap) -> PersistenceResult<()> { /* ... */ }
    async fn flush(&self) -> PersistenceResult<()> { /* ... */ }
    // ... additional required methods
}
```

**Try it:** Start the persistent_conversation_bot example. Tell it your name and age, then restart the bot. Send `/show_data` to verify your data survived the restart.

### Error Handling

Errors in handlers are propagated to registered error handlers. This keeps your handler code clean while centralizing error reporting.

**Returning errors from handlers:**

```rust
async fn bad_command(_update: Update, _context: Context) -> HandlerResult {
    Err(HandlerError::Other(Box::new(
        std::io::Error::new(std::io::ErrorKind::Other, "Something went wrong!"),
    )))
}
```

The `?` operator on Bot API calls automatically converts `TelegramError` into `HandlerError::Other`, so you can write:

```rust
context.bot().send_message(chat_id, text).send().await?;
```

If the API call fails, the error propagates to the error handler.

**Registering an error handler:**

```rust
async fn error_handler(update: Option<Update>, context: CallbackContext) -> bool {
    let error_text = context
        .error
        .as_ref()
        .map(|e| format!("{e}"))
        .unwrap_or_else(|| "Unknown error".to_string());

    tracing::error!("Exception while handling an update: {error_text}");

    // Send error details to a developer chat
    let dev_chat_id: i64 = 123456789;
    let _ = context
        .bot()
        .send_message(dev_chat_id, &format!("Error: {error_text}"))
        .send()
        .await;

    // Return false to let other error handlers also run
    false
}

// Register it
app.add_error_handler(
    Arc::new(|update, ctx| Box::pin(error_handler(update, ctx))),
    true,  // block: wait for this handler to finish before continuing
).await;
```

The error handler receives:
- `update: Option<Update>` -- the update that caused the error (if available).
- `context.error` -- the actual error as `Option<Arc<dyn Error>>`.
- `context.chat_data()`, `context.user_data()` -- for diagnostic context.

Return `true` to stop processing further error handlers, `false` to let them all run.

**Try it:** Register an error handler that counts errors in `bot_data` and sends a summary when the count exceeds a threshold.

### Group and Channel Bots

Bots in groups see messages differently depending on privacy mode:

- **Privacy mode ON** (default): The bot only sees messages that are commands (`/command`), replies to the bot, or mentions of the bot.
- **Privacy mode OFF**: The bot sees all messages. Disable it via `@BotFather -> /setprivacy -> Disable`.

For group bots, always use `update.effective_chat()` to get the chat context. The chat type tells you the environment:

```rust
async fn handler(update: Update, context: Context) -> HandlerResult {
    let chat = update.effective_chat().unwrap();
    match chat.type_field.as_str() {
        "private" => { /* one-on-one conversation */ }
        "group" | "supergroup" => { /* group chat */ }
        "channel" => { /* channel post */ }
        _ => {}
    }
    Ok(())
}
```

For admin commands in groups, verify the sender's role:

```rust
async fn admin_only(update: Update, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().unwrap().id;
    let user_id = update.effective_user().unwrap().id;

    let member = context
        .bot()
        .get_chat_member(chat_id, user_id)
        .await?;

    // member is a ChatMember enum with variants: Owner, Administrator, Member, etc.
    // Check if the user is an admin or owner
    let is_admin = matches!(member, telegram_bot::raw::types::chat_member::ChatMember::Owner(_)
        | telegram_bot::raw::types::chat_member::ChatMember::Administrator(_));

    if !is_admin {
        context.reply_text(&update, "This command is for admins only.").await?;
        return Ok(());
    }

    // Admin-only logic here
    Ok(())
}
```

**Try it:** Build a bot that tracks join/leave events using `FnHandler::on_chat_member` and sends a welcome message when new members join.

### Payments and Invoices

Telegram supports payments through the Bot API. You need a payment provider token from `@BotFather`.

```rust
use serde_json::json;
use telegram_bot::ext::prelude::*;

async fn start_payment(update: Update, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().unwrap().id;

    let prices = vec![json!({"label": "Widget", "amount": 500})]; // 500 cents = $5.00

    context
        .bot()
        .send_invoice(
            chat_id,
            "Purchase Widget",
            "A very useful widget.",
            "widget-payload-001",
            "USD",
            prices,
        )
        .provider_token("your-provider-token")
        .need_name(true)
        .need_email(true)
        .send()
        .await?;

    Ok(())
}

async fn precheckout(update: Update, context: Context) -> HandlerResult {
    let query = update.pre_checkout_query.as_ref().unwrap();

    // Verify the payload
    if query.invoice_payload == "widget-payload-001" {
        context.bot().answer_pre_checkout_query(&query.id, true).send().await?;
    } else {
        context
            .bot()
            .answer_pre_checkout_query(&query.id, false)
            .error_message("Invalid payment")
            .send()
            .await?;
    }
    Ok(())
}

async fn successful_payment(update: Update, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().unwrap().id;
    context.bot().send_message(chat_id, "Payment received! Thank you.").send().await?;
    Ok(())
}

fn main() {
    telegram_bot::run(async {
        let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
        let app = ApplicationBuilder::new().token(token).build();

        app.add_typed_handler(CommandHandler::new("buy", start_payment), 0).await;
        app.add_typed_handler(FnHandler::on_pre_checkout_query(precheckout), 0).await;
        app.add_typed_handler(
            FnHandler::new(
                |u| u.effective_message().and_then(|m| m.successful_payment.as_ref()).is_some(),
                successful_payment,
            ),
            0,
        ).await;

        app.run_polling().await.unwrap();
    });
}
```

The payment flow:
1. Your bot sends an invoice via `send_invoice`.
2. The user fills in their payment details on Telegram's payment sheet.
3. Telegram sends a `pre_checkout_query` -- your bot must answer with `true` (approve) or `false` (reject).
4. If approved and payment succeeds, your bot receives a message with `successful_payment`.

For invoices with shipping, add `.is_flexible(true)` and handle `FnHandler::on_shipping_query` to provide shipping options.

**Try it:** Run the `payment_bot` example with a test payment provider token from BotFather.

### Custom Filters

For complex matching logic that you want to reuse across multiple handlers, implement the `Filter` trait:

```rust
use telegram_bot::ext::prelude::*;

/// Filter that matches messages from a specific set of user IDs.
struct AllowedUsers {
    user_ids: Vec<i64>,
}

impl Filter for AllowedUsers {
    fn check_update(&self, update: &Update) -> FilterResult {
        match update.effective_user() {
            Some(user) if self.user_ids.contains(&user.id) => FilterResult::Match,
            _ => FilterResult::NoMatch,
        }
    }

    fn name(&self) -> &str {
        "AllowedUsers"
    }
}

// Use it with MessageHandler
let admin_filter = F::new(AllowedUsers { user_ids: vec![123, 456] });
let handler = MessageHandler::new(admin_filter & TEXT(), admin_only_fn);
```

Data filters can extract information for use by the handler:

```rust
struct ExtractUrls;

impl Filter for ExtractUrls {
    fn check_update(&self, update: &Update) -> FilterResult {
        let urls: Vec<String> = update
            .effective_message()
            .and_then(|m| m.entities.as_ref())
            .map(|entities| {
                entities.iter()
                    .filter(|e| e.entity_type == MessageEntityType::Url)
                    .filter_map(|e| {
                        let text = update.effective_message()?.text.as_ref()?;
                        let start = e.offset as usize;
                        let end = start + e.length as usize;
                        Some(text[start..end].to_string())
                    })
                    .collect()
            })
            .unwrap_or_default();

        if urls.is_empty() {
            FilterResult::NoMatch
        } else {
            let mut data = std::collections::HashMap::new();
            data.insert("urls".to_string(), urls);
            FilterResult::MatchWithData(data)
        }
    }

    fn name(&self) -> &str {
        "ExtractUrls"
    }
}
```

**Try it:** Create a `TimeOfDay` filter that only matches messages sent during business hours (9am-5pm in UTC).

### Nested Conversations

The framework supports nested conversations where one conversation can hand off control to another and receive state back when it completes. This is implemented via the `ConversationHandler` builder:

```rust
use telegram_bot::ext::handlers::conversation::*;

#[derive(Clone, Hash, Eq, PartialEq)]
enum State { AskName, AskAge }

let conv = ConversationHandler::builder()
    .entry_point(Box::new(start_handler))
    .state(State::AskName, vec![Box::new(name_handler)])
    .state(State::AskAge, vec![Box::new(age_handler)])
    .fallback(Box::new(cancel_handler))
    .build();
```

The `ConversationHandler` maintains a state machine per conversation key (typically `(chat_id, user_id)`). Each step handler returns a `ConversationResult<S>`:

- `ConversationResult::NextState(state)` -- transition to the given state.
- `ConversationResult::End` -- end the conversation and remove the state.
- `ConversationResult::Stay` -- remain in the current state.

For nested conversations, use `map_to_parent` to specify how the child conversation's end states map back to the parent's states.

See the `nested_conversation_bot` example for a full working implementation.

### Typed Data Access

The `DataReadGuard` and `DataWriteGuard` types provide typed accessors over the `HashMap<String, Value>` data stores, eliminating manual JSON value extraction:

```rust
// Reading data (immutable, shared access)
let guard = context.bot_data().await;
let name: Option<&str> = guard.get_str("admin_name");
let count: Option<i64> = guard.get_i64("request_count");
let threshold: Option<f64> = guard.get_f64("threshold");
let active: Option<bool> = guard.get_bool("maintenance");
let raw_value: Option<&Value> = guard.get("any_key");
let id_set: HashSet<i64> = guard.get_id_set("admin_ids");
let is_empty: bool = guard.is_empty();
let size: usize = guard.len();

// Writing data (exclusive access)
let mut guard = context.bot_data_mut().await;
guard.set_str("admin_name", "Alice");
guard.set_i64("request_count", 42);
guard.set_bool("maintenance", false);
guard.insert("complex".to_string(), serde_json::json!({"nested": true}));
guard.add_to_id_set("admin_ids", 12345);
guard.remove_from_id_set("admin_ids", 67890);
guard.remove("obsolete_key");
```

The `Deref` implementation means you can also use raw `HashMap` methods directly on the guard:

```rust
let guard = context.bot_data().await;
if guard.contains_key("my_key") {
    // ...
}
for (key, value) in guard.iter() {
    // ...
}
```

This typed layer is especially valuable when combined with persistence, because it provides a clear interface for the kinds of data your bot stores without requiring custom serde structs for simple key-value patterns.

**Try it:** Build a bot that uses `bot_data_mut().await.add_to_id_set("subscribers", user_id)` to maintain a subscriber list, and a `/broadcast <message>` command that sends to all subscribers.

---

## Part 5: Production Deployment

### Release Build

Development builds are unoptimized and large. For production, always build in release mode:

```sh
cargo build --release
```

To minimize binary size further:

```toml
# Cargo.toml
[profile.release]
lto = true        # Link-time optimization
codegen-units = 1 # Single codegen unit for better optimization
strip = true      # Strip debug symbols
```

With these settings, a typical bot binary is approximately 10MB. Runtime memory usage is 15-27MB RSS, depending on the number of concurrent users and cached data.

### Docker Deployment

A multi-stage Dockerfile for minimal image size:

```dockerfile
# Stage 1: Build
FROM rust:1.85-slim AS builder

WORKDIR /app
COPY . .
RUN cargo build --release --bin my_bot

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/my_bot /usr/local/bin/my_bot

ENV TELEGRAM_BOT_TOKEN=""
EXPOSE 8000

CMD ["my_bot"]
```

Build and run:

```sh
docker build -t my-telegram-bot .
docker run -e TELEGRAM_BOT_TOKEN="your-token" my-telegram-bot
```

For even smaller images, use `alpine` as the runtime base and compile with `musl`:

```dockerfile
FROM rust:1.85-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY . .
RUN cargo build --release --bin my_bot

FROM alpine:3.20
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/my_bot /usr/local/bin/my_bot
CMD ["my_bot"]
```

### Webhook vs Polling

| Aspect | Polling | Webhook |
|--------|---------|---------|
| **Setup** | Zero configuration | Requires HTTPS domain + TLS certificate |
| **Latency** | Higher (poll interval) | Near-instant (Telegram pushes updates) |
| **Resource use** | Continuous HTTP requests | Only on incoming updates |
| **Hosting** | Any server, even behind NAT | Public HTTPS endpoint required |
| **Best for** | Development, small bots | Production, high-traffic bots |

**Use polling when:**
- You are developing locally.
- Your server is behind a NAT or firewall.
- Your bot has low traffic (fewer than 100 messages/minute).

**Use webhooks when:**
- You are deploying to production.
- You have a public domain with TLS.
- You need minimal latency.
- You are running on a cloud platform (AWS, GCP, Railway, Fly.io).

### Environment Variables and Secrets

Never hardcode secrets in source code. Use environment variables:

```rust
let token = std::env::var("TELEGRAM_BOT_TOKEN")
    .expect("TELEGRAM_BOT_TOKEN must be set");

let admin_id: i64 = std::env::var("ADMIN_CHAT_ID")
    .unwrap_or_else(|_| "0".into())
    .parse()
    .unwrap_or(0);
```

For local development, use a `.env` file with a tool like `dotenvy`:

```sh
# .env
TELEGRAM_BOT_TOKEN=123456:ABC-DEF
ADMIN_CHAT_ID=789012
```

**Always** add `.env` to your `.gitignore`.

For Docker, pass secrets at runtime:

```sh
docker run -e TELEGRAM_BOT_TOKEN="$TOKEN" my-bot
```

For Kubernetes, use Secrets. For cloud platforms, use their secrets management service (AWS Secrets Manager, GCP Secret Manager, etc.).

### Monitoring with Tracing

The framework uses the `tracing` crate for structured logging. Initialize it in your main function:

```rust
tracing_subscriber::fmt::init();
```

For production, configure JSON output and log levels:

```rust
use tracing_subscriber::EnvFilter;

tracing_subscriber::fmt()
    .json()
    .with_env_filter(
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info,telegram_bot=debug"))
    )
    .init();
```

Set the log level via the `RUST_LOG` environment variable:

```sh
RUST_LOG=info cargo run                   # Standard
RUST_LOG=debug cargo run                  # Verbose
RUST_LOG=telegram_bot=trace cargo run     # Framework internals
```

Add custom spans and events in your handlers:

```rust
use tracing::{info, warn, instrument};

#[instrument(skip(context))]
async fn my_handler(update: Update, context: Context) -> HandlerResult {
    let user_id = update.effective_user().map(|u| u.id).unwrap_or(0);
    info!(user_id, "Processing request");

    if some_condition {
        warn!(user_id, "Unusual condition detected");
    }

    Ok(())
}
```

### Security Best Practices

1. **Keep your token secret.** Never commit it to version control. Use environment variables or a secrets manager.

2. **Validate input.** Never trust user-supplied data. Parse numbers with `.parse()` and handle errors. Sanitize text before embedding it in HTML messages.

3. **Use HTTPS for webhooks.** Telegram requires HTTPS. Use a reverse proxy (nginx, Caddy) or a platform that provides TLS automatically.

4. **Restrict admin commands.** Always verify the sender's identity before executing privileged operations. Use `get_chat_member` to check admin status.

5. **Rate limit expensive operations.** The `ApplicationBuilder::concurrent_updates(n)` setting controls how many updates are processed simultaneously. Set it based on your bot's workload.

6. **Handle errors gracefully.** Register an error handler that logs the error and notifies a developer. Never expose internal error details to users.

7. **Update dependencies.** Run `cargo update` regularly and audit your dependency tree with `cargo audit`.

8. **Minimize attack surface.** In Docker, run as a non-root user and use minimal base images.

---

## Where to Go from Here

- Browse the [20 example bots](https://github.com/HexiCoreDev/rust-telegram-bot/tree/main/crates/telegram-bot/examples) in the repository.
- Read the [Telegram Bot API documentation](https://core.telegram.org/bots/api) for the full list of available methods and types.
- Check the [wiki](https://github.com/HexiCoreDev/rust-telegram-bot/wiki) for FAQs, troubleshooting, and performance tuning.
- Open an issue or pull request on [GitHub](https://github.com/HexiCoreDev/rust-telegram-bot) if you find a bug or have a feature request.

Happy building.
