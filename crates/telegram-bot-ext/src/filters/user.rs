//! User identity filter with thread-safe mutable sets.

use std::collections::HashSet;
use std::sync::RwLock;

use crate::filters::base::{Filter, FilterResult, Update};

pub struct UserFilter {
    user_ids: RwLock<HashSet<i64>>,
    usernames: RwLock<HashSet<String>>,
    allow_empty: bool,
}

impl UserFilter {
    pub fn empty(allow_empty: bool) -> Self {
        Self {
            user_ids: RwLock::new(HashSet::new()),
            usernames: RwLock::new(HashSet::new()),
            allow_empty,
        }
    }
    pub fn from_ids(ids: impl IntoIterator<Item = i64>) -> Self {
        Self {
            user_ids: RwLock::new(ids.into_iter().collect()),
            usernames: RwLock::new(HashSet::new()),
            allow_empty: false,
        }
    }
    pub fn from_usernames(names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            user_ids: RwLock::new(HashSet::new()),
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
    pub fn add_user_ids(&self, ids: impl IntoIterator<Item = i64>) {
        self.user_ids.write().unwrap().extend(ids);
    }
    pub fn remove_user_ids(&self, ids: impl IntoIterator<Item = i64>) {
        let mut set = self.user_ids.write().unwrap();
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
    pub fn user_ids(&self) -> HashSet<i64> {
        self.user_ids.read().unwrap().clone()
    }
    pub fn usernames(&self) -> HashSet<String> {
        self.usernames.read().unwrap().clone()
    }
}

impl Filter for UserFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let user = match update
            .effective_message()
            .and_then(|m| m.from_user.as_ref())
        {
            Some(u) => u,
            None => return FilterResult::NoMatch,
        };
        let ids = self.user_ids.read().unwrap();
        if !ids.is_empty() {
            return if ids.contains(&user.id) {
                FilterResult::Match
            } else {
                FilterResult::NoMatch
            };
        }
        let names = self.usernames.read().unwrap();
        if !names.is_empty() {
            return if user
                .username
                .as_deref()
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
        "filters.User"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn user_update(uid: i64, username: Option<&str>) -> Update {
        let mut from = json!({"id": uid, "is_bot": false, "first_name": "Test"});
        if let Some(u) = username {
            from.as_object_mut()
                .unwrap()
                .insert("username".to_owned(), json!(u));
        }
        serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "from": from, "text": "hi"}})).unwrap()
    }

    #[test]
    fn filter_by_id() {
        let f = UserFilter::from_ids([42]);
        assert!(f.check_update(&user_update(42, None)).is_match());
        assert!(!f.check_update(&user_update(99, None)).is_match());
    }
    #[test]
    fn filter_by_username() {
        let f = UserFilter::from_usernames(["@alice"]);
        assert!(f.check_update(&user_update(1, Some("alice"))).is_match());
        assert!(!f.check_update(&user_update(1, Some("bob"))).is_match());
    }
    #[test]
    fn allow_empty_true() {
        let f = UserFilter::empty(true);
        assert!(f.check_update(&user_update(1, None)).is_match());
    }
    #[test]
    fn allow_empty_false() {
        let f = UserFilter::empty(false);
        assert!(!f.check_update(&user_update(1, None)).is_match());
    }
    #[test]
    fn add_remove_ids() {
        let f = UserFilter::empty(false);
        assert!(!f.check_update(&user_update(42, None)).is_match());
        f.add_user_ids([42]);
        assert!(f.check_update(&user_update(42, None)).is_match());
        f.remove_user_ids([42]);
        assert!(!f.check_update(&user_update(42, None)).is_match());
    }
    #[test]
    fn add_remove_usernames() {
        let f = UserFilter::empty(false);
        f.add_usernames(["alice"]);
        assert!(f.check_update(&user_update(1, Some("alice"))).is_match());
        f.remove_usernames(["alice"]);
        assert!(!f.check_update(&user_update(1, Some("alice"))).is_match());
    }
    #[test]
    fn no_from_field() {
        let f = UserFilter::from_ids([1]);
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "text": "anonymous"}})).unwrap();
        assert!(!f.check_update(&update).is_match());
    }
}
