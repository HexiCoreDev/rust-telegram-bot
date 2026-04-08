//! SQLite-backed persistence.
//!
//! Gated behind `feature = "persistence-sqlite"`. Uses `rusqlite` with the
//! `bundled` feature so no external library is required at link time.
//!
//! Schema:
//! ```sql
//! CREATE TABLE IF NOT EXISTS user_data    (user_id INTEGER PRIMARY KEY, data TEXT NOT NULL);
//! CREATE TABLE IF NOT EXISTS chat_data    (chat_id INTEGER PRIMARY KEY, data TEXT NOT NULL);
//! CREATE TABLE IF NOT EXISTS bot_data     (id INTEGER PRIMARY KEY DEFAULT 1, data TEXT NOT NULL);
//! CREATE TABLE IF NOT EXISTS callback_data(id INTEGER PRIMARY KEY DEFAULT 1, data TEXT NOT NULL);
//! CREATE TABLE IF NOT EXISTS conversations(handler TEXT NOT NULL, key TEXT NOT NULL, state TEXT,
//!     PRIMARY KEY (handler, key));
//! ```

#![cfg(feature = "persistence-sqlite")]

use std::collections::HashMap;
use std::path::PathBuf;

use rusqlite::Connection;
use serde_json::Value;
use tokio::sync::Mutex;

use crate::persistence::base::{
    BasePersistence, PersistenceError, PersistenceInput, PersistenceResult,
};
use crate::utils::types::{CdcData, ConversationDict, ConversationKey, JsonMap};

/// Persistence backed by a single SQLite database file.
///
/// All writes are serialized through a `tokio::sync::Mutex` wrapping the
/// synchronous `rusqlite::Connection`.  Reads acquire the same mutex, so
/// there is no risk of SQLITE_BUSY errors from concurrent WAL writers.
#[derive(Debug)]
pub struct SqlitePersistence {
    conn: Mutex<Connection>,
    store_data: PersistenceInput,
    update_interval: f64,
}

