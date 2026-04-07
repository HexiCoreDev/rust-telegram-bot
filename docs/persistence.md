# Persistence

Persistence lets your bot survive restarts. User data, chat data, bot-wide data,
conversation states, and callback data cache are all saved and reloaded automatically by
the `Application`.

---

## What Gets Persisted

The `PersistenceInput` struct controls which categories are stored:

```rust
pub struct PersistenceInput {
    pub bot_data: bool,      // bot-wide key/value store
    pub chat_data: bool,     // per-chat key/value store
    pub user_data: bool,     // per-user key/value store
    pub callback_data: bool, // inline keyboard callback data cache
}
```

All flags default to `true`. Override in your custom backend by returning a different
`PersistenceInput` from `store_data()`.

Data is stored as `HashMap<String, serde_json::Value>` (aliased as `JsonMap`). Use
`serde_json::json!` to insert structured values.

---

## BasePersistence Trait

```rust
pub trait BasePersistence: Send + Sync + fmt::Debug {
    // Read
    async fn get_user_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>>;
    async fn get_chat_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>>;
    async fn get_bot_data(&self) -> PersistenceResult<JsonMap>;
    async fn get_callback_data(&self) -> PersistenceResult<Option<CdcData>>;
    async fn get_conversations(&self, name: &str) -> PersistenceResult<ConversationDict>;

    // Write
    async fn update_user_data(&self, user_id: i64, data: &JsonMap) -> PersistenceResult<()>;
    async fn update_chat_data(&self, chat_id: i64, data: &JsonMap) -> PersistenceResult<()>;
    async fn update_bot_data(&self, data: &JsonMap) -> PersistenceResult<()>;
    async fn update_callback_data(&self, data: &CdcData) -> PersistenceResult<()>;
    async fn update_conversation(&self, name: &str, key: &ConversationKey,
                                 new_state: Option<&Value>) -> PersistenceResult<()>;

    // Delete
    async fn drop_chat_data(&self, chat_id: i64) -> PersistenceResult<()>;
    async fn drop_user_data(&self, user_id: i64) -> PersistenceResult<()>;

    // Optional hooks
    async fn refresh_user_data(&self, user_id: i64, data: &mut JsonMap) -> PersistenceResult<()> { Ok(()) }
    async fn refresh_chat_data(&self, chat_id: i64, data: &mut JsonMap) -> PersistenceResult<()> { Ok(()) }
    async fn refresh_bot_data(&self, data: &mut JsonMap) -> PersistenceResult<()> { Ok(()) }

    // Lifecycle
    async fn flush(&self) -> PersistenceResult<()>;
    fn update_interval(&self) -> f64 { 60.0 }  // seconds between auto-save cycles
    fn store_data(&self) -> PersistenceInput { PersistenceInput::default() }
}
```

The `Application` calls the read methods once during initialisation to populate its
in-memory stores. It calls the write methods after each update cycle and periodically
(every `update_interval()` seconds). `flush()` is called when the application stops.

The refresh hooks (`refresh_user_data`, `refresh_chat_data`, `refresh_bot_data`) are
called before each dispatch, giving the backend a chance to pull fresh data from an
external source (e.g. a remote database).

---

## In-Memory (DictPersistence)

For development and testing: stores everything in memory. Data is lost when the process
exits.

```rust
use telegram_bot_ext::persistence::dict::DictPersistence;

let persistence = DictPersistence::new();

let app = ApplicationBuilder::new()
    .token(token)
    .persistence(Box::new(persistence))
    .build();
```

`DictPersistence` is the Rust equivalent of Python's `PicklePersistence` used in test
scenarios, or simply using the in-memory stores without any backend.

---

## JSON File Persistence

Saves each data category as a separate JSON file in a directory. Human-readable, good
for development bots and small deployments.

```rust
use telegram_bot_ext::persistence::json_file::JsonFilePersistence;

// Arguments: directory path, store_user_data, store_chat_data (others enabled by default)
let persistence = JsonFilePersistence::new("./bot_data", true, true);

let app = ApplicationBuilder::new()
    .token(token)
    .persistence(Box::new(persistence))
    .build();
```

Files created (example with directory `./bot_data`):

```
bot_data/
  user_data.json
  chat_data.json
  bot_data.json
  conversations.json
  callback_data.json
```

Enable it via the feature flag:

```toml
telegram-bot = { ..., features = ["persistence-json"] }
```

Notes:
- Writes are atomic (write to a temp file, then rename).
- Not suitable for high write-frequency bots under concurrent load.
- Easy to inspect and modify by hand during development.

---

## SQLite Persistence

Production-ready. Uses WAL mode and prepared statements. Good for bots with hundreds of
users and moderate update rates.

```rust
use telegram_bot_ext::persistence::sqlite::SqlitePersistence;

let persistence = SqlitePersistence::open("bot.db")
    .expect("Failed to open SQLite database");

let app = ApplicationBuilder::new()
    .token(token)
    .persistence(Box::new(persistence))
    .build();
```

Enable it via the feature flag:

