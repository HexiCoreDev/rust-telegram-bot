//! Arbitrary callback data cache.
//!
//! Ported from `python-telegram-bot/src/telegram/ext/_callbackdatacache.py`.
//!
//! Stores arbitrary callback data for inline keyboard buttons so that the actual
//! objects (not just short strings) can be passed through the Telegram callback
//! mechanism.  Uses a simple LRU eviction strategy bounded by `maxsize`.

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

use telegram_bot_raw::types::callback_query::CallbackQuery;
use telegram_bot_raw::types::inline::inline_keyboard_button::InlineKeyboardButton;
use telegram_bot_raw::types::inline::inline_keyboard_markup::InlineKeyboardMarkup;

// ---------------------------------------------------------------------------
// UUID generation (SystemTime + atomic counter -- no external crate)
// ---------------------------------------------------------------------------

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generates a 32-hex-char unique id using the current timestamp and an atomic counter.
fn generate_uuid() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{ts:016x}{seq:016x}")
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Raised when the received callback data has been tampered with or deleted from cache.
#[derive(Debug, Clone, thiserror::Error)]
#[error(
    "The object belonging to this callback_data was deleted or the callback_data was manipulated."
)]
pub struct InvalidCallbackData {
    /// The raw callback data string that could not be resolved.
    pub callback_data: Option<String>,
}

// ---------------------------------------------------------------------------
// Internal keyboard metadata
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct KeyboardData {
    keyboard_uuid: String,
    access_time: f64,
    /// Maps button uuid -> arbitrary data stored as `Value`.
    button_data: HashMap<String, Value>,
}

impl KeyboardData {
    fn new(keyboard_uuid: String) -> Self {
        Self {
            keyboard_uuid,
            access_time: now_f64(),
            button_data: HashMap::new(),
        }
    }

    fn update_access_time(&mut self) {
        self.access_time = now_f64();
    }

    fn to_tuple(&self) -> (String, f64, HashMap<String, Value>) {
        (
            self.keyboard_uuid.clone(),
            self.access_time,
            self.button_data.clone(),
        )
    }
}

fn now_f64() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

// ---------------------------------------------------------------------------
// LRU map (simple insertion-order VecDeque + HashMap)
// ---------------------------------------------------------------------------

/// A minimal bounded LRU cache backed by a `HashMap` + `VecDeque` for ordering.
#[derive(Debug, Clone)]
struct LruMap<V> {
    map: HashMap<String, V>,
    order: VecDeque<String>,
    maxsize: usize,
}

impl<V> LruMap<V> {
    fn new(maxsize: usize) -> Self {
        Self {
            map: HashMap::with_capacity(maxsize),
            order: VecDeque::with_capacity(maxsize),
            maxsize,
        }
    }

    fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        if self.map.contains_key(key) {
            // Move to back (most recently used)
            self.order.retain(|k| k != key);
            self.order.push_back(key.to_owned());
            self.map.get_mut(key)
        } else {
            None
        }
    }

    fn insert(&mut self, key: String, value: V) {
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        } else if self.map.len() >= self.maxsize {
            if let Some(evicted) = self.order.pop_front() {
                self.map.remove(&evicted);
            }
        }
        self.order.push_back(key.clone());
        self.map.insert(key, value);
    }

    fn remove(&mut self, key: &str) -> Option<V> {
        if let Some(v) = self.map.remove(key) {
            self.order.retain(|k| k != key);
            Some(v)
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
    }

    fn values(&self) -> impl Iterator<Item = &V> {
        self.map.values()
    }

    fn iter(&self) -> impl Iterator<Item = (&String, &V)> {
        self.map.iter()
    }

    fn retain<F: FnMut(&String, &V) -> bool>(&mut self, mut f: F) {
        let to_remove: Vec<String> = self
            .map
            .iter()
            .filter(|(k, v)| !f(k, v))
            .map(|(k, _)| k.clone())
            .collect();
        for key in &to_remove {
            self.map.remove(key);
        }
        self.order.retain(|k| !to_remove.contains(k));
    }
}

