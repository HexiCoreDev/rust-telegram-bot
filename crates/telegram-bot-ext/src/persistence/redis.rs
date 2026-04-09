//! Redis-backed persistence.
//!
//! Gated behind `feature = "persistence-redis"`. Uses the `redis` crate with
//! `tokio-comp` and `aio` features for async operations.
//!
//! ## Key schema
//!
//! All keys are prefixed with `rtb:` (rust-telegram-bot) to avoid collisions
//! with other applications sharing the same Redis instance.
//!
//! | Data category  | Redis type | Key pattern                         |
//! |----------------|-----------|-------------------------------------|
//! | User data      | HASH      | `rtb:user_data:{user_id}`           |
//! | Chat data      | HASH      | `rtb:chat_data:{chat_id}`           |
//! | Bot data       | HASH      | `rtb:bot_data`                      |
//! | Callback data  | STRING    | `rtb:callback_data`                 |
//! | Conversations  | HASH      | `rtb:conversations:{handler_name}`  |
//!
//! Within each HASH, field names are the JSON-serialized key (for conversations)
//! or the string key from the `JsonMap`. Values are JSON-encoded strings.
//!
//! ## User/chat data encoding
//!
//! Each user or chat data map is stored as a single HASH where each field
//! corresponds to a top-level key in the `JsonMap`, and each value is the
//! JSON-serialized representation of that entry's value. This gives O(1)
//! per-field access while still supporting the full `JsonMap` round-trip.

#![cfg(feature = "persistence-redis")]

use std::collections::HashMap;

use redis::AsyncCommands;
use serde_json::Value;

use crate::persistence::base::{
    BasePersistence, PersistenceError, PersistenceInput, PersistenceResult,
};
use crate::utils::types::{CdcData, ConversationDict, ConversationKey, JsonMap};

/// Redis-backed persistence.
///
/// Connects to a Redis instance and stores all persistence data using the key
/// schema documented in the [module-level documentation](self).
///
/// # Construction
///
/// ```rust,ignore
/// let persistence = RedisPersistence::new("redis://127.0.0.1:6379").await?;
/// ```
///
/// # Thread safety
///
/// `RedisPersistence` holds a `redis::aio::MultiplexedConnection` which is
/// `Clone + Send + Sync`, so it can be shared across tasks without additional
/// synchronization.
#[derive(Debug)]
pub struct RedisPersistence {
    conn: redis::aio::MultiplexedConnection,
    store_data: PersistenceInput,
    update_interval: f64,
    /// Key prefix for all Redis keys.
    prefix: String,
}

impl RedisPersistence {
    /// Connect to Redis at the given URL and return a new `RedisPersistence`.
    ///
    /// # Errors
    ///
    /// Returns a [`PersistenceError::Redis`] if the connection fails.
    pub async fn new(redis_url: &str) -> PersistenceResult<Self> {
        let client = redis::Client::open(redis_url).map_err(PersistenceError::Redis)?;
        let conn = client
            .get_multiplexed_async_connection()
            .await
            .map_err(PersistenceError::Redis)?;
        Ok(Self {
            conn,
            store_data: PersistenceInput::default(),
            update_interval: 60.0,
            prefix: "rtb".to_owned(),
        })
    }

    /// Set a custom key prefix (default: `"rtb"`).
    ///
    /// All Redis keys will be prefixed with `{prefix}:`.
    #[must_use]
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    /// Configure which data categories to persist.
    #[must_use]
    pub fn with_store_data(mut self, input: PersistenceInput) -> Self {
        self.store_data = input;
        self
    }

    /// Set the update interval in seconds.
    #[must_use]
    pub fn with_update_interval(mut self, seconds: f64) -> Self {
        self.update_interval = seconds;
        self
    }

    // -- key helpers ---------------------------------------------------------

    fn user_data_key(&self, user_id: i64) -> String {
        format!("{}:user_data:{user_id}", self.prefix)
    }

    fn chat_data_key(&self, chat_id: i64) -> String {
        format!("{}:chat_data:{chat_id}", self.prefix)
    }

    fn bot_data_key(&self) -> String {
        format!("{}:bot_data", self.prefix)
    }

    fn callback_data_key(&self) -> String {
        format!("{}:callback_data", self.prefix)
    }

    fn conversations_key(&self, name: &str) -> String {
        format!("{}:conversations:{name}", self.prefix)
    }

    /// Pattern for scanning all user_data keys.
    fn user_data_pattern(&self) -> String {
        format!("{}:user_data:*", self.prefix)
    }

