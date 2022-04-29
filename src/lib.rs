#![doc = include_str!("../README.md")]

use ::std::collections::HashMap;
use std::{collections::BTreeMap, hash::Hash};

/// See [crate level documentation](crate).
pub use michie_macro::memoized;

pub trait MemoizationStore<K, V> {
    fn insert(&mut self, key: K, value: V);
    fn get(&self, key: &K) -> Option<&V>;
}

impl<K, V> MemoizationStore<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn insert(&mut self, key: K, value: V) {
        HashMap::insert(self, key, value);
    }
    fn get(&self, key: &K) -> Option<&V> {
        HashMap::get(self, key)
    }
}

impl<K, V> MemoizationStore<K, V> for BTreeMap<K, V>
where
    K: Ord,
{
    fn insert(&mut self, key: K, value: V) {
        BTreeMap::insert(self, key, value);
    }
    fn get(&self, key: &K) -> Option<&V> {
        BTreeMap::get(self, key)
    }
}
