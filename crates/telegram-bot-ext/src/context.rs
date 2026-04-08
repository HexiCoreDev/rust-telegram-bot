//! Callback context passed to handlers and error handlers.
//!
//! Ported from `python-telegram-bot/src/telegram/ext/_callbackcontext.py`.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use serde_json::Value;
use tokio::sync::RwLock;

use telegram_bot_raw::types::update::Update;

use crate::context_types::DefaultData;
use crate::ext_bot::ExtBot;
#[cfg(feature = "job-queue")]
use crate::job_queue::JobQueue;

// ---------------------------------------------------------------------------
// Typed data guard wrappers
// ---------------------------------------------------------------------------

/// A typed read guard over a [`DefaultData`] map.
///
/// Provides convenience accessors that eliminate manual `get().and_then(|v| v.as_*)` chains
/// while still exposing the raw `HashMap` via [`raw()`](Self::raw).
pub struct DataReadGuard<'a> {
    inner: tokio::sync::RwLockReadGuard<'a, DefaultData>,
}

impl<'a> DataReadGuard<'a> {
    /// Get a string value by key.
    #[must_use]
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.inner.get(key).and_then(|v| v.as_str())
    }

    /// Get an `i64` value by key.
    #[must_use]
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.inner.get(key).and_then(|v| v.as_i64())
    }

    /// Get a `f64` value by key.
    #[must_use]
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.inner.get(key).and_then(|v| v.as_f64())
    }

    /// Get a `bool` value by key.
    #[must_use]
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.inner.get(key).and_then(|v| v.as_bool())
    }

    /// Get a raw [`Value`] by key.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.inner.get(key)
    }

    /// Get a set of `i64` IDs stored as a JSON array under `key`.
    ///
    /// This is a common pattern for tracking `user_ids`, `chat_ids`, etc.
    /// Returns an empty set if the key is missing or the value is not an array.
    #[must_use]
    pub fn get_id_set(&self, key: &str) -> HashSet<i64> {
        self.inner
            .get(key)
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_i64()).collect())
            .unwrap_or_default()
    }

    /// Access the raw underlying `HashMap`.
    #[must_use]
    pub fn raw(&self) -> &DefaultData {
        &self.inner
    }

    /// Returns `true` if the underlying map is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the number of entries in the underlying map.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl std::ops::Deref for DataReadGuard<'_> {
    type Target = DefaultData;

    fn deref(&self) -> &DefaultData {
        &self.inner
    }
}

/// A typed write guard over a [`DefaultData`] map.
///
/// Provides typed setters alongside the raw `HashMap` accessors.
pub struct DataWriteGuard<'a> {
    inner: tokio::sync::RwLockWriteGuard<'a, DefaultData>,
}

impl<'a> DataWriteGuard<'a> {
    // -- Typed getters (same as DataReadGuard) --------------------------------

