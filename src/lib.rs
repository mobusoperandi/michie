#![doc = include_str!("../README.md")]

use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

/// See [crate level documentation](crate).
pub use michie_macro::memoized;

pub trait MemoizationStore<K, R> {
    fn insert(&mut self, key: K, value: R);
    fn get(&self, key: &K) -> Option<&R>;
}

impl<K, R> MemoizationStore<K, R> for HashMap<K, R>
where
    K: Eq + Hash,
{
    fn insert(&mut self, key: K, value: R) {
        HashMap::insert(self, key, value);
    }
    fn get(&self, key: &K) -> Option<&R> {
        HashMap::get(self, key)
    }
}

impl<K, R> MemoizationStore<K, R> for BTreeMap<K, R>
where
    K: Ord,
{
    fn insert(&mut self, key: K, value: R) {
        BTreeMap::insert(self, key, value);
    }
    fn get(&self, key: &K) -> Option<&R> {
        BTreeMap::get(self, key)
    }
}
