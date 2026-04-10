# Persistence

Persistence lets your bot remember data across restarts. Without it, all user data, chat data, and bot-wide data live only in memory and vanish when the process exits.

## How It Works

The framework provides a `BasePersistence` trait that defines how data is stored and loaded. You pick an implementation, pass it to `ApplicationBuilder`, and the `Application` handles the rest -- loading data at startup, flushing changes periodically, and saving on shutdown.

Three backends ship out of the box:

| Backend | Feature Flag | Best For |
|---|---|---|
| `DictPersistence` | `persistence` | Testing, prototyping (in-memory only) |
| `JsonFilePersistence` | `persistence-json` | Simple bots, human-readable storage |
| `SqlitePersistence` | `persistence-sqlite` | Production bots, concurrent access |

## Enabling Persistence

Add the appropriate feature to your `Cargo.toml`:

```toml
[dependencies]
rust-tg-bot = { version = "1.0.0-beta.5", features = ["persistence-json"] }
```

Or for SQLite:

```toml
[dependencies]
rust-tg-bot = { version = "1.0.0-beta.5", features = ["persistence-sqlite"] }
```

## JsonFilePersistence

The most common choice for getting started. Stores all data in one or more JSON files on disk.

```rust
use rust_tg_bot::ext::persistence::json_file::JsonFilePersistence;
use rust_tg_bot::ext::prelude::{
    Application, ApplicationBuilder, Arc, CommandHandler, Context,
    HandlerResult, Update,
};

async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    context.reply_text(&update, "Hello! Your data will persist.").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();

    // Arguments: file prefix, single-file mode, pretty-print
    let persistence = JsonFilePersistence::new("my_bot_data", true, false);

    let app: Arc<Application> = ApplicationBuilder::new()
        .token(token)
        .persistence(Box::new(persistence))
        .build();

    app.add_handler(CommandHandler::new("start", start), 0).await;

    app.run_polling().await.unwrap();
}
```

The three arguments to `JsonFilePersistence::new`:

1. **File prefix** -- the base name for the JSON file(s). Single-file mode creates `my_bot_data.json`.
2. **Single-file mode** -- `true` stores everything in one file; `false` creates separate files per data category (`my_bot_data_user_data.json`, `my_bot_data_chat_data.json`, etc.).
3. **Pretty-print** -- `true` formats the JSON with indentation for debugging; `false` is compact.

## SqlitePersistence

For production bots that need reliability under concurrent load:

```rust
use rust_tg_bot::ext::persistence::sqlite::SqlitePersistence;
use rust_tg_bot::ext::prelude::{Application, ApplicationBuilder, Arc};

#[tokio::main]
async fn main() {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();

    let persistence = SqlitePersistence::open("bot.db")
        .expect("failed to open SQLite database");

    let app: Arc<Application> = ApplicationBuilder::new()
        .token(token)
        .persistence(Box::new(persistence))
        .build();

    // ... register handlers ...

    app.run_polling().await.unwrap();
}
```

`SqlitePersistence::open` creates the database file and initialises the schema automatically. It uses WAL journal mode for better concurrent read performance and wraps the connection in a `tokio::sync::Mutex` to prevent `SQLITE_BUSY` errors.

For testing, use the in-memory variant:

```rust
let persistence = SqlitePersistence::in_memory()
    .expect("failed to create in-memory SQLite database");
```

## Accessing Data from Handlers

Once persistence is configured, `Context` gives you access to three data scopes.

### User Data

Scoped to the user who triggered the update. Each user gets their own `HashMap<String, JsonValue>`.

```rust
use rust_tg_bot::ext::prelude::{
    Arc, Context, HandlerResult, JsonValue, Update,
};

async fn save_preference(update: Arc<Update>, context: Context) -> HandlerResult {
    // Read current user data (returns a cloned snapshot)
    let user_data = context.user_data().await.unwrap_or_default();
    let visit_count = user_data
        .get("visits")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    // Write a new value
    context
        .set_user_data("visits".to_string(), JsonValue::from(visit_count + 1))
        .await;

    context
        .reply_text(&update, &format!("Visit count: {}", visit_count + 1))
        .await?;
    Ok(())
}
```