    /// Get a string value by key.
    #[must_use]
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.inner.get(key).and_then(|v| v.as_str())
    }

    /// Get an `i64` value by key.
    #[must_use]
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.inner.get(key).and_then(|v| v.as_i64())
    }

    /// Get a `f64` value by key.
    #[must_use]
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.inner.get(key).and_then(|v| v.as_f64())
    }

    /// Get a `bool` value by key.
    #[must_use]
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.inner.get(key).and_then(|v| v.as_bool())
    }

    /// Get a raw [`Value`] by key.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.inner.get(key)
    }

    /// Get a set of `i64` IDs stored as a JSON array under `key`.
    #[must_use]
    pub fn get_id_set(&self, key: &str) -> HashSet<i64> {
        self.inner
            .get(key)
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_i64()).collect())
            .unwrap_or_default()
    }

    // -- Typed setters --------------------------------------------------------

    /// Set a string value.
    pub fn set_str(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.inner.insert(key.into(), Value::String(value.into()));
    }

    /// Set an `i64` value.
    pub fn set_i64(&mut self, key: impl Into<String>, value: i64) {
        self.inner.insert(key.into(), Value::Number(value.into()));
    }

    /// Set a `bool` value.
    pub fn set_bool(&mut self, key: impl Into<String>, value: bool) {
        self.inner.insert(key.into(), Value::Bool(value));
    }

    /// Insert a raw [`Value`].
    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        self.inner.insert(key, value)
    }

    /// Add an `i64` to a set stored as a JSON array under `key`.
    ///
    /// Creates the array if the key does not exist. Deduplicates values.
    pub fn add_to_id_set(&mut self, key: &str, id: i64) {
        let entry = self
            .inner
            .entry(key.to_owned())
            .or_insert_with(|| Value::Array(vec![]));
        if let Some(arr) = entry.as_array_mut() {
            let val = Value::Number(id.into());
            if !arr.contains(&val) {
                arr.push(val);
            }
        }
    }

    /// Remove an `i64` from a set stored as a JSON array under `key`.
    pub fn remove_from_id_set(&mut self, key: &str, id: i64) {
        if let Some(arr) = self.inner.get_mut(key).and_then(|v| v.as_array_mut()) {
            arr.retain(|v| v.as_i64() != Some(id));
        }
    }

    /// Access the raw underlying `HashMap`.
    #[must_use]
    pub fn raw(&self) -> &DefaultData {
        &self.inner
    }

    /// Access the raw underlying `HashMap` mutably.
    pub fn raw_mut(&mut self) -> &mut DefaultData {
        &mut self.inner
    }

    /// Access the `Entry` API of the underlying `HashMap`.
    pub fn entry(
        &mut self,
        key: String,
    ) -> std::collections::hash_map::Entry<'_, String, Value> {
        self.inner.entry(key)
    }

    /// Get a mutable reference to a value by key.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.inner.get_mut(key)
    }

    /// Returns `true` if the underlying map is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the number of entries in the underlying map.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Remove a key from the underlying map.
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.inner.remove(key)
    }
}

impl std::ops::Deref for DataWriteGuard<'_> {
    type Target = DefaultData;

    fn deref(&self) -> &DefaultData {
        &self.inner
    }
}

impl std::ops::DerefMut for DataWriteGuard<'_> {
    fn deref_mut(&mut self) -> &mut DefaultData {
        &mut self.inner
    }
}

// ---------------------------------------------------------------------------
// CallbackContext
// ---------------------------------------------------------------------------

/// A context object passed to handler callbacks.
#[derive(Debug, Clone)]
pub struct CallbackContext {
    /// The bot associated with this context.
    bot: Arc<ExtBot>,

    /// The chat id associated with this context (used to look up `chat_data`).
    chat_id: Option<i64>,

    /// The user id associated with this context (used to look up `user_data`).
    user_id: Option<i64>,

    // -- Shared data references (populated by Application) --------------------

    /// Reference into the application's per-user data store.
    user_data_store: Arc<RwLock<HashMap<i64, DefaultData>>>,

    /// Reference into the application's per-chat data store.
    chat_data_store: Arc<RwLock<HashMap<i64, DefaultData>>>,

    /// Reference to the application's bot-wide data.
    bot_data: Arc<RwLock<DefaultData>>,

    // -- Per-callback mutable state -------------------------------------------

    /// Positional regex match results (populated by regex-based handlers).
    pub matches: Option<Vec<String>>,

    /// Named regex capture groups (populated by regex-based handlers when the
    /// pattern contains at least one named group).
    ///
    /// Mirrors Python's `context.matches` which exposes the full `re.Match`
    /// object including `match.groupdict()`.
    pub named_matches: Option<HashMap<String, String>>,

    /// Arguments to a command (populated by `CommandHandler`).
    pub args: Option<Vec<String>>,

    /// The error that was raised.  Only present in error handler contexts.
    pub error: Option<Arc<dyn std::error::Error + Send + Sync>>,

    /// Extra key-value pairs that handlers can attach for downstream handlers.
    pub extra: HashMap<String, Value>,

    /// Optional reference to the application's job queue.
    ///
    /// Requires the `job-queue` feature.
    #[cfg(feature = "job-queue")]
    pub job_queue: Option<Arc<JobQueue>>,
}