impl SqlitePersistence {
    /// Open (or create) a SQLite database at `path` and initialise the schema.
    pub fn open(path: impl Into<PathBuf>) -> PersistenceResult<Self> {
        let path = path.into();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&path).map_err(PersistenceError::Sqlite)?;
        conn.execute_batch("PRAGMA journal_mode=WAL;")
            .map_err(PersistenceError::Sqlite)?;
        Self::init_tables(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
            store_data: PersistenceInput::default(),
            update_interval: 60.0,
        })
    }

    /// Open an in-memory database (useful for testing).
    pub fn in_memory() -> PersistenceResult<Self> {
        let conn = Connection::open_in_memory().map_err(PersistenceError::Sqlite)?;
        Self::init_tables(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
            store_data: PersistenceInput::default(),
            update_interval: 60.0,
        })
    }

    pub fn with_store_data(mut self, input: PersistenceInput) -> Self {
        self.store_data = input;
        self
    }

    pub fn with_update_interval(mut self, seconds: f64) -> Self {
        self.update_interval = seconds;
        self
    }

    fn init_tables(conn: &Connection) -> PersistenceResult<()> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS user_data (
                user_id INTEGER PRIMARY KEY,
                data TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS chat_data (
                chat_id INTEGER PRIMARY KEY,
                data TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS bot_data (
                id INTEGER PRIMARY KEY DEFAULT 1,
                data TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS callback_data (
                id INTEGER PRIMARY KEY DEFAULT 1,
                data TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS conversations (
                handler TEXT NOT NULL,
                key TEXT NOT NULL,
                state TEXT,
                PRIMARY KEY (handler, key)
            );",
        )
        .map_err(PersistenceError::Sqlite)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// BasePersistence implementation
// ---------------------------------------------------------------------------

impl BasePersistence for SqlitePersistence {
    async fn get_user_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare("SELECT user_id, data FROM user_data")
            .map_err(PersistenceError::Sqlite)?;
        let rows = stmt
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                let json: String = row.get(1)?;
                Ok((id, json))
            })
            .map_err(PersistenceError::Sqlite)?;

        let mut out = HashMap::new();
        for row in rows {
            let (id, json) = row.map_err(PersistenceError::Sqlite)?;
            let map: JsonMap = serde_json::from_str(&json)?;
            out.insert(id, map);
        }
        Ok(out)
    }

    async fn get_chat_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare("SELECT chat_id, data FROM chat_data")
            .map_err(PersistenceError::Sqlite)?;
        let rows = stmt
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                let json: String = row.get(1)?;
                Ok((id, json))
            })
            .map_err(PersistenceError::Sqlite)?;

        let mut out = HashMap::new();
        for row in rows {
            let (id, json) = row.map_err(PersistenceError::Sqlite)?;
            let map: JsonMap = serde_json::from_str(&json)?;
            out.insert(id, map);
        }
        Ok(out)
    }

    async fn get_bot_data(&self) -> PersistenceResult<JsonMap> {
        let conn = self.conn.lock().await;
        let result: Option<String> = conn
            .query_row("SELECT data FROM bot_data WHERE id = 1", [], |row| {
                row.get(0)
            })
            .ok();
        match result {
            Some(json) => Ok(serde_json::from_str(&json)?),
            None => Ok(JsonMap::new()),
        }
    }

    async fn get_callback_data(&self) -> PersistenceResult<Option<CdcData>> {
        let conn = self.conn.lock().await;
        let result: Option<String> = conn
            .query_row("SELECT data FROM callback_data WHERE id = 1", [], |row| {
                row.get(0)
            })
            .ok();
        match result {
            Some(json) => Ok(serde_json::from_str(&json)?),
            None => Ok(None),
        }
    }

    async fn get_conversations(&self, name: &str) -> PersistenceResult<ConversationDict> {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare("SELECT key, state FROM conversations WHERE handler = ?1")
            .map_err(PersistenceError::Sqlite)?;
        let rows = stmt
            .query_map([name], |row| {
                let key_json: String = row.get(0)?;
                let state_json: Option<String> = row.get(1)?;
                Ok((key_json, state_json))
            })
            .map_err(PersistenceError::Sqlite)?;

        let mut out = ConversationDict::new();
        for row in rows {
            let (key_json, state_json) = row.map_err(PersistenceError::Sqlite)?;
            let key: ConversationKey = serde_json::from_str(&key_json)?;
            let state: Option<Value> = match state_json {
                Some(s) => Some(serde_json::from_str(&s)?),
                None => None,
            };
            out.insert(key, state);
        }
        Ok(out)
    }

    async fn update_user_data(&self, user_id: i64, data: &JsonMap) -> PersistenceResult<()> {
        let json = serde_json::to_string(data)?;
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR REPLACE INTO user_data (user_id, data) VALUES (?1, ?2)",
            rusqlite::params![user_id, json],
        )
        .map_err(PersistenceError::Sqlite)?;
        Ok(())
    }

    async fn update_chat_data(&self, chat_id: i64, data: &JsonMap) -> PersistenceResult<()> {
        let json = serde_json::to_string(data)?;
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR REPLACE INTO chat_data (chat_id, data) VALUES (?1, ?2)",
            rusqlite::params![chat_id, json],
        )
        .map_err(PersistenceError::Sqlite)?;
        Ok(())
    }

    async fn update_bot_data(&self, data: &JsonMap) -> PersistenceResult<()> {
        let json = serde_json::to_string(data)?;
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR REPLACE INTO bot_data (id, data) VALUES (1, ?1)",
            [&json],
        )
        .map_err(PersistenceError::Sqlite)?;
        Ok(())
    }

    async fn update_callback_data(&self, data: &CdcData) -> PersistenceResult<()> {
        let json = serde_json::to_string(data)?;
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR REPLACE INTO callback_data (id, data) VALUES (1, ?1)",
            [&json],
        )
        .map_err(PersistenceError::Sqlite)?;
        Ok(())
    }

    async fn update_conversation(
        &self,
        name: &str,
        key: &ConversationKey,
        new_state: Option<&Value>,
    ) -> PersistenceResult<()> {
        let key_json = serde_json::to_string(key)?;
        let state_json: Option<String> = new_state.map(|v| serde_json::to_string(v)).transpose()?;
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR REPLACE INTO conversations (handler, key, state) VALUES (?1, ?2, ?3)",
            rusqlite::params![name, key_json, state_json],
        )
        .map_err(PersistenceError::Sqlite)?;
        Ok(())
    }

    async fn drop_chat_data(&self, chat_id: i64) -> PersistenceResult<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM chat_data WHERE chat_id = ?1", [chat_id])
            .map_err(PersistenceError::Sqlite)?;
        Ok(())
    }

    async fn drop_user_data(&self, user_id: i64) -> PersistenceResult<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM user_data WHERE user_id = ?1", [user_id])
            .map_err(PersistenceError::Sqlite)?;
        Ok(())
    }

    async fn flush(&self) -> PersistenceResult<()> {
        // SQLite writes are immediate; nothing extra to flush.
        // A WAL checkpoint could be done here but rusqlite handles that
        // automatically on close.
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

    #[tokio::test]
    async fn in_memory_roundtrip() {
        let p = SqlitePersistence::in_memory().unwrap();
        let mut m = JsonMap::new();
        m.insert("a".into(), Value::Bool(true));
        p.update_bot_data(&m).await.unwrap();
        let loaded = p.get_bot_data().await.unwrap();
        assert_eq!(loaded.get("a"), Some(&Value::Bool(true)));
    }

    #[tokio::test]
    async fn user_data_crud() {
        let p = SqlitePersistence::in_memory().unwrap();
        let mut data = JsonMap::new();
        data.insert("name".into(), Value::String("bob".into()));
        p.update_user_data(7, &data).await.unwrap();

        let loaded = p.get_user_data().await.unwrap();
        assert_eq!(loaded.get(&7), Some(&data));

        p.drop_user_data(7).await.unwrap();
        let loaded = p.get_user_data().await.unwrap();
        assert!(!loaded.contains_key(&7));
    }

    #[tokio::test]
    async fn conversations_roundtrip() {
        use crate::utils::types::ConversationKeyPart;

        let p = SqlitePersistence::in_memory().unwrap();
        let key = vec![ConversationKeyPart::Int(1), ConversationKeyPart::Int(2)];
        let state = Value::Number(42.into());
        p.update_conversation("handler1", &key, Some(&state))
            .await
            .unwrap();

        let conv = p.get_conversations("handler1").await.unwrap();
        assert_eq!(conv.get(&key), Some(&Some(state)));
    }

    #[tokio::test]
    async fn callback_data_roundtrip() {
        let p = SqlitePersistence::in_memory().unwrap();
        let cdc: CdcData = (
            vec![("uuid1".into(), 1.0, HashMap::new())],
            HashMap::from([("a".into(), "b".into())]),
        );
        p.update_callback_data(&cdc).await.unwrap();
        let loaded = p.get_callback_data().await.unwrap();
        assert_eq!(loaded, Some(cdc));
    }
}