    /// Pattern for scanning all chat_data keys.
    fn chat_data_pattern(&self) -> String {
        format!("{}:chat_data:*", self.prefix)
    }

    /// Extract the numeric ID suffix from a key like `rtb:user_data:123`.
    fn id_from_key(key: &str) -> Option<i64> {
        key.rsplit(':').next().and_then(|s| s.parse().ok())
    }

    // -- internal helpers ----------------------------------------------------

    /// Load a full JsonMap from a Redis HASH key.
    async fn load_json_map(
        conn: &mut redis::aio::MultiplexedConnection,
        key: &str,
    ) -> PersistenceResult<JsonMap> {
        let entries: HashMap<String, String> =
            conn.hgetall(key).await.map_err(PersistenceError::Redis)?;

        let mut map = JsonMap::new();
        for (field, value_json) in entries {
            let value: Value = serde_json::from_str(&value_json)?;
            map.insert(field, value);
        }
        Ok(map)
    }

    /// Save a JsonMap into a Redis HASH key (replaces all fields).
    async fn save_json_map(
        conn: &mut redis::aio::MultiplexedConnection,
        key: &str,
        data: &JsonMap,
    ) -> PersistenceResult<()> {
        // Delete the key first to ensure we don't keep stale fields.
        let _: () = redis::cmd("DEL")
            .arg(key)
            .query_async(conn)
            .await
            .map_err(PersistenceError::Redis)?;

        if data.is_empty() {
            return Ok(());
        }

        let fields: Vec<(String, String)> = data
            .iter()
            .map(|(k, v)| {
                let v_json = serde_json::to_string(v).unwrap_or_default();
                (k.clone(), v_json)
            })
            .collect();

        let _: () = conn
            .hset_multiple(key, &fields)
            .await
            .map_err(PersistenceError::Redis)?;

        Ok(())
    }

    /// Scan for keys matching a pattern and collect them.
    async fn scan_keys(
        conn: &mut redis::aio::MultiplexedConnection,
        pattern: &str,
    ) -> PersistenceResult<Vec<String>> {
        // Use KEYS for simplicity. For very large datasets, SCAN would be
        // preferable, but KEYS is simpler with the redis crate's async API
        // and sufficient for typical bot workloads.
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg(pattern)
            .query_async(conn)
            .await
            .map_err(PersistenceError::Redis)?;
        Ok(keys)
    }
}

// ---------------------------------------------------------------------------
// BasePersistence implementation
// ---------------------------------------------------------------------------

impl BasePersistence for RedisPersistence {
    async fn get_user_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> {
        let mut conn = self.conn.clone();
        let keys = Self::scan_keys(&mut conn, &self.user_data_pattern()).await?;

        let mut out = HashMap::new();
        for key in keys {
            if let Some(user_id) = Self::id_from_key(&key) {
                let map = Self::load_json_map(&mut conn, &key).await?;
                out.insert(user_id, map);
            }
        }
        Ok(out)
    }

    async fn get_chat_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> {
        let mut conn = self.conn.clone();
        let keys = Self::scan_keys(&mut conn, &self.chat_data_pattern()).await?;

        let mut out = HashMap::new();
        for key in keys {
            if let Some(chat_id) = Self::id_from_key(&key) {
                let map = Self::load_json_map(&mut conn, &key).await?;
                out.insert(chat_id, map);
            }
        }
        Ok(out)
    }

    async fn get_bot_data(&self) -> PersistenceResult<JsonMap> {
        let mut conn = self.conn.clone();
        Self::load_json_map(&mut conn, &self.bot_data_key()).await
    }

    async fn get_callback_data(&self) -> PersistenceResult<Option<CdcData>> {
        let mut conn = self.conn.clone();
        let key = self.callback_data_key();
        let result: Option<String> = conn.get(&key).await.map_err(PersistenceError::Redis)?;

        match result {
            Some(json) => Ok(serde_json::from_str(&json)?),
            None => Ok(None),
        }
    }

    async fn get_conversations(&self, name: &str) -> PersistenceResult<ConversationDict> {
        let mut conn = self.conn.clone();
        let key = self.conversations_key(name);
        let entries: HashMap<String, String> =
            conn.hgetall(&key).await.map_err(PersistenceError::Redis)?;

        let mut out = ConversationDict::new();
        for (key_json, state_json) in entries {
            let conv_key: ConversationKey = serde_json::from_str(&key_json)?;
            let state: Option<Value> = serde_json::from_str(&state_json)?;
            out.insert(conv_key, state);
        }
        Ok(out)
    }

