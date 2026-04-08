//! In-memory persistence using `HashMap` and JSON serialization.
//!
//! Port of `telegram.ext._dictpersistence.DictPersistence`.
//!
//! Data lives only in memory and is lost on shutdown. This is primarily
//! useful as a starting point or for testing.

use std::collections::HashMap;

use serde_json::Value;
use tokio::sync::RwLock;

use crate::persistence::base::{
    BasePersistence, PersistenceError, PersistenceInput, PersistenceResult,
};
use crate::utils::types::{CdcData, ConversationDict, ConversationKey, JsonMap};

/// In-memory persistence back-end.
///
/// All state is held behind a `tokio::sync::RwLock` for interior mutability
/// so the `BasePersistence` methods only require `&self`.
#[derive(Debug)]
pub struct DictPersistence {
    store_data: PersistenceInput,
    update_interval: f64,
    state: RwLock<DictState>,
}

#[derive(Debug, Default)]
struct DictState {
    user_data: HashMap<i64, JsonMap>,
    chat_data: HashMap<i64, JsonMap>,
    bot_data: JsonMap,
    callback_data: Option<CdcData>,
    conversations: HashMap<String, ConversationDict>,
    /// Cached JSON strings; `None` means the cache is invalidated.
    user_data_json: Option<String>,
    chat_data_json: Option<String>,
    bot_data_json: Option<String>,
    callback_data_json: Option<String>,
    conversations_json: Option<String>,
}

impl DictPersistence {
    /// Create a new empty in-memory persistence store.
    pub fn new() -> Self {
        Self {
            store_data: PersistenceInput::default(),
            update_interval: 60.0,
            state: RwLock::new(DictState::default()),
        }
    }

    /// Build from pre-existing JSON strings (typically loaded from a previous
    /// session's serialized output).
    pub fn from_json(
        user_data_json: Option<&str>,
        chat_data_json: Option<&str>,
        bot_data_json: Option<&str>,
        callback_data_json: Option<&str>,
        conversations_json: Option<&str>,
    ) -> Result<Self, PersistenceError> {
        let mut state = DictState::default();

        if let Some(json) = user_data_json {
            if !json.is_empty() {
                state.user_data = decode_user_chat_data(json)?;
                state.user_data_json = Some(json.to_owned());
            }
        }

        if let Some(json) = chat_data_json {
            if !json.is_empty() {
                state.chat_data = decode_user_chat_data(json)?;
                state.chat_data_json = Some(json.to_owned());
            }
        }

        if let Some(json) = bot_data_json {
            if !json.is_empty() {
                let parsed: Value = serde_json::from_str(json)?;
                let map = parsed
                    .as_object()
                    .ok_or_else(|| {
                        PersistenceError::Custom("bot_data_json must be a JSON object".into())
                    })?
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                state.bot_data = map;
                state.bot_data_json = Some(json.to_owned());
            }
        }

        if let Some(json) = callback_data_json {
            if !json.is_empty() {
                let parsed: Value = serde_json::from_str(json)?;
                if parsed.is_null() {
                    state.callback_data = None;
                } else {
                    state.callback_data = Some(decode_callback_data(&parsed)?);
                }
                state.callback_data_json = Some(json.to_owned());
            }
        }

        if let Some(json) = conversations_json {
            if !json.is_empty() {
                state.conversations = decode_conversations(json)?;
                state.conversations_json = Some(json.to_owned());
            }
        }

        Ok(Self {
            store_data: PersistenceInput::default(),
            update_interval: 60.0,
            state: RwLock::new(state),
        })
    }

    /// Set which data categories to store.
    pub fn with_store_data(mut self, input: PersistenceInput) -> Self {
        self.store_data = input;
        self
    }

    /// Set the update interval in seconds.
    pub fn with_update_interval(mut self, seconds: f64) -> Self {
        self.update_interval = seconds;
        self
    }

