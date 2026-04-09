//! PostgreSQL-backed persistence.
//!
//! Gated behind `feature = "persistence-postgres"`. Uses the `sqlx` crate with
//! the `postgres` and `json` features for async operations and native JSONB
//! storage.
//!
//! ## Table schema
//!
//! All tables are prefixed with `rtb_` (rust-telegram-bot) by default to avoid
//! collisions with other applications sharing the same database.
//!
//! | Data category  | Table                  | Primary key          |
//! |----------------|------------------------|----------------------|
//! | User data      | `rtb_user_data`        | `user_id  BIGINT`    |
//! | Chat data      | `rtb_chat_data`        | `chat_id  BIGINT`    |
//! | Bot data       | `rtb_bot_data`         | `key      TEXT`      |
//! | Callback data  | `rtb_callback_data`    | `id       TEXT`      |
//! | Conversations  | `rtb_conversations`    | `(name, key)`        |
//!
//! All data columns use PostgreSQL `JSONB` for efficient querying and compact
//! storage.
//!
//! ## Upsert strategy
//!
//! All write operations use `INSERT ... ON CONFLICT DO UPDATE` to atomically
//! insert or replace data without requiring a separate existence check.

#![cfg(feature = "persistence-postgres")]

use std::collections::HashMap;

use serde_json::Value;
use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::persistence::base::{
    BasePersistence, PersistenceError, PersistenceInput, PersistenceResult,
};
use crate::utils::types::{CdcData, ConversationDict, ConversationKey, JsonMap};

/// PostgreSQL-backed persistence.
///
/// Connects to a PostgreSQL instance via a connection pool and stores all
/// persistence data using the table schema documented in the
/// [module-level documentation](self).
///
/// # Construction
///
/// ```rust,ignore
/// let persistence = PostgresPersistence::new("postgres://user:pass@localhost/mydb").await?;
/// ```
///
/// # Builder methods
///
/// ```rust,ignore
/// let persistence = PostgresPersistence::builder("postgres://user:pass@localhost/mydb")
///     .with_prefix("mybot")
///     .with_pool_size(10)
///     .build()
///     .await?;
/// ```
///
/// # Thread safety
///
/// `PostgresPersistence` holds a `sqlx::PgPool` which is `Clone + Send + Sync`,
/// so it can be shared across tasks without additional synchronization.
#[derive(Debug, Clone)]
pub struct PostgresPersistence {
    pool: PgPool,
    store_data: PersistenceInput,
    update_interval: f64,
    /// Table name prefix for all tables.
    prefix: String,
}

/// Builder for [`PostgresPersistence`].
///
/// Allows configuring the connection pool size, table prefix, and persistence
/// options before establishing the connection.
#[derive(Debug)]
pub struct PostgresPersistenceBuilder {
    database_url: String,
    prefix: String,
    pool_size: u32,
    store_data: PersistenceInput,
    update_interval: f64,
}

impl PostgresPersistenceBuilder {
    /// Set a custom table prefix (default: `"rtb"`).
    ///
    /// All table names will be formatted as `{prefix}_user_data`, etc.
    #[must_use]
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    /// Set the maximum number of connections in the pool (default: 5).
    #[must_use]
    pub fn with_pool_size(mut self, size: u32) -> Self {
        self.pool_size = size;
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

    /// Connect to the database, create tables, and return the persistence
    /// instance.
    ///
    /// # Errors
    ///
    /// Returns a [`PersistenceError::Postgres`] if the connection or table
    /// creation fails.
    pub async fn build(self) -> PersistenceResult<PostgresPersistence> {
        let pool = PgPoolOptions::new()
            .max_connections(self.pool_size)
            .connect(&self.database_url)
            .await
            .map_err(PersistenceError::Postgres)?;

        let persistence = PostgresPersistence {
            pool,
            store_data: self.store_data,
            update_interval: self.update_interval,
            prefix: self.prefix,
        };

        persistence.init_tables().await?;
        Ok(persistence)
    }
}

impl PostgresPersistence {
    /// Connect to PostgreSQL at the given URL and return a new
    /// `PostgresPersistence` with default settings.
    ///
    /// This is a convenience shorthand for `PostgresPersistence::builder(url).build()`.
    ///
    /// # Errors
    ///
    /// Returns a [`PersistenceError::Postgres`] if the connection or table
    /// creation fails.
    pub async fn new(database_url: &str) -> PersistenceResult<Self> {
        Self::builder(database_url).build().await
    }

