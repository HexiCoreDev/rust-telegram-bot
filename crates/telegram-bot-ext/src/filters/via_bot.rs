//! ViaBot identity filter.

use std::collections::HashSet;
use std::sync::RwLock;

use crate::filters::base::{Filter, FilterResult, Update};

pub struct ViaBotFilter {
    bot_ids: RwLock<HashSet<i64>>,
    usernames: RwLock<HashSet<String>>,
    allow_empty: bool,
}

impl ViaBotFilter {
    pub fn empty(allow_empty: bool) -> Self {
        Self {
            bot_ids: RwLock::new(HashSet::new()),
            usernames: RwLock::new(HashSet::new()),
            allow_empty,
        }
    }
    pub fn from_ids(ids: impl IntoIterator<Item = i64>) -> Self {
        Self {
            bot_ids: RwLock::new(ids.into_iter().collect()),
            usernames: RwLock::new(HashSet::new()),
            allow_empty: false,
        }
    }
    pub fn from_usernames(names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            bot_ids: RwLock::new(HashSet::new()),
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
    pub fn add_bot_ids(&self, ids: impl IntoIterator<Item = i64>) {
        self.bot_ids.write().unwrap().extend(ids);
    }
    pub fn remove_bot_ids(&self, ids: impl IntoIterator<Item = i64>) {
        let mut set = self.bot_ids.write().unwrap();
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
    pub fn bot_ids(&self) -> HashSet<i64> {
        self.bot_ids.read().unwrap().clone()
    }
    pub fn usernames(&self) -> HashSet<String> {
        self.usernames.read().unwrap().clone()
    }
}

impl Filter for ViaBotFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let via_bot = match update.effective_message().and_then(|m| m.via_bot.as_ref()) {
            Some(vb) => vb,
            None => return FilterResult::NoMatch,
        };
        let ids = self.bot_ids.read().unwrap();
        if !ids.is_empty() {
            return if ids.contains(&via_bot.id) {
                FilterResult::Match
            } else {
                FilterResult::NoMatch
            };
        }
        let names = self.usernames.read().unwrap();
        if !names.is_empty() {
            return if via_bot
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
        "filters.ViaBot"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn via_bot_update(bot_id: i64, username: Option<&str>) -> Update {
        let mut bot = json!({"id": bot_id, "is_bot": true, "first_name": "InlineBot"});
        if let Some(u) = username {
            bot.as_object_mut()
                .unwrap()
                .insert("username".to_owned(), json!(u));
        }
        serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "via_bot": bot, "text": "inline result"}})).unwrap()
    }

    #[test]
    fn by_bot_id() {
        let f = ViaBotFilter::from_ids([100]);
        assert!(f.check_update(&via_bot_update(100, None)).is_match());
        assert!(!f.check_update(&via_bot_update(200, None)).is_match());
    }
    #[test]
    fn by_username() {
        let f = ViaBotFilter::from_usernames(["@inlinebot"]);
        assert!(f
            .check_update(&via_bot_update(1, Some("inlinebot")))
            .is_match());
        assert!(!f
            .check_update(&via_bot_update(1, Some("otherbot")))
            .is_match());
    }
    #[test]
    fn allow_empty() {
        let f = ViaBotFilter::empty(true);
        assert!(f.check_update(&via_bot_update(1, None)).is_match());
    }
    #[test]
    fn reject_empty() {
        let f = ViaBotFilter::empty(false);
        assert!(!f.check_update(&via_bot_update(1, None)).is_match());
    }
    #[test]
    fn no_via_bot() {
        let f = ViaBotFilter::from_ids([1]);
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "text": "no bot"}})).unwrap();
        assert!(!f.check_update(&update).is_match());
    }
    #[test]
    fn mutation() {
        let f = ViaBotFilter::empty(false);
        assert!(!f.check_update(&via_bot_update(42, None)).is_match());
        f.add_bot_ids([42]);
        assert!(f.check_update(&via_bot_update(42, None)).is_match());
        f.remove_bot_ids([42]);
        assert!(!f.check_update(&via_bot_update(42, None)).is_match());
    }
}