// ---------------------------------------------------------------------------
// Persistence data type alias
// ---------------------------------------------------------------------------

/// Persistent representation of the cache state.
///
/// Tuple of:
/// - list of `(keyboard_uuid, access_time, button_data)` tuples
/// - map of `callback_query_id -> keyboard_uuid`
pub type CdcData = (
    Vec<(String, f64, HashMap<String, Value>)>,
    HashMap<String, String>,
);

// ---------------------------------------------------------------------------
// CallbackDataCache
// ---------------------------------------------------------------------------

/// A custom cache for storing the callback data of an [`ExtBot`](super::ext_bot::ExtBot).
///
/// Internally, it keeps two mappings with fixed maximum size:
///
/// * One for mapping the data received in callback queries to the cached objects.
/// * One for mapping the IDs of received callback queries to the cached objects.
///
/// The second mapping allows manually dropping data cached for keyboards of messages sent via
/// inline mode.  If necessary, the least recently used items are evicted.
#[derive(Debug, Clone)]
pub struct CallbackDataCache {
    keyboard_data: LruMap<KeyboardData>,
    callback_queries: LruMap<String>,
    maxsize: usize,
}

impl CallbackDataCache {
    /// Creates a new `CallbackDataCache`.
    ///
    /// # Arguments
    ///
    /// * `maxsize` - Maximum number of items in each of the internal mappings.
    #[must_use]
    pub fn new(maxsize: usize) -> Self {
        Self {
            keyboard_data: LruMap::new(maxsize),
            callback_queries: LruMap::new(maxsize),
            maxsize,
        }
    }

    /// Loads persisted data into the cache.
    pub fn load_persistence_data(&mut self, data: CdcData) {
        let (keyboard_list, query_map) = data;
        for (uuid, access_time, button_data) in keyboard_list {
            self.keyboard_data.insert(
                uuid.clone(),
                KeyboardData {
                    keyboard_uuid: uuid,
                    access_time,
                    button_data,
                },
            );
        }
        for (qid, kbd_uuid) in query_map {
            self.callback_queries.insert(qid, kbd_uuid);
        }
    }

    /// The maximum size of the cache.
    #[must_use]
    pub fn maxsize(&self) -> usize {
        self.maxsize
    }

    /// Returns the data that needs to be persisted.
    #[must_use]
    pub fn persistence_data(&self) -> CdcData {
        let kbd_list: Vec<_> = self.keyboard_data.values().map(KeyboardData::to_tuple).collect();
        let query_map: HashMap<String, String> = self
            .callback_queries
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        (kbd_list, query_map)
    }

    /// Registers the reply markup in the cache.
    ///
    /// If any of the buttons have `callback_data`, stores that data and builds a new keyboard
    /// with the correspondingly replaced buttons.  Otherwise, returns the original reply markup
    /// unchanged.
    pub fn process_keyboard(&mut self, reply_markup: &InlineKeyboardMarkup) -> InlineKeyboardMarkup {
        let keyboard_uuid = generate_uuid();
        let mut kbd_data = KeyboardData::new(keyboard_uuid.clone());

        let mut new_rows: Vec<Vec<InlineKeyboardButton>> = Vec::new();
        let mut any_replaced = false;

        for row in &reply_markup.inline_keyboard {
            let mut new_row: Vec<InlineKeyboardButton> = Vec::new();
            for btn in row {
                if btn.callback_data.is_some() {
                    let mut btn_copy = btn.clone();
                    let btn_uuid = generate_uuid();
                    kbd_data.button_data.insert(
                        btn_uuid.clone(),
                        Value::String(btn.callback_data.clone().unwrap_or_default()),
                    );
                    btn_copy.callback_data = Some(format!("{keyboard_uuid}{btn_uuid}"));
                    new_row.push(btn_copy);
                    any_replaced = true;
                } else {
                    new_row.push(btn.clone());
                }
            }
            new_rows.push(new_row);
        }

        if !any_replaced {
            return reply_markup.clone();
        }

        self.keyboard_data.insert(keyboard_uuid, kbd_data);

        InlineKeyboardMarkup {
            inline_keyboard: new_rows,
            extra: reply_markup.extra.clone(),
        }
    }