impl CallbackContext {
    /// Creates a new `CallbackContext`.
    #[must_use]
    pub fn new(
        bot: Arc<ExtBot>,
        chat_id: Option<i64>,
        user_id: Option<i64>,
        user_data_store: Arc<RwLock<HashMap<i64, DefaultData>>>,
        chat_data_store: Arc<RwLock<HashMap<i64, DefaultData>>>,
        bot_data: Arc<RwLock<DefaultData>>,
    ) -> Self {
        Self {
            bot,
            chat_id,
            user_id,
            user_data_store,
            chat_data_store,
            bot_data,
            matches: None,
            named_matches: None,
            args: None,
            error: None,
            extra: HashMap::new(),
            #[cfg(feature = "job-queue")]
            job_queue: None,
        }
    }

    // -- Factory methods (mirrors Python classmethod constructors) -------------

    /// Constructs a context from a typed [`Update`].
    #[must_use]
    pub fn from_update(
        update: &Update,
        bot: Arc<ExtBot>,
        user_data_store: Arc<RwLock<HashMap<i64, DefaultData>>>,
        chat_data_store: Arc<RwLock<HashMap<i64, DefaultData>>>,
        bot_data: Arc<RwLock<DefaultData>>,
    ) -> Self {
        let (chat_id, user_id) = extract_ids(update);
        Self::new(bot, chat_id, user_id, user_data_store, chat_data_store, bot_data)
    }

    /// Constructs a context for an error handler.
    #[must_use]
    pub fn from_error(
        update: Option<&Update>,
        error: Arc<dyn std::error::Error + Send + Sync>,
        bot: Arc<ExtBot>,
        user_data_store: Arc<RwLock<HashMap<i64, DefaultData>>>,
        chat_data_store: Arc<RwLock<HashMap<i64, DefaultData>>>,
        bot_data: Arc<RwLock<DefaultData>>,
    ) -> Self {
        let (chat_id, user_id) = update.map_or((None, None), extract_ids);
        let mut ctx = Self::new(bot, chat_id, user_id, user_data_store, chat_data_store, bot_data);
        ctx.error = Some(error);
        ctx
    }

    // -- Accessors ------------------------------------------------------------

    // -- Accessors ------------------------------------------------------------

    /// Returns a reference to the bot associated with this context.
    #[must_use]
    pub fn bot(&self) -> &Arc<ExtBot> {
        &self.bot
    }

    /// Returns the chat ID extracted from the update, if available.
    #[must_use]
    pub fn chat_id(&self) -> Option<i64> {
        self.chat_id
    }

    /// Returns the user ID extracted from the update, if available.
    #[must_use]
    pub fn user_id(&self) -> Option<i64> {
        self.user_id
    }

    // -- Typed bot_data accessors ---------------------------------------------

