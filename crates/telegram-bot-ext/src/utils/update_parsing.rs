//! Helper functions for parsing update-related inputs.
//!
//! Port of `telegram.ext._utils._update_parsing`.
//! These are library-internal utilities for normalising handler filter inputs.

use std::collections::HashSet;

/// Accept a single chat ID or a collection of chat IDs and return a
/// `HashSet<i64>`.
///
/// If `None` is passed, an empty set is returned.
pub fn parse_chat_id(chat_id: Option<SingleOrCollection<i64>>) -> HashSet<i64> {
    match chat_id {
        None => HashSet::new(),
        Some(SingleOrCollection::Single(id)) => {
            let mut set = HashSet::with_capacity(1);
            set.insert(id);
            set
        }
        Some(SingleOrCollection::Collection(ids)) => ids.into_iter().collect(),
    }
}

/// Accept a single username or a collection of usernames and return a
/// `HashSet<String>` with the leading `@` stripped.
///
/// If `None` is passed, an empty set is returned.
pub fn parse_username(username: Option<SingleOrCollection<String>>) -> HashSet<String> {
    match username {
        None => HashSet::new(),
        Some(SingleOrCollection::Single(u)) => {
            let mut set = HashSet::with_capacity(1);
            set.insert(strip_at(u));
            set
        }
        Some(SingleOrCollection::Collection(us)) => us.into_iter().map(strip_at).collect(),
    }
}

/// Mirrors Python's `SCT[T]` (Single-or-Collection-of-T).
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SingleOrCollection<T> {
    Single(T),
    Collection(Vec<T>),
}

impl<T> From<T> for SingleOrCollection<T> {
    fn from(value: T) -> Self {
        Self::Single(value)
    }
}

impl<T> From<Vec<T>> for SingleOrCollection<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Collection(value)
    }
}

/// Strip a leading `@` from a username string.
fn strip_at(mut s: String) -> String {
    if s.starts_with('@') {
        s.remove(0);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_chat_id_none() {
        assert!(parse_chat_id(None).is_empty());
    }

    #[test]
    fn parse_chat_id_single() {
        let result = parse_chat_id(Some(42i64.into()));
        assert_eq!(result.len(), 1);
        assert!(result.contains(&42));
    }

    #[test]
    fn parse_chat_id_collection() {
        let result = parse_chat_id(Some(vec![1i64, 2, 3].into()));
        assert_eq!(result.len(), 3);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn parse_username_strips_at() {
        let result = parse_username(Some("@testuser".to_owned().into()));
        assert!(result.contains("testuser"));
    }

    #[test]
    fn parse_username_no_at() {
        let result = parse_username(Some("testuser".to_owned().into()));
        assert!(result.contains("testuser"));
    }

    #[test]
    fn parse_username_collection() {
        let input = vec!["@alice".to_owned(), "bob".to_owned()];
        let result = parse_username(Some(input.into()));
        assert!(result.contains("alice"));
        assert!(result.contains("bob"));
    }
}