    /// Extracts keyboard uuid and button uuid from a raw callback data string.
    ///
    /// The first 32 characters are the keyboard uuid, the rest is the button uuid.
    #[must_use]
    pub fn extract_uuids(callback_data: &str) -> (&str, &str) {
        if callback_data.len() >= 32 {
            (&callback_data[..32], &callback_data[32..])
        } else {
            (callback_data, "")
        }
    }

    fn get_keyboard_uuid_and_button_data(
        &mut self,
        callback_data: &str,
    ) -> Result<(String, Value), InvalidCallbackData> {
        let (keyboard_uuid, button_uuid) = Self::extract_uuids(callback_data);

        let kbd = self.keyboard_data.get_mut(keyboard_uuid).ok_or_else(|| InvalidCallbackData {
            callback_data: Some(callback_data.to_owned()),
        })?;

        let btn_data = kbd
            .button_data
            .get(button_uuid)
            .cloned()
            .ok_or_else(|| InvalidCallbackData {
                callback_data: Some(callback_data.to_owned()),
            })?;

        kbd.update_access_time();

        Ok((keyboard_uuid.to_owned(), btn_data))
    }

    /// Replaces the data in the inline keyboard attached to a raw JSON message value.
    ///
    /// Works with `Message.reply_markup` being `Option<Value>` (the raw type from the
    /// `telegram-bot-raw` crate).
    ///
    /// Returns the keyboard UUID if resolution succeeded.
    pub fn process_message_value(&mut self, message: &mut Value) -> Option<String> {
        let rm = message.get_mut("reply_markup")?;
        if rm.is_null() {
            return None;
        }

        // Try to deserialize as InlineKeyboardMarkup
        let mut markup: InlineKeyboardMarkup = serde_json::from_value(rm.clone()).ok()?;

        let mut keyboard_uuid: Option<String> = None;

        for row in &mut markup.inline_keyboard {
            for button in row {
                if let Some(ref raw_data) = button.callback_data.clone() {
                    match self.get_keyboard_uuid_and_button_data(raw_data) {
                        Ok((kbd_id, data)) => {
                            button.callback_data = Some(data.to_string());
                            if keyboard_uuid.is_none() {
                                keyboard_uuid = Some(kbd_id);
                            }
                        }
                        Err(_) => {
                            button.callback_data = None;
                        }
                    }
                }
            }
        }

        // Write back the modified markup
        if let Ok(v) = serde_json::to_value(&markup) {
            *rm = v;
        }

        keyboard_uuid
    }

    /// Replaces the data in the callback query (and attached message keyboard) with cached
    /// objects.
    ///
    /// **In place** -- modifies the passed `CallbackQuery`.
    pub fn process_callback_query(&mut self, callback_query: &mut CallbackQuery) {
        if let Some(ref raw_data) = callback_query.data.clone() {
            match self.get_keyboard_uuid_and_button_data(raw_data) {
                Ok((kbd_uuid, data)) => {
                    callback_query.data = Some(data.to_string());
                    self.callback_queries
                        .insert(callback_query.id.clone(), kbd_uuid);
                }
                Err(_) => {
                    callback_query.data = None;
                }
            }
        }

        // Process the attached message (as raw Value via the `message` field).
        if let Some(ref mut msg) = callback_query.message {
            // The message is Box<MaybeInaccessibleMessage> which contains reply_markup: Option<Value>.
            // We need to convert to a Value, process it, and write it back.
            if let Ok(mut msg_val) = serde_json::to_value(&**msg) {
                self.process_message_value(&mut msg_val);
                if let Ok(processed_msg) = serde_json::from_value::<telegram_bot_raw::types::message::MaybeInaccessibleMessage>(msg_val) {
                    **msg = processed_msg;
                }
            }
        }
    }