    /// Acquire a read lock on the bot-wide data store, returning a typed guard.
    pub async fn bot_data(&self) -> DataReadGuard<'_> {
        DataReadGuard {
            inner: self.bot_data.read().await,
        }
    }

    /// Acquire a write lock on the bot-wide data store, returning a typed guard.
    pub async fn bot_data_mut(&self) -> DataWriteGuard<'_> {
        DataWriteGuard {
            inner: self.bot_data.write().await,
        }
    }

    // -- user_data / chat_data (unchanged API, returns cloned snapshot) --------

    /// Returns a cloned snapshot of the current user's data, if a user ID is set.
    pub async fn user_data(&self) -> Option<DefaultData> {
        let uid = self.user_id?;
        let store = self.user_data_store.read().await;
        store.get(&uid).cloned()
    }

    /// Returns a cloned snapshot of the current chat's data, if a chat ID is set.
    pub async fn chat_data(&self) -> Option<DefaultData> {
        let cid = self.chat_id?;
        let store = self.chat_data_store.read().await;
        store.get(&cid).cloned()
    }

    /// Insert a key-value pair into the current user's data store. Returns `false` if no user ID.
    pub async fn set_user_data(&self, key: String, value: Value) -> bool {
        let uid = match self.user_id {
            Some(id) => id,
            None => return false,
        };
        let mut store = self.user_data_store.write().await;
        store.entry(uid).or_insert_with(HashMap::new).insert(key, value);
        true
    }

    /// Insert a key-value pair into the current chat's data store. Returns `false` if no chat ID.
    pub async fn set_chat_data(&self, key: String, value: Value) -> bool {
        let cid = match self.chat_id {
            Some(id) => id,
            None => return false,
        };
        let mut store = self.chat_data_store.write().await;
        store.entry(cid).or_insert_with(HashMap::new).insert(key, value);
        true
    }

    /// Returns the first positional regex match, if available.
    #[must_use]
    pub fn match_result(&self) -> Option<&str> {
        self.matches.as_ref().and_then(|m| m.first().map(String::as_str))
    }

    /// Drop the cached callback data for a given callback query ID.
    pub async fn drop_callback_data(
        &self,
        callback_query_id: &str,
    ) -> Result<(), crate::callback_data_cache::InvalidCallbackData> {
        let cache = self
            .bot
            .callback_data_cache()
            .ok_or(crate::callback_data_cache::InvalidCallbackData {
                callback_data: None,
            })?;
        let mut guard = cache.write().await;
        guard.drop_data(callback_query_id)
    }

    /// Set the job queue reference on this context.
    ///
    /// Requires the `job-queue` feature.
    #[cfg(feature = "job-queue")]
    pub fn with_job_queue(mut self, jq: Arc<JobQueue>) -> Self {
        self.job_queue = Some(jq);
        self
    }


    // -- Convenience methods (mirrors python-telegram-bot patterns) -----------

    /// Send a text reply to the chat associated with the given update.
    ///
    /// This is a convenience method that mirrors python-telegram-bot's
    /// `update.message.reply_text(text)` / `context.bot.send_message(...)`.
    ///
    /// # Errors
    ///
    /// Returns `TelegramError` if the chat cannot be determined from the
    /// update or if the Telegram API call fails.
    pub async fn reply_text(
        &self,
        update: &Update,
        text: &str,
    ) -> Result<telegram_bot_raw::types::message::Message, telegram_bot_raw::error::TelegramError> {
        let chat_id = update
            .effective_chat()
            .map(|c| c.id)
            .ok_or_else(|| telegram_bot_raw::error::TelegramError::Network("No chat in update".into()))?;
        self.bot().send_message(chat_id, text).await
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract chat and user IDs from a typed [`Update`] using its computed
/// properties. This is vastly cleaner than the previous Value-based approach.
fn extract_ids(update: &Update) -> (Option<i64>, Option<i64>) {
    let chat_id = update.effective_chat().map(|c| c.id);
    let user_id = update.effective_user().map(|u| u.id);
    (chat_id, user_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ext_bot::test_support::mock_request;
    use telegram_bot_raw::bot::Bot;

    fn make_bot() -> Arc<ExtBot> {
        let bot = Bot::new("test", mock_request());
        Arc::new(ExtBot::from_bot(bot))
    }

    fn make_stores() -> (
        Arc<RwLock<HashMap<i64, DefaultData>>>,
        Arc<RwLock<HashMap<i64, DefaultData>>>,
        Arc<RwLock<DefaultData>>,
    ) {
        (
            Arc::new(RwLock::new(HashMap::new())),
            Arc::new(RwLock::new(HashMap::new())),
            Arc::new(RwLock::new(HashMap::new())),
        )
    }

    fn make_update(json_val: serde_json::Value) -> Update {
        serde_json::from_value(json_val).unwrap()
    }

    #[test]
    fn context_basic_creation() {
        let bot = make_bot();
        let (ud, cd, bd) = make_stores();
        let ctx = CallbackContext::new(bot.clone(), Some(42), Some(7), ud, cd, bd);
        assert_eq!(ctx.chat_id(), Some(42));
        assert_eq!(ctx.user_id(), Some(7));
        assert!(ctx.error.is_none());
        assert!(ctx.args.is_none());
        assert!(ctx.matches.is_none());
        assert!(ctx.named_matches.is_none());
        #[cfg(feature = "job-queue")]
        assert!(ctx.job_queue.is_none());
    }

    #[test]
    fn extract_ids_from_message_update() {
        let update = make_update(serde_json::json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 100, "type": "private"}, "from": {"id": 200, "is_bot": false, "first_name": "Test"}}}));
        let (chat_id, user_id) = extract_ids(&update);
        assert_eq!(chat_id, Some(100));
        assert_eq!(user_id, Some(200));
    }

    #[test]
    fn extract_ids_from_callback_query() {
        let update = make_update(serde_json::json!({"update_id": 2, "callback_query": {"id": "abc", "from": {"id": 300, "is_bot": false, "first_name": "U"}, "chat_instance": "ci", "message": {"message_id": 5, "date": 0, "chat": {"id": 400, "type": "group"}}}}));
        let (chat_id, user_id) = extract_ids(&update);
        assert_eq!(chat_id, Some(400));
        assert_eq!(user_id, Some(300));
    }

    #[test]
    fn extract_ids_returns_none_for_empty() {
        let update = make_update(serde_json::json!({"update_id": 3}));
        let (chat_id, user_id) = extract_ids(&update);
        assert!(chat_id.is_none());
        assert!(user_id.is_none());
    }

    #[test]
    fn from_update_factory() {
        let bot = make_bot();
        let (ud, cd, bd) = make_stores();
        let update = make_update(serde_json::json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 10, "type": "private"}, "from": {"id": 20, "is_bot": false, "first_name": "T"}}}));
        let ctx = CallbackContext::from_update(&update, bot, ud, cd, bd);
        assert_eq!(ctx.chat_id(), Some(10));
        assert_eq!(ctx.user_id(), Some(20));
    }

    #[test]
    fn from_error_factory() {
        let bot = make_bot();
        let (ud, cd, bd) = make_stores();
        let err: Arc<dyn std::error::Error + Send + Sync> = Arc::new(std::io::Error::new(std::io::ErrorKind::Other, "boom"));
        let ctx = CallbackContext::from_error(None, err, bot, ud, cd, bd);
        assert!(ctx.error.is_some());
        assert!(ctx.chat_id().is_none());
    }

    #[tokio::test]
    async fn bot_data_access() {
        let bot = make_bot();
        let (ud, cd, bd) = make_stores();
        let ctx = CallbackContext::new(bot, None, None, ud, cd, bd);
        { let mut guard = ctx.bot_data_mut().await; guard.insert("key".into(), Value::String("val".into())); }
        let guard = ctx.bot_data().await;
        assert_eq!(guard.get("key"), Some(&Value::String("val".into())));
    }

    #[tokio::test]
    async fn user_data_returns_none_without_user_id() {
        let bot = make_bot();
        let (ud, cd, bd) = make_stores();
        let ctx = CallbackContext::new(bot, None, None, ud, cd, bd);
        assert!(ctx.user_data().await.is_none());
    }

    #[tokio::test]
    async fn chat_data_returns_none_without_chat_id() {
        let bot = make_bot();
        let (ud, cd, bd) = make_stores();
        let ctx = CallbackContext::new(bot, None, None, ud, cd, bd);
        assert!(ctx.chat_data().await.is_none());
    }

    #[tokio::test]
    async fn set_user_data_works() {
        let bot = make_bot();
        let (ud, cd, bd) = make_stores();
        let ctx = CallbackContext::new(bot, None, Some(42), ud.clone(), cd, bd);
        assert!(ctx.set_user_data("score".into(), Value::Number(100.into())).await);
        let store = ud.read().await;
        assert_eq!(store.get(&42).unwrap().get("score"), Some(&Value::Number(100.into())));
    }

    #[tokio::test]
    async fn set_chat_data_works() {
        let bot = make_bot();
        let (ud, cd, bd) = make_stores();
        let ctx = CallbackContext::new(bot, Some(10), None, ud, cd.clone(), bd);
        assert!(ctx.set_chat_data("topic".into(), Value::String("rust".into())).await);
        let store = cd.read().await;
        assert_eq!(store.get(&10).unwrap().get("topic"), Some(&Value::String("rust".into())));
    }

    #[tokio::test]
    async fn set_user_data_returns_false_without_user_id() {
        let bot = make_bot();
        let (ud, cd, bd) = make_stores();
        let ctx = CallbackContext::new(bot, None, None, ud, cd, bd);
        assert!(!ctx.set_user_data("k".into(), Value::Null).await);
    }

    #[test]
    fn match_result_shortcut() {
        let bot = make_bot();
        let (ud, cd, bd) = make_stores();
        let mut ctx = CallbackContext::new(bot, None, None, ud, cd, bd);
        assert!(ctx.match_result().is_none());
        ctx.matches = Some(vec!["hello".into(), "world".into()]);
        assert_eq!(ctx.match_result(), Some("hello"));
    }

    #[cfg(feature = "job-queue")]
    #[test]
    fn with_job_queue() {
        let bot = make_bot();
        let (ud, cd, bd) = make_stores();
        let ctx = CallbackContext::new(bot, None, None, ud, cd, bd);
        let jq = Arc::new(JobQueue::new());
        let ctx = ctx.with_job_queue(jq.clone());
        assert!(ctx.job_queue.is_some());
    }

    // -- Typed guard tests ----------------------------------------------------

    #[tokio::test]
    async fn data_write_guard_typed_setters() {
        let bot = make_bot();
        let (ud, cd, bd) = make_stores();
        let ctx = CallbackContext::new(bot, None, None, ud, cd, bd);

        {
            let mut guard = ctx.bot_data_mut().await;
            guard.set_str("name", "Alice");
            guard.set_i64("score", 42);
            guard.set_bool("active", true);
        }

        let guard = ctx.bot_data().await;
        assert_eq!(guard.get_str("name"), Some("Alice"));
        assert_eq!(guard.get_i64("score"), Some(42));
        assert_eq!(guard.get_bool("active"), Some(true));
    }

    #[tokio::test]
    async fn data_write_guard_id_set_operations() {
        let bot = make_bot();
        let (ud, cd, bd) = make_stores();
        let ctx = CallbackContext::new(bot, None, None, ud, cd, bd);

        {
            let mut guard = ctx.bot_data_mut().await;
            guard.add_to_id_set("user_ids", 100);
            guard.add_to_id_set("user_ids", 200);
            guard.add_to_id_set("user_ids", 100); // duplicate -- should not add
        }

        let guard = ctx.bot_data().await;
        let ids = guard.get_id_set("user_ids");
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&100));
        assert!(ids.contains(&200));

        drop(guard);

        {
            let mut guard = ctx.bot_data_mut().await;
            guard.remove_from_id_set("user_ids", 100);
        }

        let guard = ctx.bot_data().await;
        let ids = guard.get_id_set("user_ids");
        assert_eq!(ids.len(), 1);
        assert!(ids.contains(&200));
    }

    #[tokio::test]
    async fn data_read_guard_empty_id_set() {
        let bot = make_bot();
        let (ud, cd, bd) = make_stores();
        let ctx = CallbackContext::new(bot, None, None, ud, cd, bd);

        let guard = ctx.bot_data().await;
        let ids = guard.get_id_set("nonexistent");
        assert!(ids.is_empty());
    }

    #[tokio::test]
    async fn data_guard_deref_to_hashmap() {
        let bot = make_bot();
        let (ud, cd, bd) = make_stores();
        let ctx = CallbackContext::new(bot, None, None, ud, cd, bd);

        {
            let mut guard = ctx.bot_data_mut().await;
            guard.set_str("key", "val");
        }

        let guard = ctx.bot_data().await;
        // Use Deref to access HashMap methods directly
        assert!(guard.contains_key("key"));
        assert_eq!(guard.len(), 1);
    }
}
