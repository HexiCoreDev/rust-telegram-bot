//! A mutable mapping that tracks which keys have been written to.
//!
//! Port of `telegram.ext._utils.trackingdict.TrackingDict`.
//! Read access is **not** tracked; only mutations (`insert`, `remove`,
//! `clear`, etc.) mark keys as dirty.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Sentinel value returned by [`TrackingDict::pop_accessed_write_items`]
/// when an entry was deleted rather than updated.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EntryValue<V> {
    /// The key still exists and holds this value.
    Value(V),
    /// The key was deleted since the last drain.
    Deleted,
}

/// A `HashMap` wrapper that records which keys have been mutated.
#[derive(Debug, Clone)]
pub struct TrackingDict<K, V> {
    data: HashMap<K, V>,
    dirty: HashSet<K>,
}

impl<K, V> Default for TrackingDict<K, V>
where
    K: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> TrackingDict<K, V>
where
    K: Eq + Hash + Clone,
{
    /// Create an empty `TrackingDict`.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            dirty: HashSet::new(),
        }
    }

    /// Create a `TrackingDict` pre-populated with `data`.
    /// None of the initial keys are considered dirty.
    pub fn from_map(data: HashMap<K, V>) -> Self {
        Self {
            data,
            dirty: HashSet::new(),
        }
    }

    // ------------------------------------------------------------------
    // Read access (not tracked)
    // ------------------------------------------------------------------

    /// Returns a reference to the value for the given key, if present.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    /// Returns `true` if the map contains the key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Iterate over all `(key, value)` pairs. Not tracked.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.data.iter()
    }

    /// Returns a reference to the underlying `HashMap`. Not tracked.
    pub fn inner(&self) -> &HashMap<K, V> {
        &self.data
    }

    // ------------------------------------------------------------------
    // Write access (tracked)
    // ------------------------------------------------------------------

    /// Insert a key-value pair. Marks `key` as dirty.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.dirty.insert(key.clone());
        self.data.insert(key, value)
    }

    /// Remove a key. Marks `key` as dirty if it was present.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.data.contains_key(key) {
            self.dirty.insert(key.clone());
        }
        self.data.remove(key)
    }

    /// Clear all entries. Marks every existing key as dirty.
    pub fn clear(&mut self) {
        for key in self.data.keys() {
            self.dirty.insert(key.clone());
        }
        self.data.clear();
    }

    /// Like `HashMap::entry`, but marks the key as dirty on any insertion.
    pub fn set_default(&mut self, key: K, default: V) -> &mut V
    where
        V: Clone,
    {
        if !self.data.contains_key(&key) {
            self.dirty.insert(key.clone());
            self.data.insert(key.clone(), default);
        }
        self.data.get_mut(&key).expect("just inserted")
    }

    // ------------------------------------------------------------------
    // Bulk update without tracking
    // ------------------------------------------------------------------

    /// Merge entries from `other` without marking any key as dirty.
    /// Equivalent to Python's `update_no_track`.
    pub fn update_no_track(&mut self, other: HashMap<K, V>) {
        for (k, v) in other {
            self.data.insert(k, v);
        }
    }

    // ------------------------------------------------------------------
    // Dirty-key access
    // ------------------------------------------------------------------

    /// Manually mark a key as dirty so it appears in the next drain.
    pub fn mark_as_accessed(&mut self, key: K) {
        self.dirty.insert(key);
    }

    /// Drain and return all keys that have been written to since the last
    /// call to this method (or since construction).
    pub fn pop_accessed_keys(&mut self) -> HashSet<K> {
        std::mem::take(&mut self.dirty)
    }

    /// Drain dirty keys together with their current values.
    /// If a key was deleted, the value is [`EntryValue::Deleted`].
    pub fn pop_accessed_write_items(&mut self) -> Vec<(K, EntryValue<V>)>
    where
        V: Clone,
    {
        let keys = self.pop_accessed_keys();
        keys.into_iter()
            .map(|k| {
                let v = self
                    .data
                    .get(&k)
                    .map_or(EntryValue::Deleted, |v| EntryValue::Value(v.clone()));
                (k, v)
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Trait implementations
// ---------------------------------------------------------------------------

impl<K, V> FromIterator<(K, V)> for TrackingDict<K, V>
where
    K: Eq + Hash + Clone,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::from_map(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_tracks_key() {
        let mut td: TrackingDict<String, i32> = TrackingDict::new();
        td.insert("a".into(), 1);
        let keys = td.pop_accessed_keys();
        assert!(keys.contains("a"));
        // Second call should be empty.
        assert!(td.pop_accessed_keys().is_empty());
    }

    #[test]
    fn remove_tracks_key() {
        let mut td = TrackingDict::from_map(HashMap::from([("x".to_owned(), 42)]));
        td.remove(&"x".to_owned());
        let keys = td.pop_accessed_keys();
        assert!(keys.contains("x"));
    }

    #[test]
    fn update_no_track_is_silent() {
        let mut td: TrackingDict<String, i32> = TrackingDict::new();
        td.update_no_track(HashMap::from([("b".into(), 2)]));
        assert!(td.pop_accessed_keys().is_empty());
        assert_eq!(td.get(&"b".into()), Some(&2));
    }

    #[test]
    fn clear_marks_all_dirty() {
        let mut td =
            TrackingDict::from_map(HashMap::from([("a".to_owned(), 1), ("b".to_owned(), 2)]));
        td.clear();
        let keys = td.pop_accessed_keys();
        assert!(keys.contains("a"));
        assert!(keys.contains("b"));
        assert!(td.is_empty());
    }

    #[test]
    fn pop_accessed_write_items_returns_deleted() {
        let mut td = TrackingDict::from_map(HashMap::from([("k".to_owned(), 10)]));
        td.remove(&"k".to_owned());
        let items = td.pop_accessed_write_items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].1, EntryValue::Deleted);
    }

    #[test]
    fn set_default_tracks_on_miss() {
        let mut td: TrackingDict<String, i32> = TrackingDict::new();
        td.set_default("new".to_owned(), 5);
        let keys = td.pop_accessed_keys();
        assert!(keys.contains("new"));
    }
}