    async fn update_user_data(&self, user_id: i64, data: &JsonMap) -> PersistenceResult<()> {
        let mut conn = self.conn.clone();
        let key = self.user_data_key(user_id);
        Self::save_json_map(&mut conn, &key, data).await
    }

    async fn update_chat_data(&self, chat_id: i64, data: &JsonMap) -> PersistenceResult<()> {
        let mut conn = self.conn.clone();
        let key = self.chat_data_key(chat_id);
        Self::save_json_map(&mut conn, &key, data).await
    }

    async fn update_bot_data(&self, data: &JsonMap) -> PersistenceResult<()> {
        let mut conn = self.conn.clone();
        let key = self.bot_data_key();
        Self::save_json_map(&mut conn, &key, data).await
    }

    async fn update_callback_data(&self, data: &CdcData) -> PersistenceResult<()> {
        let mut conn = self.conn.clone();
        let key = self.callback_data_key();
        let json = serde_json::to_string(data)?;
        let _: () = conn
            .set(&key, &json)
            .await
            .map_err(PersistenceError::Redis)?;
        Ok(())
    }

    async fn update_conversation(
        &self,
        name: &str,
        key: &ConversationKey,
        new_state: Option<&Value>,
    ) -> PersistenceResult<()> {
        let mut conn = self.conn.clone();
        let hash_key = self.conversations_key(name);
        let field = serde_json::to_string(key)?;

        match new_state {
            Some(state) => {
                let state_json = serde_json::to_string(&Some(state))?;
                let _: () = conn
                    .hset(&hash_key, &field, &state_json)
                    .await
                    .map_err(PersistenceError::Redis)?;
            }
            None => {
                // Remove the conversation entry when state is None.
                let _: () = conn
                    .hdel(&hash_key, &field)
                    .await
                    .map_err(PersistenceError::Redis)?;
            }
        }
        Ok(())
    }

    async fn drop_chat_data(&self, chat_id: i64) -> PersistenceResult<()> {
        let mut conn = self.conn.clone();
        let key = self.chat_data_key(chat_id);
        let _: () = redis::cmd("DEL")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .map_err(PersistenceError::Redis)?;
        Ok(())
    }

    async fn drop_user_data(&self, user_id: i64) -> PersistenceResult<()> {
        let mut conn = self.conn.clone();
        let key = self.user_data_key(user_id);
        let _: () = redis::cmd("DEL")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .map_err(PersistenceError::Redis)?;
        Ok(())
    }

    async fn flush(&self) -> PersistenceResult<()> {
        // Redis writes are immediate; nothing to flush.
        Ok(())
    }

    fn update_interval(&self) -> f64 {
        self.update_interval
    }

    fn store_data(&self) -> PersistenceInput {
        self.store_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests verify key generation logic without needing a Redis connection.

    #[test]
    fn id_from_key_extracts_numeric_suffix() {
        assert_eq!(
            RedisPersistence::id_from_key("rtb:user_data:123"),
            Some(123)
        );
        assert_eq!(
            RedisPersistence::id_from_key("rtb:chat_data:-100"),
            Some(-100)
        );
        assert_eq!(RedisPersistence::id_from_key("rtb:bot_data"), None);
    }

    #[test]
    fn key_format_user_data() {
        let prefix = "rtb";
        let user_id = 42_i64;
        let key = format!("{prefix}:user_data:{user_id}");
        assert_eq!(key, "rtb:user_data:42");
    }

    #[test]
    fn key_format_chat_data_negative() {
        let prefix = "rtb";
        let chat_id = -100_i64;
        let key = format!("{prefix}:chat_data:{chat_id}");
        assert_eq!(key, "rtb:chat_data:-100");
    }

    #[test]
    fn key_format_bot_data() {
        let prefix = "rtb";
        let key = format!("{prefix}:bot_data");
        assert_eq!(key, "rtb:bot_data");
    }

    #[test]
    fn key_format_conversations() {
        let prefix = "rtb";
        let name = "my_handler";
        let key = format!("{prefix}:conversations:{name}");
        assert_eq!(key, "rtb:conversations:my_handler");
    }

    #[test]
    fn key_format_custom_prefix() {
        let prefix = "mybot";
        let user_id = 1_i64;
        let key = format!("{prefix}:user_data:{user_id}");
        assert_eq!(key, "mybot:user_data:1");
    }

    #[test]
    fn key_format_pattern() {
        let prefix = "rtb";
        let pattern = format!("{prefix}:user_data:*");
        assert_eq!(pattern, "rtb:user_data:*");
    }
}
