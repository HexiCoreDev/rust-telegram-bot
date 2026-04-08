//! Chat identity filter and ChatType namespace.
//!
//! - [`ChatFilter`] -- restrict to specific chat IDs or usernames (`RwLock`).
//! - [`ChatType`] namespace -- `CHANNEL`, `GROUP`, `GROUPS`, `PRIVATE`, `SUPERGROUP`.
//! - [`SenderChatFilter`] -- restrict by `sender_chat` ID / username.
//! - Sender-chat sub-filters: `SenderChatChannel`, `SenderChatSuperGroup`.

use std::collections::HashSet;
use std::sync::RwLock;

use crate::filters::base::{effective_message_val, to_value, Filter, FilterResult, Update};

// ---------------------------------------------------------------------------
// ChatFilter (identity)
// ---------------------------------------------------------------------------

/// Filters messages from specific chats, identified by ID or username.
///
/// Thread-safe: uses [`RwLock`] for internal mutation.
pub struct ChatFilter {
    chat_ids: RwLock<HashSet<i64>>,
    usernames: RwLock<HashSet<String>>,
    allow_empty: bool,
}

impl ChatFilter {
    /// Create an empty filter.
    pub fn empty(allow_empty: bool) -> Self {
        Self {
            chat_ids: RwLock::new(HashSet::new()),
            usernames: RwLock::new(HashSet::new()),
            allow_empty,
        }
    }

    /// Create a filter from chat IDs.
    pub fn from_ids(ids: impl IntoIterator<Item = i64>) -> Self {
        Self {
            chat_ids: RwLock::new(ids.into_iter().collect()),
            usernames: RwLock::new(HashSet::new()),
            allow_empty: false,
        }
    }

    /// Create a filter from usernames (leading `@` stripped).
    pub fn from_usernames(names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            chat_ids: RwLock::new(HashSet::new()),
            usernames: RwLock::new(
                names
                    .into_iter()
                    .map(|n| {
                        let s: String = n.into();
                        s.strip_prefix('@').unwrap_or(&s).to_owned()
                    })
                    .collect(),
            ),
            allow_empty: false,
        }
    }

    /// Add chat IDs at runtime.
    pub fn add_chat_ids(&self, ids: impl IntoIterator<Item = i64>) {
        let mut set = self.chat_ids.write().unwrap();
        set.extend(ids);
    }

    /// Remove chat IDs at runtime.
    pub fn remove_chat_ids(&self, ids: impl IntoIterator<Item = i64>) {
        let mut set = self.chat_ids.write().unwrap();
        for id in ids {
            set.remove(&id);
        }
    }

    /// Add usernames at runtime (leading `@` stripped).
    pub fn add_usernames(&self, names: impl IntoIterator<Item = impl Into<String>>) {
        let mut set = self.usernames.write().unwrap();
        for n in names {
            let s: String = n.into();
            set.insert(s.strip_prefix('@').unwrap_or(&s).to_owned());
        }
    }

    /// Remove usernames at runtime (leading `@` stripped).
    pub fn remove_usernames(&self, names: impl IntoIterator<Item = impl Into<String>>) {
        let mut set = self.usernames.write().unwrap();
        for n in names {
            let s: String = n.into();
            set.remove(s.strip_prefix('@').unwrap_or(&s));
        }
    }

    /// Snapshot of current chat-ID set.
    pub fn chat_ids(&self) -> HashSet<i64> {
        self.chat_ids.read().unwrap().clone()
    }

    /// Snapshot of current username set.
    pub fn usernames(&self) -> HashSet<String> {
        self.usernames.read().unwrap().clone()
    }
}

