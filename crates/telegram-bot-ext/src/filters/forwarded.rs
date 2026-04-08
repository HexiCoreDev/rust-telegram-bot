//! ForwardedFrom identity filter.

use std::collections::HashSet;
use std::sync::RwLock;

use crate::filters::base::{effective_message_val, to_value, Filter, FilterResult, Update};

pub struct ForwardedFromFilter {
    chat_ids: RwLock<HashSet<i64>>,
    usernames: RwLock<HashSet<String>>,
    allow_empty: bool,
}

impl ForwardedFromFilter {
    pub fn empty(allow_empty: bool) -> Self {
        Self {
            chat_ids: RwLock::new(HashSet::new()),
            usernames: RwLock::new(HashSet::new()),
            allow_empty,
        }
    }

    pub fn from_ids(ids: impl IntoIterator<Item = i64>) -> Self {
        Self {
            chat_ids: RwLock::new(ids.into_iter().collect()),
            usernames: RwLock::new(HashSet::new()),
            allow_empty: false,
        }
    }

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

    pub fn add_chat_ids(&self, ids: impl IntoIterator<Item = i64>) {
        self.chat_ids.write().unwrap().extend(ids);
    }
    pub fn remove_chat_ids(&self, ids: impl IntoIterator<Item = i64>) {
        let mut set = self.chat_ids.write().unwrap();
        for id in ids {
            set.remove(&id);
        }
    }
    pub fn add_usernames(&self, names: impl IntoIterator<Item = impl Into<String>>) {
        let mut set = self.usernames.write().unwrap();
        for n in names {
            let s: String = n.into();
            set.insert(s.strip_prefix('@').unwrap_or(&s).to_owned());
        }
    }
    pub fn remove_usernames(&self, names: impl IntoIterator<Item = impl Into<String>>) {
        let mut set = self.usernames.write().unwrap();
        for n in names {
            let s: String = n.into();
            set.remove(s.strip_prefix('@').unwrap_or(&s));
        }
    }
    pub fn chat_ids(&self) -> HashSet<i64> {
        self.chat_ids.read().unwrap().clone()
    }
    pub fn usernames(&self) -> HashSet<String> {
        self.usernames.read().unwrap().clone()
    }
}

fn origin_id_username(origin: &serde_json::Value) -> (Option<i64>, Option<&str>) {
    let origin_type = origin.get("type").and_then(|v| v.as_str()).unwrap_or("");
    let entity = match origin_type {
        "user" => origin.get("sender_user"),
        "chat" => origin.get("sender_chat"),
        "channel" => origin.get("chat"),
        _ => None,
    };
    match entity {
        Some(e) => (
            e.get("id").and_then(|v| v.as_i64()),
            e.get("username").and_then(|v| v.as_str()),
        ),
        None => (None, None),
    }
}

impl Filter for ForwardedFromFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let __v = to_value(update);
        let origin = match effective_message_val(&__v).and_then(|m| m.get("forward_origin")) {
            Some(o) if !o.is_null() => o,
            _ => return FilterResult::NoMatch,
        };
        let (id, username) = origin_id_username(origin);

        let ids = self.chat_ids.read().unwrap();
        if !ids.is_empty() {
            return if id.map(|i| ids.contains(&i)).unwrap_or(false) {
                FilterResult::Match
            } else {
                FilterResult::NoMatch
            };
        }

        let names = self.usernames.read().unwrap();
        if !names.is_empty() {
            return if username.map(|u| names.contains(u)).unwrap_or(false) {
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
        "filters.ForwardedFrom"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn forwarded_user(uid: i64, username: Option<&str>) -> Update {
        let mut user = json!({"id": uid, "is_bot": false, "first_name": "Fwd"});
        if let Some(u) = username {
            user.as_object_mut()
                .unwrap()
                .insert("username".to_owned(), json!(u));
        }
        serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "forward_origin": {"type": "user", "date": 0, "sender_user": user}, "text": "forwarded"}})).unwrap()
    }

    fn forwarded_channel(chat_id: i64, username: Option<&str>) -> Update {
        let mut chat = json!({"id": chat_id, "type": "channel"});
        if let Some(u) = username {
            chat.as_object_mut()
                .unwrap()
                .insert("username".to_owned(), json!(u));
        }
        serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "forward_origin": {"type": "channel", "date": 0, "chat": chat, "message_id": 42}, "text": "forwarded"}})).unwrap()
    }

    #[test]
    fn by_user_id() {
        let f = ForwardedFromFilter::from_ids([42]);
        assert!(f.check_update(&forwarded_user(42, None)).is_match());
        assert!(!f.check_update(&forwarded_user(99, None)).is_match());
    }
    #[test]
    fn by_username() {
        let f = ForwardedFromFilter::from_usernames(["alice"]);
        assert!(f.check_update(&forwarded_user(1, Some("alice"))).is_match());
        assert!(!f.check_update(&forwarded_user(1, Some("bob"))).is_match());
    }
    #[test]
    fn by_channel_id() {
        let f = ForwardedFromFilter::from_ids([-100]);
        assert!(f.check_update(&forwarded_channel(-100, None)).is_match());
    }
    #[test]
    fn by_channel_username() {
        let f = ForwardedFromFilter::from_usernames(["mychannel"]);
        assert!(f
            .check_update(&forwarded_channel(-100, Some("mychannel")))
            .is_match());
    }
    #[test]
    fn allow_empty() {
        let f = ForwardedFromFilter::empty(true);
        assert!(f.check_update(&forwarded_user(1, None)).is_match());
    }
    #[test]
    fn reject_empty() {
        let f = ForwardedFromFilter::empty(false);
        assert!(!f.check_update(&forwarded_user(1, None)).is_match());
    }

    #[test]
    fn hidden_user_no_match() {
        let f = ForwardedFromFilter::from_ids([42]);
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "forward_origin": {"type": "hidden_user", "date": 0, "sender_user_name": "Hidden"}, "text": "forwarded"}})).unwrap();
        assert!(!f.check_update(&update).is_match());
    }

    #[test]
    fn no_forward_origin() {
        let f = ForwardedFromFilter::from_ids([42]);
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "text": "not forwarded"}})).unwrap();
        assert!(!f.check_update(&update).is_match());
    }

    #[test]
    fn mutation() {
        let f = ForwardedFromFilter::empty(false);
        assert!(!f.check_update(&forwarded_user(42, None)).is_match());
        f.add_chat_ids([42]);
        assert!(f.check_update(&forwarded_user(42, None)).is_match());
        f.remove_chat_ids([42]);
        assert!(!f.check_update(&forwarded_user(42, None)).is_match());
    }

    #[test]
    fn forwarded_from_chat() {
        let f = ForwardedFromFilter::from_ids([-200]);
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "forward_origin": {"type": "chat", "date": 0, "sender_chat": {"id": -200, "type": "supergroup"}}, "text": "forwarded"}})).unwrap();
        assert!(f.check_update(&update).is_match());
    }
}