### Chat Data

Scoped to the chat where the update originated. Useful for group settings.

```rust
async fn set_welcome(update: Arc<Update>, context: Context) -> HandlerResult {
    let text = update
        .effective_message()
        .and_then(|m| m.text.as_deref())
        .unwrap_or("Welcome!");

    context
        .set_chat_data(
            "welcome_message".to_string(),
            JsonValue::String(text.to_string()),
        )
        .await;

    context.reply_text(&update, "Welcome message updated.").await?;
    Ok(())
}
```

### Bot Data

Shared across all users and chats. Accessed through typed guards that wrap `tokio::sync::RwLock`.

```rust
async fn global_counter(update: Arc<Update>, context: Context) -> HandlerResult {
    // Read with a typed guard
    let count = {
        let data = context.bot_data().await;
        data.get_i64("global_count").unwrap_or(0)
    };

    // Write with a typed guard
    {
        let mut data = context.bot_data_mut().await;
        data.set_i64("global_count", count + 1);
    }

    context
        .reply_text(&update, &format!("Global message count: {}", count + 1))
        .await?;
    Ok(())
}
```

## DataReadGuard and DataWriteGuard

The `bot_data()` and `bot_data_mut()` methods return typed guards that provide convenience accessors. These eliminate manual `get().and_then(|v| v.as_*)` chains.

### DataReadGuard

```rust
let data = context.bot_data().await;

data.get_str("name");        // Option<&str>
data.get_i64("count");       // Option<i64>
data.get_f64("ratio");       // Option<f64>
data.get_bool("enabled");    // Option<bool>
data.get("raw_key");         // Option<&Value>
data.get_id_set("user_ids"); // HashSet<i64>
data.raw();                  // &HashMap<String, Value>
data.is_empty();             // bool
data.len();                  // usize
```

### DataWriteGuard

```rust
let mut data = context.bot_data_mut().await;

data.set_str("name", "MyBot");
data.set_i64("count", 42);
data.set_bool("enabled", true);
data.insert("key".to_string(), JsonValue::Array(vec![]));
data.add_to_id_set("user_ids", 12345);
data.remove_from_id_set("user_ids", 12345);
data.remove("old_key");
data.entry("key".to_string());  // HashMap Entry API
data.raw_mut();                 // &mut HashMap<String, Value>
```

Both guards implement `Deref` (and `DerefMut` for the write guard) to `HashMap<String, Value>`, so you can also use standard `HashMap` methods directly.

## The BasePersistence Trait

If the built-in backends do not fit your needs, implement `BasePersistence` yourself. The trait requires `Send + Sync` because it is stored behind an `Arc` and accessed from multiple async tasks. It uses native `async fn` in traits (stabilised in Rust 1.75) -- no `async_trait` macro needed.

```rust
use rust_tg_bot::ext::persistence::base::{
    BasePersistence, PersistenceInput, PersistenceResult,
};
use std::collections::HashMap;

#[derive(Debug)]
struct RedisPersistence {
    // your connection pool
}

impl BasePersistence for RedisPersistence {
    async fn get_user_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> {
        // load from Redis
        todo!()
    }

    async fn update_user_data(
        &self, user_id: i64, data: &JsonMap,
    ) -> PersistenceResult<()> {
        // save to Redis
        todo!()
    }

    async fn get_chat_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> {
        todo!()
    }

    async fn update_chat_data(
        &self, chat_id: i64, data: &JsonMap,
    ) -> PersistenceResult<()> {
        todo!()
    }

    async fn get_bot_data(&self) -> PersistenceResult<JsonMap> {
        todo!()
    }

    async fn update_bot_data(&self, data: &JsonMap) -> PersistenceResult<()> {
        todo!()
    }

    async fn get_callback_data(&self) -> PersistenceResult<Option<CdcData>> {
        todo!()
    }

    async fn update_callback_data(&self, data: &CdcData) -> PersistenceResult<()> {
        todo!()
    }

    async fn get_conversations(
        &self, name: &str,
    ) -> PersistenceResult<ConversationDict> {
        todo!()
    }

    async fn update_conversation(
        &self,
        name: &str,
        key: &ConversationKey,
        new_state: Option<&serde_json::Value>,
    ) -> PersistenceResult<()> {
        todo!()
    }

    async fn drop_chat_data(&self, chat_id: i64) -> PersistenceResult<()> {
        todo!()
    }

    async fn drop_user_data(&self, user_id: i64) -> PersistenceResult<()> {
        todo!()
    }

    async fn flush(&self) -> PersistenceResult<()> {
        // flush pending writes
        todo!()
    }
}
```