impl Filter for ChatFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let __v = to_value(update);
        let chat = match effective_message_val(&__v).and_then(|m| m.get("chat")) {
            Some(c) if !c.is_null() => c,
            _ => return FilterResult::NoMatch,
        };

        let ids = self.chat_ids.read().unwrap();
        if !ids.is_empty() {
            return if chat
                .get("id")
                .and_then(|v| v.as_i64())
                .map(|id| ids.contains(&id))
                .unwrap_or(false)
            {
                FilterResult::Match
            } else {
                FilterResult::NoMatch
            };
        }

        let names = self.usernames.read().unwrap();
        if !names.is_empty() {
            return if chat
                .get("username")
                .and_then(|v| v.as_str())
                .map(|u| names.contains(u))
                .unwrap_or(false)
            {
                FilterResult::Match
            } else {
                FilterResult::NoMatch
            };
        }

        if self.allow_empty {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.Chat"
    }
}

// ---------------------------------------------------------------------------
// ChatType namespace
// ---------------------------------------------------------------------------

macro_rules! chat_type_filter {
    ($(#[$meta:meta])* $struct_name:ident, $chat_type:expr, $display:expr) => {
        $(#[$meta])*
        pub struct $struct_name;

        impl Filter for $struct_name {
            fn check_update(&self, update: &Update) -> FilterResult { let __v = to_value(update);
                if effective_message_val(&__v)
                    .and_then(|m| m.get("chat"))
                    .and_then(|c| c.get("type"))
                    .and_then(|v| v.as_str())
                    == Some($chat_type)
                {
                    FilterResult::Match
                } else {
                    FilterResult::NoMatch
                }
            }

            fn name(&self) -> &str {
                $display
            }
        }
    };
}

chat_type_filter!(
    /// `filters.ChatType.CHANNEL` -- updates from channels.
    ChatTypeChannel, "channel", "filters.ChatType.CHANNEL"
);

chat_type_filter!(
    /// `filters.ChatType.GROUP` -- updates from basic groups.
    ChatTypeGroup, "group", "filters.ChatType.GROUP"
);

chat_type_filter!(
    /// `filters.ChatType.PRIVATE` -- updates from private chats.
    ChatTypePrivate, "private", "filters.ChatType.PRIVATE"
);

chat_type_filter!(
    /// `filters.ChatType.SUPERGROUP` -- updates from supergroups.
    ChatTypeSuperGroup, "supergroup", "filters.ChatType.SUPERGROUP"
);

/// `filters.ChatType.GROUPS` -- updates from groups *or* supergroups.
pub struct ChatTypeGroups;

impl Filter for ChatTypeGroups {
    fn check_update(&self, update: &Update) -> FilterResult {
        let __v = to_value(update);
        let chat_type = effective_message_val(&__v)
            .and_then(|m| m.get("chat"))
            .and_then(|c| c.get("type"))
            .and_then(|v| v.as_str());
        if matches!(chat_type, Some("group") | Some("supergroup")) {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.ChatType.GROUPS"
    }
}

/// Convenience namespace grouping all `ChatType` constants.
pub mod chat_type {
    use super::*;

    pub const CHANNEL: ChatTypeChannel = ChatTypeChannel;
    pub const GROUP: ChatTypeGroup = ChatTypeGroup;
    pub const GROUPS: ChatTypeGroups = ChatTypeGroups;
    pub const PRIVATE: ChatTypePrivate = ChatTypePrivate;
    pub const SUPERGROUP: ChatTypeSuperGroup = ChatTypeSuperGroup;
}

// ---------------------------------------------------------------------------
// SenderChat identity filter
// ---------------------------------------------------------------------------

/// Filters messages by `sender_chat` ID or username.
///
/// Thread-safe: uses [`RwLock`].
pub struct SenderChatFilter {
    chat_ids: RwLock<HashSet<i64>>,
    usernames: RwLock<HashSet<String>>,
    allow_empty: bool,
}

impl SenderChatFilter {
    /// Create an empty filter.
    pub fn empty(allow_empty: bool) -> Self {
        Self {
            chat_ids: RwLock::new(HashSet::new()),
            usernames: RwLock::new(HashSet::new()),
            allow_empty,
        }
    }

    /// Create from chat IDs.
    pub fn from_ids(ids: impl IntoIterator<Item = i64>) -> Self {
        Self {
            chat_ids: RwLock::new(ids.into_iter().collect()),
            usernames: RwLock::new(HashSet::new()),
            allow_empty: false,
        }
    }

    /// Create from usernames.
    pub fn from_usernames(names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            chat_ids: RwLock::new(HashSet::new()),
            usernames: RwLock::new(
                names
                    .into_iter()
                    .map(|n| {
                        let s: String = n.into();
                        s.strip_prefix('@').unwrap_or(&s).to_owned()
                    })
                    .collect(),
            ),
            allow_empty: false,
        }
    }

    /// Add chat IDs at runtime.
    pub fn add_chat_ids(&self, ids: impl IntoIterator<Item = i64>) {
        let mut set = self.chat_ids.write().unwrap();
        set.extend(ids);
    }

    /// Remove chat IDs at runtime.
    pub fn remove_chat_ids(&self, ids: impl IntoIterator<Item = i64>) {
        let mut set = self.chat_ids.write().unwrap();
        for id in ids {
            set.remove(&id);
        }
    }

    /// Add usernames at runtime.
    pub fn add_usernames(&self, names: impl IntoIterator<Item = impl Into<String>>) {
        let mut set = self.usernames.write().unwrap();
        for n in names {
            let s: String = n.into();
            set.insert(s.strip_prefix('@').unwrap_or(&s).to_owned());
        }
    }

    /// Remove usernames at runtime.
    pub fn remove_usernames(&self, names: impl IntoIterator<Item = impl Into<String>>) {
        let mut set = self.usernames.write().unwrap();
        for n in names {
            let s: String = n.into();
            set.remove(s.strip_prefix('@').unwrap_or(&s));
        }
    }
}

impl Filter for SenderChatFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let __v = to_value(update);
        let sender_chat = match effective_message_val(&__v).and_then(|m| m.get("sender_chat")) {
            Some(sc) if !sc.is_null() => sc,
            _ => return FilterResult::NoMatch,
        };

        let ids = self.chat_ids.read().unwrap();
        if !ids.is_empty() {
            return if sender_chat
                .get("id")
                .and_then(|v| v.as_i64())
                .map(|id| ids.contains(&id))
                .unwrap_or(false)
            {
                FilterResult::Match
            } else {
                FilterResult::NoMatch
            };
        }

        let names = self.usernames.read().unwrap();
        if !names.is_empty() {
            return if sender_chat
                .get("username")
                .and_then(|v| v.as_str())
                .map(|u| names.contains(u))
                .unwrap_or(false)
            {
                FilterResult::Match
            } else {
                FilterResult::NoMatch
            };
        }

        if self.allow_empty {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.SenderChat"
    }
}

// ---------------------------------------------------------------------------
// SenderChat sub-filters (channel / supergroup)
// ---------------------------------------------------------------------------

/// `filters.SenderChat.CHANNEL` -- messages whose sender_chat is a channel.
pub struct SenderChatChannel;

impl Filter for SenderChatChannel {
    fn check_update(&self, update: &Update) -> FilterResult {
        let __v = to_value(update);
        if effective_message_val(&__v)
            .and_then(|m| m.get("sender_chat"))
            .and_then(|sc| sc.get("type"))
            .and_then(|v| v.as_str())
            == Some("channel")
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.SenderChat.CHANNEL"
    }
}

/// `filters.SenderChat.SUPER_GROUP` -- messages whose sender_chat is a supergroup.
pub struct SenderChatSuperGroup;

impl Filter for SenderChatSuperGroup {
    fn check_update(&self, update: &Update) -> FilterResult {
        let __v = to_value(update);
        if effective_message_val(&__v)
            .and_then(|m| m.get("sender_chat"))
            .and_then(|sc| sc.get("type"))
            .and_then(|v| v.as_str())
            == Some("supergroup")
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.SenderChat.SUPER_GROUP"
    }
}

/// Convenience namespace for sender-chat sub-filters.
pub mod sender_chat {
    use super::*;

    /// Matches any message with a `sender_chat`.
    pub const ALL: crate::filters::base::SenderChatPresence =
        crate::filters::base::SenderChatPresence;
    pub const CHANNEL: SenderChatChannel = SenderChatChannel;
    pub const SUPER_GROUP: SenderChatSuperGroup = SenderChatSuperGroup;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn chat_update(chat_id: i64, chat_type: &str, username: Option<&str>) -> Update {
        let mut chat = json!({"id": chat_id, "type": chat_type});
        if let Some(u) = username {
            chat.as_object_mut()
                .unwrap()
                .insert("username".to_owned(), json!(u));
        }
        serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": chat,
                "text": "hi"
            }
        }))
        .unwrap()
    }

    #[test]
    fn chat_filter_by_id() {
        let f = ChatFilter::from_ids([-1234]);
        assert!(f
            .check_update(&chat_update(-1234, "supergroup", None))
            .is_match());
        assert!(!f
            .check_update(&chat_update(-5678, "supergroup", None))
            .is_match());
    }

    #[test]
    fn chat_filter_by_username() {
        let f = ChatFilter::from_usernames(["mychat"]);
        assert!(f
            .check_update(&chat_update(-1, "supergroup", Some("mychat")))
            .is_match());
        assert!(!f
            .check_update(&chat_update(-1, "supergroup", Some("other")))
            .is_match());
    }

    #[test]
    fn chat_filter_allow_empty() {
        let f = ChatFilter::empty(true);
        assert!(f.check_update(&chat_update(1, "private", None)).is_match());
    }

    #[test]
    fn chat_filter_mutation() {
        let f = ChatFilter::empty(false);
        assert!(!f.check_update(&chat_update(42, "private", None)).is_match());
        f.add_chat_ids([42]);
        assert!(f.check_update(&chat_update(42, "private", None)).is_match());
        f.remove_chat_ids([42]);
        assert!(!f.check_update(&chat_update(42, "private", None)).is_match());
    }

    #[test]
    fn chat_type_channel() {
        assert!(chat_type::CHANNEL
            .check_update(&chat_update(1, "channel", None))
            .is_match());
        assert!(!chat_type::CHANNEL
            .check_update(&chat_update(1, "private", None))
            .is_match());
    }

    #[test]
    fn chat_type_group() {
        assert!(chat_type::GROUP
            .check_update(&chat_update(1, "group", None))
            .is_match());
        assert!(!chat_type::GROUP
            .check_update(&chat_update(1, "supergroup", None))
            .is_match());
    }

    #[test]
    fn chat_type_groups() {
        assert!(chat_type::GROUPS
            .check_update(&chat_update(1, "group", None))
            .is_match());
        assert!(chat_type::GROUPS
            .check_update(&chat_update(1, "supergroup", None))
            .is_match());
        assert!(!chat_type::GROUPS
            .check_update(&chat_update(1, "private", None))
            .is_match());
    }

    #[test]
    fn chat_type_private() {
        assert!(chat_type::PRIVATE
            .check_update(&chat_update(1, "private", None))
            .is_match());
    }

    #[test]
    fn chat_type_supergroup() {
        assert!(chat_type::SUPERGROUP
            .check_update(&chat_update(1, "supergroup", None))
            .is_match());
    }

    #[test]
    fn sender_chat_channel() {
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "supergroup"},
                "sender_chat": {"id": -100, "type": "channel"},
                "text": "forwarded"
            }
        }))
        .unwrap();
        assert!(SenderChatChannel.check_update(&update).is_match());
        assert!(!SenderChatSuperGroup.check_update(&update).is_match());
    }

    #[test]
    fn sender_chat_filter_by_id() {
        let f = SenderChatFilter::from_ids([-100]);
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "supergroup"},
                "sender_chat": {"id": -100, "type": "channel"},
                "text": "hi"
            }
        }))
        .unwrap();
        assert!(f.check_update(&update).is_match());
    }
}