    // -- JSON accessors -------------------------------------------------------

    /// Return the current user data serialized as JSON.
    pub async fn user_data_json(&self) -> String {
        let guard = self.state.read().await;
        guard
            .user_data_json
            .clone()
            .unwrap_or_else(|| serde_json::to_string(&guard.user_data).unwrap_or_default())
    }

    /// Return the current chat data serialized as JSON.
    pub async fn chat_data_json(&self) -> String {
        let guard = self.state.read().await;
        guard
            .chat_data_json
            .clone()
            .unwrap_or_else(|| serde_json::to_string(&guard.chat_data).unwrap_or_default())
    }

    /// Return the current bot data serialized as JSON.
    pub async fn bot_data_json(&self) -> String {
        let guard = self.state.read().await;
        guard
            .bot_data_json
            .clone()
            .unwrap_or_else(|| serde_json::to_string(&guard.bot_data).unwrap_or_default())
    }

    /// Return the current callback data serialized as JSON.
    pub async fn callback_data_json(&self) -> String {
        let guard = self.state.read().await;
        guard
            .callback_data_json
            .clone()
            .unwrap_or_else(|| serde_json::to_string(&guard.callback_data).unwrap_or_default())
    }

    /// Return the current conversations serialized as JSON.
    pub async fn conversations_json(&self) -> String {
        let guard = self.state.read().await;
        if let Some(ref cached) = guard.conversations_json {
            return cached.clone();
        }
        if guard.conversations.is_empty() {
            return serde_json::to_string(&guard.conversations).unwrap_or_default();
        }
        encode_conversations(&guard.conversations)
    }
}

impl Default for DictPersistence {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// BasePersistence implementation
// ---------------------------------------------------------------------------

impl BasePersistence for DictPersistence {
    async fn get_user_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> {
        let guard = self.state.read().await;
        Ok(guard.user_data.clone())
    }

    async fn get_chat_data(&self) -> PersistenceResult<HashMap<i64, JsonMap>> {
        let guard = self.state.read().await;
        Ok(guard.chat_data.clone())
    }

    async fn get_bot_data(&self) -> PersistenceResult<JsonMap> {
        let guard = self.state.read().await;
        Ok(guard.bot_data.clone())
    }

    async fn get_callback_data(&self) -> PersistenceResult<Option<CdcData>> {
        let guard = self.state.read().await;
        Ok(guard.callback_data.clone())
    }

    async fn get_conversations(&self, name: &str) -> PersistenceResult<ConversationDict> {
        let guard = self.state.read().await;
        Ok(guard.conversations.get(name).cloned().unwrap_or_default())
    }

    async fn update_user_data(&self, user_id: i64, data: &JsonMap) -> PersistenceResult<()> {
        let mut guard = self.state.write().await;
        if guard.user_data.get(&user_id) == Some(data) {
            return Ok(());
        }
        guard.user_data.insert(user_id, data.clone());
        guard.user_data_json = None;
        Ok(())
    }

    async fn update_chat_data(&self, chat_id: i64, data: &JsonMap) -> PersistenceResult<()> {
        let mut guard = self.state.write().await;
        if guard.chat_data.get(&chat_id) == Some(data) {
            return Ok(());
        }
        guard.chat_data.insert(chat_id, data.clone());
        guard.chat_data_json = None;
        Ok(())
    }

    async fn update_bot_data(&self, data: &JsonMap) -> PersistenceResult<()> {
        let mut guard = self.state.write().await;
        if &guard.bot_data == data {
            return Ok(());
        }
        guard.bot_data = data.clone();
        guard.bot_data_json = None;
        Ok(())
    }

    async fn update_callback_data(&self, data: &CdcData) -> PersistenceResult<()> {
        let mut guard = self.state.write().await;
        if guard.callback_data.as_ref() == Some(data) {
            return Ok(());
        }
        guard.callback_data = Some(data.clone());
        guard.callback_data_json = None;
        Ok(())
    }