Key trait methods:

| Method | Purpose |
|---|---|
| `get_user_data` / `get_chat_data` / `get_bot_data` | Load data at startup |
| `update_user_data` / `update_chat_data` / `update_bot_data` | Persist changes |
| `get_conversations` / `update_conversation` | Store conversation state |
| `get_callback_data` / `update_callback_data` | Store callback data cache |
| `drop_chat_data` / `drop_user_data` | Delete data for a specific entity |
| `flush` | Called on shutdown to save pending writes |
| `update_interval` | How often (in seconds) the Application flushes (default: 60) |
| `store_data` | Returns `PersistenceInput` controlling which categories are persisted |
| `refresh_user_data` / `refresh_chat_data` / `refresh_bot_data` | Optional hooks called before dispatching |

## Complete Example

This example collects facts about the user and persists them across restarts:

```rust
use rust_tg_bot::ext::persistence::json_file::JsonFilePersistence;
use rust_tg_bot::ext::prelude::{
    Application, ApplicationBuilder, Arc, CommandHandler, Context,
    HandlerResult, JsonValue, MessageHandler, Update, COMMAND, TEXT,
};

async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let user_data = context.user_data().await.unwrap_or_default();
    let name = update
        .effective_user()
        .map(|u| u.first_name.clone())
        .unwrap_or_else(|| "stranger".to_string());

    if user_data.is_empty() || user_data.keys().all(|k| k.starts_with('_')) {
        context
            .reply_text(
                &update,
                &format!("Welcome, {name}! Tell me something about yourself."),
            )
            .await?;
    } else {
        let facts: Vec<String> = user_data
            .iter()
            .filter(|(k, _)| !k.starts_with('_'))
            .map(|(k, v)| format!("{k}: {}", v.as_str().unwrap_or("?")))
            .collect();
        context
            .reply_text(
                &update,
                &format!(
                    "Welcome back, {name}! I remember:\n{}",
                    facts.join("\n"),
                ),
            )
            .await?;
    }
    Ok(())
}

async fn remember(update: Arc<Update>, context: Context) -> HandlerResult {
    let text = update
        .effective_message()
        .and_then(|m| m.text.as_deref())
        .unwrap_or("");

    if let Some((key, value)) = text.split_once(':') {
        context
            .set_user_data(
                key.trim().to_string(),
                JsonValue::String(value.trim().to_string()),
            )
            .await;
        context.reply_text(&update, "Got it! I will remember that.").await?;
    } else {
        context
            .reply_text(&update, "Send facts as 'key: value'.")
            .await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let persistence = JsonFilePersistence::new("remember_bot", true, false);

    let app: Arc<Application> = ApplicationBuilder::new()
        .token(token)
        .persistence(Box::new(persistence))
        .build();

    app.add_handler(CommandHandler::new("start", start), 0).await;
    app.add_handler(
        MessageHandler::new(TEXT() & !COMMAND(), remember), 0,
    ).await;

    app.run_polling().await.unwrap();
}
```

## Next Steps

- [Webhooks](webhooks.md) -- run your bot in webhook mode for production.
- [Error Handling](../advanced/error-handling.md) -- handle persistence errors gracefully.