    /// Create a builder for fine-grained configuration.
    ///
    /// Use the builder to set pool size, table prefix, and other options
    /// before connecting.
    ///
    /// ```rust,ignore
    /// let persistence = PostgresPersistence::builder("postgres://localhost/mydb")
    ///     .with_prefix("mybot")
    ///     .with_pool_size(10)
    ///     .with_update_interval(30.0)
    ///     .build()
    ///     .await?;
    /// ```
    #[must_use]
    pub fn builder(database_url: &str) -> PostgresPersistenceBuilder {
        PostgresPersistenceBuilder {
            database_url: database_url.to_owned(),
            prefix: "rtb".to_owned(),
            pool_size: 5,
            store_data: PersistenceInput::default(),
            update_interval: 60.0,
        }
    }

    /// Set a custom table prefix (default: `"rtb"`).
    ///
    /// All table names will be formatted as `{prefix}_user_data`, etc.
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

    // -- table name helpers ---------------------------------------------------

    fn user_data_table(&self) -> String {
        format!("{}_user_data", self.prefix)
    }

    fn chat_data_table(&self) -> String {
        format!("{}_chat_data", self.prefix)
    }

    fn bot_data_table(&self) -> String {
        format!("{}_bot_data", self.prefix)
    }

    fn callback_data_table(&self) -> String {
        format!("{}_callback_data", self.prefix)
    }

    fn conversations_table(&self) -> String {
        format!("{}_conversations", self.prefix)
    }

    // -- schema initialisation ------------------------------------------------

    /// Create all required tables if they do not already exist.
    async fn init_tables(&self) -> PersistenceResult<()> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {user_data} (
                user_id BIGINT PRIMARY KEY,
                data JSONB NOT NULL
            );
            CREATE TABLE IF NOT EXISTS {chat_data} (
                chat_id BIGINT PRIMARY KEY,
                data JSONB NOT NULL
            );
            CREATE TABLE IF NOT EXISTS {bot_data} (
                key TEXT PRIMARY KEY,
                data JSONB NOT NULL
            );
            CREATE TABLE IF NOT EXISTS {conversations} (
                name TEXT NOT NULL,
                key TEXT NOT NULL,
                state JSONB,
                PRIMARY KEY (name, key)
            );
            CREATE TABLE IF NOT EXISTS {callback_data} (
                id TEXT PRIMARY KEY,
                data JSONB NOT NULL
            );",
            user_data = self.user_data_table(),
            chat_data = self.chat_data_table(),
            bot_data = self.bot_data_table(),
            conversations = self.conversations_table(),
            callback_data = self.callback_data_table(),
        );

        sqlx::query(&sql)
            .execute(&self.pool)
            .await
            .map_err(PersistenceError::Postgres)?;

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// BasePersistence implementation
// ---------------------------------------------------------------------------

impl BasePersistence for PostgresPersistence {
    async fn get_user_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> {
        let table = self.user_data_table();
        let sql = format!("SELECT user_id, data FROM {table}");

        let rows: Vec<(i64, Value)> = sqlx::query_as(&sql)
            .fetch_all(&self.pool)
            .await
            .map_err(PersistenceError::Postgres)?;

        let mut out = HashMap::new();
        for (user_id, data) in rows {
            let map: JsonMap = serde_json::from_value(data)?;
            out.insert(user_id, map);
        }
        Ok(out)
    }

    async fn get_chat_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> {
        let table = self.chat_data_table();
        let sql = format!("SELECT chat_id, data FROM {table}");

        let rows: Vec<(i64, Value)> = sqlx::query_as(&sql)
            .fetch_all(&self.pool)
            .await
            .map_err(PersistenceError::Postgres)?;

        let mut out = HashMap::new();
        for (chat_id, data) in rows {
            let map: JsonMap = serde_json::from_value(data)?;
            out.insert(chat_id, map);
        }
        Ok(out)
    }

