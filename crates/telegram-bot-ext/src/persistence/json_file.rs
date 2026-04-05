//! JSON-file persistence back-end.
//!
//! This is the Rust equivalent of Python's `PicklePersistence`, but uses JSON
//! instead of pickle for a portable, human-readable format.
//!
//! Two modes are supported:
//! - **single file**: all data is stored in one JSON file.
//! - **multi file**: each data category gets its own file
//!   (`<prefix>_user_data.json`, `<prefix>_chat_data.json`, etc.).
//!
//! Gated on `feature = "persistence-json"`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::RwLock;

use crate::persistence::base::{
    BasePersistence, PersistenceError, PersistenceInput, PersistenceResult,
};
use crate::utils::types::{CdcData, ConversationDict, ConversationKey, JsonMap};

// ---------------------------------------------------------------------------
// On-disk schema
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Serialize, Deserialize)]
struct AllData {
    #[serde(default)]
    user_data: HashMap<String, JsonMap>,
    #[serde(default)]
    chat_data: HashMap<String, JsonMap>,
    #[serde(default)]
    bot_data: JsonMap,
    #[serde(default)]
    callback_data: Option<CdcData>,
    #[serde(default)]
    conversations: HashMap<String, HashMap<String, Option<Value>>>,
}

// ---------------------------------------------------------------------------
// In-memory state
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct FileState {
    user_data: HashMap<i64, JsonMap>,
    chat_data: HashMap<i64, JsonMap>,
    bot_data: JsonMap,
    callback_data: Option<CdcData>,
    conversations: HashMap<String, ConversationDict>,
    loaded: bool,
}

// ---------------------------------------------------------------------------
// JsonFilePersistence
// ---------------------------------------------------------------------------

/// Stores all persistence data as JSON on disk.
#[derive(Debug)]
pub struct JsonFilePersistence {
    filepath: PathBuf,
    single_file: bool,
    on_flush: bool,
    store_data: PersistenceInput,
    update_interval: f64,
    state: RwLock<FileState>,
}

impl JsonFilePersistence {
    /// Create a new JSON-file persistence back-end.
    ///
    /// - `filepath`: path (without extension) used as the base name.
    /// - `single_file`: if `true`, store everything in `<filepath>.json`;
    ///   otherwise use `<filepath>_user_data.json`, etc.
    /// - `on_flush`: if `true`, writes only happen when `flush()` is called;
    ///   otherwise every mutation triggers a disk write.
    pub fn new(filepath: impl Into<PathBuf>, single_file: bool, on_flush: bool) -> Self {
        Self {
            filepath: filepath.into(),
            single_file,
            on_flush,
            store_data: PersistenceInput::default(),
            update_interval: 60.0,
            state: RwLock::new(FileState::default()),
        }
    }

    pub fn with_store_data(mut self, input: PersistenceInput) -> Self {
        self.store_data = input;
        self
    }

    pub fn with_update_interval(mut self, seconds: f64) -> Self {
        self.update_interval = seconds;
        self
    }

    // -- internal I/O ---------------------------------------------------------

    fn single_path(&self) -> PathBuf {
        self.filepath.with_extension("json")
    }

    fn category_path(&self, suffix: &str) -> PathBuf {
        let base = self.filepath.as_os_str().to_string_lossy();
        PathBuf::from(format!("{base}_{suffix}.json"))
    }

    /// Load all data from disk into `state`. Idempotent (skips if already
    /// loaded).
    async fn ensure_loaded(&self) -> PersistenceResult<()> {
        let needs_load = { !self.state.read().await.loaded };
        if !needs_load {
            return Ok(());
        }
        let mut guard = self.state.write().await;
        if guard.loaded {
            return Ok(());
        }
        if self.single_file {
            self.load_single_file(&mut guard)?;
        } else {
            self.load_multi_file(&mut guard)?;
        }
        guard.loaded = true;
        Ok(())
    }