```toml
telegram-bot = { ..., features = ["persistence-sqlite"] }
```

The `open` call creates the database file and runs migrations to create the required
tables if they do not exist. Pass an absolute path for predictable file location in
production.

Notes:
- Suitable for single-process bots. For multi-instance deployments, use a centralised
  backend (Redis, PostgreSQL) and implement `BasePersistence` yourself.
- WAL mode allows concurrent readers while a write is in progress.

---

## Accessing Persistence Data from Handlers

The `Application` maintains in-memory maps that are synchronised with the backend.
Handlers read and write these maps via the `CallbackContext`:

```rust
async fn greet(update: Value, context: CallbackContext) -> Result<(), HandlerError> {
    let user_id = update["message"]["from"]["id"]
        .as_i64()
        .unwrap();

    // Read from user_data
    {
        let user_data = context.user_data().read().await;
        let count = user_data
            .get(&user_id)
            .and_then(|d| d.get("message_count"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        println!("User {} has sent {} messages", user_id, count);
    }

    // Write to user_data
    {
        let mut user_data = context.user_data().write().await;
        let entry = user_data.entry(user_id).or_default();
        let count = entry
            .get("message_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        entry.insert("message_count".into(), serde_json::json!(count + 1));
    }

    Ok(())
}
```

The `Application` automatically persists these changes at the end of each update cycle.

---

## Writing a Custom Backend

Implement `BasePersistence` for any storage system.

### Redis example (skeleton)

```rust
use telegram_bot_ext::persistence::base::{
    BasePersistence, PersistenceInput, PersistenceResult,
};
use telegram_bot_ext::utils::types::{CdcData, ConversationDict, ConversationKey, JsonMap};
use std::collections::HashMap;

#[derive(Debug)]
pub struct RedisPersistence {
    client: redis::Client,
    prefix: String,
}

impl RedisPersistence {
    pub fn new(url: &str, prefix: impl Into<String>) -> Self {
        Self {
            client: redis::Client::open(url).expect("valid Redis URL"),
            prefix: prefix.into(),
        }
    }
}

impl BasePersistence for RedisPersistence {
    async fn get_user_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> {
        // Fetch the hash from Redis, deserialise each field from JSON
        todo!()
    }

    async fn get_chat_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> {
        todo!()
    }

    async fn get_bot_data(&self) -> PersistenceResult<JsonMap> {
        todo!()
    }

    async fn get_callback_data(&self) -> PersistenceResult<Option<CdcData>> {
        Ok(None)
    }

    async fn get_conversations(&self, _name: &str) -> PersistenceResult<ConversationDict> {
        Ok(ConversationDict::new())
    }

    async fn update_user_data(
        &self,
        user_id: i64,
        data: &JsonMap,
    ) -> PersistenceResult<()> {
        // Serialise data to JSON and write to Redis hash field
        todo!()
    }

    async fn update_chat_data(
        &self,
        chat_id: i64,
        data: &JsonMap,
    ) -> PersistenceResult<()> {
        todo!()
    }

    async fn update_bot_data(&self, data: &JsonMap) -> PersistenceResult<()> {
        todo!()
    }

    async fn update_callback_data(&self, _data: &CdcData) -> PersistenceResult<()> {
        Ok(())
    }

    async fn update_conversation(
        &self,
        _name: &str,
        _key: &ConversationKey,
        _new_state: Option<&serde_json::Value>,
    ) -> PersistenceResult<()> {
        Ok(())
    }

    async fn drop_chat_data(&self, _chat_id: i64) -> PersistenceResult<()> {
        todo!()
    }

    async fn drop_user_data(&self, _user_id: i64) -> PersistenceResult<()> {
        todo!()
    }

    async fn flush(&self) -> PersistenceResult<()> {
        // Optionally flush any write buffers
        Ok(())
    }

    fn store_data(&self) -> PersistenceInput {
        PersistenceInput {
            bot_data: true,
            chat_data: true,
            user_data: true,
            callback_data: false, // skip callback cache for this backend
        }
    }

    fn update_interval(&self) -> f64 {
        30.0 // write every 30 seconds instead of the default 60
    }
}
```

### Wiring to Application

```rust
let persistence = RedisPersistence::new("redis://127.0.0.1/", "mybot");

let app = ApplicationBuilder::new()
    .token(token)
    .persistence(Box::new(persistence))
    .build();
```

The `Application` wraps the backend in a `DynPersistence` trait object automatically.
Your backend only needs to implement `BasePersistence`.

---

## Error Handling

```rust
pub enum PersistenceError {
    Serialization(serde_json::Error),
    Io(std::io::Error),
    Custom(String),
    Sqlite(rusqlite::Error),  // only with "persistence-sqlite" feature
}

pub type PersistenceResult<T> = Result<T, PersistenceError>;
```

When persistence operations fail, the `Application` logs the error at `WARN` level and
continues processing updates. Bot data is not lost (it remains in memory); only the
write to the backend failed. On the next update cycle or on `flush()`, another write will
be attempted.