    /// Deletes the data for the specified callback query.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the callback query is not found in the cache.
    pub fn drop_data(&mut self, callback_query_id: &str) -> Result<(), InvalidCallbackData> {
        let kbd_uuid = self
            .callback_queries
            .remove(callback_query_id)
            .ok_or(InvalidCallbackData {
                callback_data: None,
            })?;

        // Silently ignore if the keyboard itself is already gone.
        let _ = self.keyboard_data.remove(&kbd_uuid);
        Ok(())
    }

    /// Clears the stored callback data.
    ///
    /// If `time_cutoff` is provided, only entries older than that UNIX timestamp are cleared.
    pub fn clear_callback_data(&mut self, time_cutoff: Option<f64>) {
        match time_cutoff {
            None => self.keyboard_data.clear(),
            Some(cutoff) => {
                self.keyboard_data
                    .retain(|_, v| v.access_time >= cutoff);
            }
        }
    }

    /// Clears all stored callback query IDs.
    pub fn clear_callback_queries(&mut self) {
        self.callback_queries.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_generation_is_unique() {
        let a = generate_uuid();
        let b = generate_uuid();
        assert_ne!(a, b);
        assert_eq!(a.len(), 32);
    }

    #[test]
    fn extract_uuids_splits_correctly() {
        let combined = format!("{}{}", "a".repeat(32), "b".repeat(32));
        let (kbd, btn) = CallbackDataCache::extract_uuids(&combined);
        assert_eq!(kbd, "a".repeat(32));
        assert_eq!(btn, "b".repeat(32));
    }

    #[test]
    fn process_keyboard_replaces_callback_data() {
        let mut cache = CallbackDataCache::new(128);

        let markup = InlineKeyboardMarkup {
            inline_keyboard: vec![vec![InlineKeyboardButton {
                text: "Click".into(),
                callback_data: Some("my_data".into()),
                ..Default::default()
            }]],
            extra: HashMap::new(),
        };

        let new_markup = cache.process_keyboard(&markup);
        let new_data = new_markup.inline_keyboard[0][0]
            .callback_data
            .as_ref()
            .unwrap();

        // The replaced data should be 64 chars (keyboard_uuid + button_uuid).
        assert_eq!(new_data.len(), 64);
        assert_ne!(new_data, "my_data");
    }

    #[test]
    fn process_keyboard_noop_without_callback_data() {
        let mut cache = CallbackDataCache::new(128);

        let markup = InlineKeyboardMarkup {
            inline_keyboard: vec![vec![InlineKeyboardButton {
                text: "URL".into(),
                url: Some("https://example.com".into()),
                ..Default::default()
            }]],
            extra: HashMap::new(),
        };

        let new_markup = cache.process_keyboard(&markup);
        assert_eq!(new_markup.inline_keyboard[0][0].url, markup.inline_keyboard[0][0].url);
    }

    #[test]
    fn roundtrip_process_and_resolve() {
        let mut cache = CallbackDataCache::new(128);

        let markup = InlineKeyboardMarkup {
            inline_keyboard: vec![vec![InlineKeyboardButton {
                text: "Click".into(),
                callback_data: Some("original".into()),
                ..Default::default()
            }]],
            extra: HashMap::new(),
        };

        let new_markup = cache.process_keyboard(&markup);
        let uuid_data = new_markup.inline_keyboard[0][0]
            .callback_data
            .clone()
            .unwrap();

        // Simulate receiving the callback query
        let mut cq = CallbackQuery {
            id: "query_1".into(),
            from_user: telegram_bot_raw::types::user::User {
                id: 1,
                is_bot: false,
                first_name: "Test".into(),
                last_name: None,
                username: None,
                language_code: None,
                can_join_groups: None,
                can_read_all_group_messages: None,
                supports_inline_queries: None,
                is_premium: None,
                added_to_attachment_menu: None,
                can_connect_to_business: None,
                has_main_web_app: None,
                has_topics_enabled: None,
                allows_users_to_create_topics: None,
                can_manage_bots: None,
                extra: HashMap::new(),
            },
            chat_instance: "inst".into(),
            message: None,
            data: Some(uuid_data),
            inline_message_id: None,
            game_short_name: None,
            extra: HashMap::new(),
        };

        cache.process_callback_query(&mut cq);

        // The data should now be the JSON of the original string.
        assert_eq!(cq.data.as_deref(), Some("\"original\""));
    }

    #[test]
    fn drop_data_removes_entry() {
        let mut cache = CallbackDataCache::new(128);

        let markup = InlineKeyboardMarkup {
            inline_keyboard: vec![vec![InlineKeyboardButton {
                text: "Click".into(),
                callback_data: Some("payload".into()),
                ..Default::default()
            }]],
            extra: HashMap::new(),
        };

        let new_markup = cache.process_keyboard(&markup);
        let uuid_data = new_markup.inline_keyboard[0][0]
            .callback_data
            .clone()
            .unwrap();

        let mut cq = CallbackQuery {
            id: "q2".into(),
            from_user: telegram_bot_raw::types::user::User {
                id: 1,
                is_bot: false,
                first_name: "T".into(),
                last_name: None,
                username: None,
                language_code: None,
                can_join_groups: None,
                can_read_all_group_messages: None,
                supports_inline_queries: None,
                is_premium: None,
                added_to_attachment_menu: None,
                can_connect_to_business: None,
                has_main_web_app: None,
                has_topics_enabled: None,
                allows_users_to_create_topics: None,
                can_manage_bots: None,
                extra: HashMap::new(),
            },
            chat_instance: "i".into(),
            message: None,
            data: Some(uuid_data),
            inline_message_id: None,
            game_short_name: None,
            extra: HashMap::new(),
        };

        cache.process_callback_query(&mut cq);
        assert!(cache.drop_data("q2").is_ok());
        assert!(cache.drop_data("q2").is_err());
    }

    #[test]
    fn lru_eviction() {
        let mut cache = CallbackDataCache::new(2);

        for i in 0..3 {
            let markup = InlineKeyboardMarkup {
                inline_keyboard: vec![vec![InlineKeyboardButton {
                    text: format!("btn_{i}"),
                    callback_data: Some(format!("data_{i}")),
                    ..Default::default()
                }]],
                extra: HashMap::new(),
            };
            cache.process_keyboard(&markup);
        }

        // Only 2 keyboards should remain in the cache.
        assert_eq!(cache.keyboard_data.map.len(), 2);
    }

    #[test]
    fn persistence_roundtrip() {
        let mut cache = CallbackDataCache::new(128);

        let markup = InlineKeyboardMarkup {
            inline_keyboard: vec![vec![InlineKeyboardButton {
                text: "Click".into(),
                callback_data: Some("persist_me".into()),
                ..Default::default()
            }]],
            extra: HashMap::new(),
        };

        cache.process_keyboard(&markup);
        let persisted = cache.persistence_data();

        let mut cache2 = CallbackDataCache::new(128);
        cache2.load_persistence_data(persisted);

        assert_eq!(cache2.keyboard_data.map.len(), 1);
    }

    #[test]
    fn clear_with_cutoff() {
        let mut cache = CallbackDataCache::new(128);

        let markup = InlineKeyboardMarkup {
            inline_keyboard: vec![vec![InlineKeyboardButton {
                text: "Old".into(),
                callback_data: Some("old_data".into()),
                ..Default::default()
            }]],
            extra: HashMap::new(),
        };

        cache.process_keyboard(&markup);

        // Clearing with a far-future cutoff should remove everything.
        cache.clear_callback_data(Some(f64::MAX));
        assert_eq!(cache.keyboard_data.map.len(), 0);
    }
}