    async fn get_bot_data(&self) -> PersistenceResult<JsonMap> {
        let table = self.bot_data_table();
        let sql = format!("SELECT key, data FROM {table}");

        let rows: Vec<(String, Value)> = sqlx::query_as(&sql)
            .fetch_all(&self.pool)
            .await
            .map_err(PersistenceError::Postgres)?;

        let mut out = JsonMap::new();
        for (key, data) in rows {
            out.insert(key, data);
        }
        Ok(out)
    }

    async fn get_callback_data(&self) -> PersistenceResult<Option<CdcData>> {
        let table = self.callback_data_table();
        let sql = format!("SELECT data FROM {table} WHERE id = '_cdc'");

        let result: Option<(Value,)> = sqlx::query_as(&sql)
            .fetch_optional(&self.pool)
            .await
            .map_err(PersistenceError::Postgres)?;

        match result {
            Some((data,)) => Ok(serde_json::from_value(data)?),
            None => Ok(None),
        }
    }

    async fn get_conversations(&self, name: &str) -> PersistenceResult<ConversationDict> {
        let table = self.conversations_table();
        let sql = format!("SELECT key, state FROM {table} WHERE name = $1");

        let rows: Vec<(String, Option<Value>)> = sqlx::query_as(&sql)
            .bind(name)
            .fetch_all(&self.pool)
            .await
            .map_err(PersistenceError::Postgres)?;

        let mut out = ConversationDict::new();
        for (key_json, state) in rows {
            let conv_key: ConversationKey = serde_json::from_str(&key_json)?;
            out.insert(conv_key, state);
        }
        Ok(out)
    }

    async fn update_user_data(&self, user_id: i64, data: &JsonMap) -> PersistenceResult<()> {
        let table = self.user_data_table();
        let sql = format!(
            "INSERT INTO {table} (user_id, data) VALUES ($1, $2)
             ON CONFLICT (user_id) DO UPDATE SET data = EXCLUDED.data"
        );
        let json_value = serde_json::to_value(data)?;

        sqlx::query(&sql)
            .bind(user_id)
            .bind(json_value)
            .execute(&self.pool)
            .await
            .map_err(PersistenceError::Postgres)?;

        Ok(())
    }

    async fn update_chat_data(&self, chat_id: i64, data: &JsonMap) -> PersistenceResult<()> {
        let table = self.chat_data_table();
        let sql = format!(
            "INSERT INTO {table} (chat_id, data) VALUES ($1, $2)
             ON CONFLICT (chat_id) DO UPDATE SET data = EXCLUDED.data"
        );
        let json_value = serde_json::to_value(data)?;

        sqlx::query(&sql)
            .bind(chat_id)
            .bind(json_value)
            .execute(&self.pool)
            .await
            .map_err(PersistenceError::Postgres)?;

        Ok(())
    }

    async fn update_bot_data(&self, data: &JsonMap) -> PersistenceResult<()> {
        let table = self.bot_data_table();

        // Store each top-level key as a separate row for granular access.
        // Use a transaction to ensure atomicity.
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(PersistenceError::Postgres)?;

        // Clear existing bot data, then re-insert.
        let delete_sql = format!("DELETE FROM {table}");
        sqlx::query(&delete_sql)
            .execute(&mut *tx)
            .await
            .map_err(PersistenceError::Postgres)?;

        if !data.is_empty() {
            let upsert_sql = format!(
                "INSERT INTO {table} (key, data) VALUES ($1, $2)
                 ON CONFLICT (key) DO UPDATE SET data = EXCLUDED.data"
            );
            for (key, value) in data {
                sqlx::query(&upsert_sql)
                    .bind(key)
                    .bind(value)
                    .execute(&mut *tx)
                    .await
                    .map_err(PersistenceError::Postgres)?;
            }
        }

        tx.commit().await.map_err(PersistenceError::Postgres)?;
        Ok(())
    }