    fn load_single_file(&self, state: &mut FileState) -> PersistenceResult<()> {
        let path = self.single_path();
        let contents = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(PersistenceError::Io(e)),
        };
        let all: AllData = serde_json::from_str(&contents)?;
        state.user_data = int_key_map(all.user_data)?;
        state.chat_data = int_key_map(all.chat_data)?;
        state.bot_data = all.bot_data;
        state.callback_data = all.callback_data;
        state.conversations = decode_conversations_map(all.conversations)?;
        Ok(())
    }

    fn load_multi_file(&self, state: &mut FileState) -> PersistenceResult<()> {
        state.user_data = self.load_category_typed("user_data")?;
        state.chat_data = self.load_category_typed("chat_data")?;
        state.bot_data = self
            .load_category::<JsonMap>("bot_data")?
            .unwrap_or_default();
        state.callback_data = self.load_category("callback_data")?;
        let raw: Option<HashMap<String, HashMap<String, Option<Value>>>> =
            self.load_category("conversations")?;
        state.conversations = match raw {
            Some(m) => decode_conversations_map(m)?,
            None => HashMap::new(),
        };
        Ok(())
    }

    fn load_category_typed(
        &self,
        suffix: &str,
    ) -> PersistenceResult<HashMap<i64, JsonMap>> {
        let raw: Option<HashMap<String, JsonMap>> = self.load_category(suffix)?;
        match raw {
            Some(m) => int_key_map(m),
            None => Ok(HashMap::new()),
        }
    }

    fn load_category<T: for<'de> Deserialize<'de>>(
        &self,
        suffix: &str,
    ) -> PersistenceResult<Option<T>> {
        let path = self.category_path(suffix);
        let contents = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(PersistenceError::Io(e)),
        };
        let val: T = serde_json::from_str(&contents)?;
        Ok(Some(val))
    }

    fn dump_single_file(&self, state: &FileState) -> PersistenceResult<()> {
        let all = AllData {
            user_data: str_key_map(&state.user_data),
            chat_data: str_key_map(&state.chat_data),
            bot_data: state.bot_data.clone(),
            callback_data: state.callback_data.clone(),
            conversations: encode_conversations_map(&state.conversations),
        };
        let json = serde_json::to_string_pretty(&all)?;
        atomic_write(&self.single_path(), json.as_bytes())
    }

    fn dump_category<T: Serialize>(&self, suffix: &str, data: &T) -> PersistenceResult<()> {
        let json = serde_json::to_string_pretty(data)?;
        atomic_write(&self.category_path(suffix), json.as_bytes())
    }

    /// Persist all dirty data to disk if `on_flush` is false.
    async fn maybe_persist(&self) -> PersistenceResult<()> {
        if self.on_flush {
            return Ok(());
        }
        self.do_flush().await
    }

    async fn do_flush(&self) -> PersistenceResult<()> {
        let guard = self.state.read().await;
        if !guard.loaded {
            return Ok(());
        }
        if self.single_file {
            self.dump_single_file(&guard)?;
        } else {
            if !guard.user_data.is_empty() {
                self.dump_category("user_data", &str_key_map(&guard.user_data))?;
            }
            if !guard.chat_data.is_empty() {
                self.dump_category("chat_data", &str_key_map(&guard.chat_data))?;
            }
            if !guard.bot_data.is_empty() {
                self.dump_category("bot_data", &guard.bot_data)?;
            }
            if guard.callback_data.is_some() {
                self.dump_category("callback_data", &guard.callback_data)?;
            }
            if !guard.conversations.is_empty() {
                self.dump_category(
                    "conversations",
                    &encode_conversations_map(&guard.conversations),
                )?;
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// BasePersistence
// ---------------------------------------------------------------------------

impl BasePersistence for JsonFilePersistence {
    async fn get_user_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> {
        self.ensure_loaded().await?;
        let guard = self.state.read().await;
        Ok(guard.user_data.clone())
    }

    async fn get_chat_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> {
        self.ensure_loaded().await?;
        let guard = self.state.read().await;
        Ok(guard.chat_data.clone())
    }

    async fn get_bot_data(&self) -> PersistenceResult<JsonMap> {
        self.ensure_loaded().await?;
        let guard = self.state.read().await;
        Ok(guard.bot_data.clone())
    }

    async fn get_callback_data(&self) -> PersistenceResult<Option<CdcData>> {
        self.ensure_loaded().await?;
        let guard = self.state.read().await;
        Ok(guard.callback_data.clone())
    }

    async fn get_conversations(&self, name: &str) -> PersistenceResult<ConversationDict> {
        self.ensure_loaded().await?;
        let guard = self.state.read().await;
        Ok(guard.conversations.get(name).cloned().unwrap_or_default())
    }

    async fn update_user_data(&self, user_id: i64, data: &JsonMap) -> PersistenceResult<()> {
        self.ensure_loaded().await?;
        {
            let mut guard = self.state.write().await;
            if guard.user_data.get(&user_id) == Some(data) {
                return Ok(());
            }
            guard.user_data.insert(user_id, data.clone());
        }
        self.maybe_persist().await
    }

    async fn update_chat_data(&self, chat_id: i64, data: &JsonMap) -> PersistenceResult<()> {
        self.ensure_loaded().await?;
        {
            let mut guard = self.state.write().await;
            if guard.chat_data.get(&chat_id) == Some(data) {
                return Ok(());
            }
            guard.chat_data.insert(chat_id, data.clone());
        }
        self.maybe_persist().await
    }

    async fn update_bot_data(&self, data: &JsonMap) -> PersistenceResult<()> {
        self.ensure_loaded().await?;
        {
            let mut guard = self.state.write().await;
            if &guard.bot_data == data {
                return Ok(());
            }
            guard.bot_data = data.clone();
        }
        self.maybe_persist().await
    }

    async fn update_callback_data(&self, data: &CdcData) -> PersistenceResult<()> {
        self.ensure_loaded().await?;
        {
            let mut guard = self.state.write().await;
            if guard.callback_data.as_ref() == Some(data) {
                return Ok(());
            }
            guard.callback_data = Some(data.clone());
        }
        self.maybe_persist().await
    }

    async fn update_conversation(
        &self,
        name: &str,
        key: &ConversationKey,
        new_state: Option<&Value>,
    ) -> PersistenceResult<()> {
        self.ensure_loaded().await?;
        {
            let mut guard = self.state.write().await;
            let handler_map = guard.conversations.entry(name.to_owned()).or_default();
            let new_val = new_state.cloned();
            if handler_map.get(key) == Some(&new_val) {
                return Ok(());
            }
            handler_map.insert(key.clone(), new_val);
        }
        self.maybe_persist().await
    }

    async fn drop_chat_data(&self, chat_id: i64) -> PersistenceResult<()> {
        self.ensure_loaded().await?;
        {
            let mut guard = self.state.write().await;
            if !guard.chat_data.contains_key(&chat_id) {
                return Ok(());
            }
            guard.chat_data.remove(&chat_id);
        }
        self.maybe_persist().await
    }

    async fn drop_user_data(&self, user_id: i64) -> PersistenceResult<()> {
        self.ensure_loaded().await?;
        {
            let mut guard = self.state.write().await;
            if !guard.user_data.contains_key(&user_id) {
                return Ok(());
            }
            guard.user_data.remove(&user_id);
        }
        self.maybe_persist().await
    }

    async fn flush(&self) -> PersistenceResult<()> {
        self.do_flush().await
    }

    fn update_interval(&self) -> f64 {
        self.update_interval
    }

    fn store_data(&self) -> PersistenceInput {
        self.store_data
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Write data to a temporary file and then atomically rename to the target
/// path to avoid corruption on crash.
fn atomic_write(path: &Path, data: &[u8]) -> PersistenceResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, data)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

/// Convert a `HashMap<String, V>` (from JSON) to `HashMap<i64, V>`.
fn int_key_map<V>(src: HashMap<String, V>) -> PersistenceResult<HashMap<i64, V>> {
    let mut out = HashMap::with_capacity(src.len());
    for (k, v) in src {
        let id: i64 = k.parse().map_err(|_| {
            PersistenceError::Custom(format!("non-integer key: {k}"))
        })?;
        out.insert(id, v);
    }
    Ok(out)
}

/// Convert `HashMap<i64, V>` to `HashMap<String, V>` for JSON serialization.
fn str_key_map<V: Clone>(src: &HashMap<i64, V>) -> HashMap<String, V> {
    src.iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect()
}

/// Decode the on-disk conversations format into our in-memory representation.
fn decode_conversations_map(
    raw: HashMap<String, HashMap<String, Option<Value>>>,
) -> PersistenceResult<HashMap<String, ConversationDict>> {
    let mut out = HashMap::with_capacity(raw.len());
    for (handler, states) in raw {
        let mut conv = ConversationDict::new();
        for (key_json, state) in states {
            let key: ConversationKey = serde_json::from_str(&key_json)?;
            conv.insert(key, state);
        }
        out.insert(handler, conv);
    }
    Ok(out)
}

/// Encode conversations for disk storage.
fn encode_conversations_map(
    src: &HashMap<String, ConversationDict>,
) -> HashMap<String, HashMap<String, Option<Value>>> {
    let mut out = HashMap::with_capacity(src.len());
    for (handler, states) in src {
        let mut inner = HashMap::with_capacity(states.len());
        for (key, state) in states {
            let key_json = serde_json::to_string(key).unwrap_or_default();
            inner.insert(key_json, state.clone());
        }
        out.insert(handler.clone(), inner);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn roundtrip_single_file() {
        let dir = std::env::temp_dir().join("tg_json_test_single");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let base = dir.join("data");

        let p = JsonFilePersistence::new(&base, true, false);
        let mut m = JsonMap::new();
        m.insert("hello".into(), Value::String("world".into()));
        p.update_bot_data(&m).await.unwrap();

        // Re-open and verify.
        let p2 = JsonFilePersistence::new(&base, true, false);
        let loaded = p2.get_bot_data().await.unwrap();
        assert_eq!(loaded.get("hello"), Some(&Value::String("world".into())));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn roundtrip_multi_file() {
        let dir = std::env::temp_dir().join("tg_json_test_multi");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let base = dir.join("data");

        let p = JsonFilePersistence::new(&base, false, false);
        let mut ud = JsonMap::new();
        ud.insert("name".into(), Value::String("alice".into()));
        p.update_user_data(1, &ud).await.unwrap();

        let p2 = JsonFilePersistence::new(&base, false, false);
        let loaded = p2.get_user_data().await.unwrap();
        assert_eq!(loaded.get(&1).and_then(|m| m.get("name")),
            Some(&Value::String("alice".into())));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn on_flush_defers_writes() {
        let dir = std::env::temp_dir().join("tg_json_test_flush");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let base = dir.join("data");

        let p = JsonFilePersistence::new(&base, true, true);
        let m = JsonMap::new();
        p.update_bot_data(&m).await.unwrap();
        // File should NOT exist yet because on_flush is true.
        assert!(!base.with_extension("json").exists());
        p.flush().await.unwrap();
        assert!(base.with_extension("json").exists());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