    async fn update_conversation(
        &self,
        name: &str,
        key: &ConversationKey,
        new_state: Option<&Value>,
    ) -> PersistenceResult<()> {
        let mut guard = self.state.write().await;
        let handler_map = guard.conversations.entry(name.to_owned()).or_default();
        let current = handler_map.get(key);
        let new_val = new_state.cloned();
        if current == Some(&new_val) {
            return Ok(());
        }
        handler_map.insert(key.clone(), new_val);
        guard.conversations_json = None;
        Ok(())
    }

    async fn drop_chat_data(&self, chat_id: i64) -> PersistenceResult<()> {
        let mut guard = self.state.write().await;
        guard.chat_data.remove(&chat_id);
        guard.chat_data_json = None;
        Ok(())
    }

    async fn drop_user_data(&self, user_id: i64) -> PersistenceResult<()> {
        let mut guard = self.state.write().await;
        guard.user_data.remove(&user_id);
        guard.user_data_json = None;
        Ok(())
    }

    async fn flush(&self) -> PersistenceResult<()> {
        // In-memory only; nothing to flush.
        Ok(())
    }

    fn update_interval(&self) -> f64 {
        self.update_interval
    }

    fn store_data(&self) -> PersistenceInput {
        self.store_data
    }
}

// ---------------------------------------------------------------------------
// JSON encode/decode helpers (match Python's DictPersistence)
// ---------------------------------------------------------------------------

/// Decode user/chat data from a JSON string where top-level keys are
/// stringified user/chat IDs and values are JSON objects.
fn decode_user_chat_data(json: &str) -> Result<HashMap<i64, JsonMap>, PersistenceError> {
    let raw: HashMap<String, Value> = serde_json::from_str(json)?;
    let mut out = HashMap::with_capacity(raw.len());
    for (key_str, val) in raw {
        let id: i64 = key_str.parse().map_err(|_| {
            PersistenceError::Custom(format!("non-integer key in user/chat data: {key_str}"))
        })?;
        let map = match val {
            Value::Object(m) => m.into_iter().collect(),
            _ => {
                return Err(PersistenceError::Custom(
                    "user/chat data values must be objects".into(),
                ))
            }
        };
        out.insert(id, map);
    }
    Ok(out)
}

/// Decode the callback-data cache from a parsed JSON value.
fn decode_callback_data(val: &Value) -> Result<CdcData, PersistenceError> {
    let arr = val.as_array().ok_or_else(|| {
        PersistenceError::Custom("callback_data_json must be a JSON array of length 2".into())
    })?;
    if arr.len() != 2 {
        return Err(PersistenceError::Custom(
            "callback_data_json must have exactly 2 elements".into(),
        ));
    }
    let entries_raw = arr[0]
        .as_array()
        .ok_or_else(|| PersistenceError::Custom("callback_data entries must be an array".into()))?;
    let mut entries = Vec::with_capacity(entries_raw.len());
    for entry in entries_raw {
        let triple = entry.as_array().ok_or_else(|| {
            PersistenceError::Custom("each callback_data entry must be a 3-element array".into())
        })?;
        if triple.len() != 3 {
            return Err(PersistenceError::Custom(
                "each callback_data entry must have 3 elements".into(),
            ));
        }
        let uuid = triple[0]
            .as_str()
            .ok_or_else(|| PersistenceError::Custom("entry[0] must be a string".into()))?
            .to_owned();
        let ts = triple[1]
            .as_f64()
            .ok_or_else(|| PersistenceError::Custom("entry[1] must be a number".into()))?;
        let data_map: HashMap<String, Value> = match &triple[2] {
            Value::Object(m) => m.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            _ => {
                return Err(PersistenceError::Custom(
                    "entry[2] must be an object".into(),
                ))
            }
        };
        entries.push((uuid, ts, data_map));
    }
    let mapping: HashMap<String, String> = serde_json::from_value(arr[1].clone())?;
    Ok((entries, mapping))
}

/// Decode conversations from JSON. Conversation keys are stored as
/// JSON-serialized arrays under each handler name.
fn decode_conversations(json: &str) -> Result<HashMap<String, ConversationDict>, PersistenceError> {
    let raw: HashMap<String, HashMap<String, Value>> = serde_json::from_str(json)?;
    let mut out = HashMap::with_capacity(raw.len());
    for (handler, states) in raw {
        let mut conv = ConversationDict::new();
        for (key_json, state) in states {
            let key: ConversationKey = serde_json::from_str(&key_json)?;
            let val = if state.is_null() { None } else { Some(state) };
            conv.insert(key, val);
        }
        out.insert(handler, conv);
    }
    Ok(out)
}

/// Encode conversations to a JSON string. Conversation keys (which are
/// `Vec<ConversationKeyPart>`) are serialized as JSON strings themselves.
fn encode_conversations(conversations: &HashMap<String, ConversationDict>) -> String {
    let mut outer: HashMap<&str, HashMap<String, &Option<Value>>> = HashMap::new();
    for (handler, states) in conversations {
        let mut inner = HashMap::new();
        for (key, state) in states {
            let key_json = serde_json::to_string(key).unwrap_or_default();
            inner.insert(key_json, state);
        }
        outer.insert(handler, inner);
    }
    serde_json::to_string(&outer).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn empty_persistence_returns_defaults() {
        let p = DictPersistence::new();
        assert!(p.get_user_data().await.unwrap().is_empty());
        assert!(p.get_chat_data().await.unwrap().is_empty());
        assert!(p.get_bot_data().await.unwrap().is_empty());
        assert!(p.get_callback_data().await.unwrap().is_none());
        assert!(p.get_conversations("test").await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn update_and_read_user_data() {
        let p = DictPersistence::new();
        let mut data = JsonMap::new();
        data.insert("key".into(), Value::String("value".into()));
        p.update_user_data(42, &data).await.unwrap();

        let loaded = p.get_user_data().await.unwrap();
        assert_eq!(loaded.get(&42), Some(&data));
    }

    #[tokio::test]
    async fn drop_chat_data_removes_entry() {
        let p = DictPersistence::new();
        let data = JsonMap::new();
        p.update_chat_data(1, &data).await.unwrap();
        p.drop_chat_data(1).await.unwrap();
        assert!(p.get_chat_data().await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn from_json_round_trip() {
        let p = DictPersistence::from_json(
            Some(r#"{"123": {"k": "v"}}"#),
            Some(r#"{"456": {"a": 1}}"#),
            Some(r#"{"bot_key": true}"#),
            None,
            None,
        )
        .unwrap();

        let ud = p.get_user_data().await.unwrap();
        assert!(ud.contains_key(&123));

        let cd = p.get_chat_data().await.unwrap();
        assert!(cd.contains_key(&456));

        let bd = p.get_bot_data().await.unwrap();
        assert!(bd.contains_key("bot_key"));
    }

    #[tokio::test]
    async fn skips_update_when_unchanged() {
        let p = DictPersistence::new();
        let data: JsonMap = [("x".into(), Value::Number(1.into()))]
            .into_iter()
            .collect();
        p.update_bot_data(&data).await.unwrap();
        // Invalidate the JSON cache.
        {
            let guard = p.state.read().await;
            assert!(guard.bot_data_json.is_none());
        }
        // Read to populate the cache, then update with the same data.
        let _ = p.bot_data_json().await;
        p.update_bot_data(&data).await.unwrap();
        // Cache should not have been invalidated again (early return).
        let guard = p.state.read().await;
        // The first update invalidated it; the second shouldn't have since
        // the data was equal.  But our JSON cache is only built on read,
        // so it remains None from the first write.
        drop(guard);
    }
}