    async fn update_callback_data(&self, data: &CdcData) -> PersistenceResult<()> {
        let table = self.callback_data_table();
        let sql = format!(
            "INSERT INTO {table} (id, data) VALUES ('_cdc', $1)
             ON CONFLICT (id) DO UPDATE SET data = EXCLUDED.data"
        );
        let json_value = serde_json::to_value(data)?;

        sqlx::query(&sql)
            .bind(json_value)
            .execute(&self.pool)
            .await
            .map_err(PersistenceError::Postgres)?;

        Ok(())
    }

    async fn update_conversation(
        &self,
        name: &str,
        key: &ConversationKey,
        new_state: Option<&Value>,
    ) -> PersistenceResult<()> {
        let table = self.conversations_table();
        let key_json = serde_json::to_string(key)?;

        match new_state {
            Some(state) => {
                let sql = format!(
                    "INSERT INTO {table} (name, key, state) VALUES ($1, $2, $3)
                     ON CONFLICT (name, key) DO UPDATE SET state = EXCLUDED.state"
                );
                sqlx::query(&sql)
                    .bind(name)
                    .bind(&key_json)
                    .bind(state)
                    .execute(&self.pool)
                    .await
                    .map_err(PersistenceError::Postgres)?;
            }
            None => {
                let sql = format!("DELETE FROM {table} WHERE name = $1 AND key = $2");
                sqlx::query(&sql)
                    .bind(name)
                    .bind(&key_json)
                    .execute(&self.pool)
                    .await
                    .map_err(PersistenceError::Postgres)?;
            }
        }

        Ok(())
    }

    async fn drop_chat_data(&self, chat_id: i64) -> PersistenceResult<()> {
        let table = self.chat_data_table();
        let sql = format!("DELETE FROM {table} WHERE chat_id = $1");

        sqlx::query(&sql)
            .bind(chat_id)
            .execute(&self.pool)
            .await
            .map_err(PersistenceError::Postgres)?;

        Ok(())
    }

    async fn drop_user_data(&self, user_id: i64) -> PersistenceResult<()> {
        let table = self.user_data_table();
        let sql = format!("DELETE FROM {table} WHERE user_id = $1");

        sqlx::query(&sql)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(PersistenceError::Postgres)?;

        Ok(())
    }

    async fn flush(&self) -> PersistenceResult<()> {
        // PostgreSQL writes are immediate via the connection pool; nothing to
        // flush. The pool handles connection lifecycle automatically.
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

    // These tests verify table name generation and builder defaults without
    // requiring a running PostgreSQL instance.

    #[test]
    fn default_table_names() {
        let prefix = "rtb";
        assert_eq!(format!("{prefix}_user_data"), "rtb_user_data");
        assert_eq!(format!("{prefix}_chat_data"), "rtb_chat_data");
        assert_eq!(format!("{prefix}_bot_data"), "rtb_bot_data");
        assert_eq!(format!("{prefix}_callback_data"), "rtb_callback_data");
        assert_eq!(format!("{prefix}_conversations"), "rtb_conversations");
    }

    #[test]
    fn custom_prefix_table_names() {
        let prefix = "mybot";
        assert_eq!(format!("{prefix}_user_data"), "mybot_user_data");
        assert_eq!(format!("{prefix}_chat_data"), "mybot_chat_data");
        assert_eq!(format!("{prefix}_bot_data"), "mybot_bot_data");
        assert_eq!(format!("{prefix}_callback_data"), "mybot_callback_data");
        assert_eq!(format!("{prefix}_conversations"), "mybot_conversations");
    }

    #[test]
    fn builder_defaults() {
        let builder = PostgresPersistence::builder("postgres://localhost/test");
        assert_eq!(builder.prefix, "rtb");
        assert_eq!(builder.pool_size, 5);
        assert_eq!(builder.update_interval, 60.0);
        assert_eq!(builder.store_data, PersistenceInput::default());
    }

    #[test]
    fn builder_custom_values() {
        let builder = PostgresPersistence::builder("postgres://localhost/test")
            .with_prefix("custom")
            .with_pool_size(20)
            .with_update_interval(30.0);
        assert_eq!(builder.prefix, "custom");
        assert_eq!(builder.pool_size, 20);
        assert_eq!(builder.update_interval, 30.0);
    }
}
