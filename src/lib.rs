#![doc = include_str!("../README.md")]

use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

/// See [crate level documentation](crate).
pub use michie_macro::memoized;

pub trait MemoizationStore<K, R> {
    fn insert(&mut self, key: K, value: R) -> R;
    fn get(&self, key: &K) -> Option<R>;
}

impl<K, R> MemoizationStore<K, R> for HashMap<K, R>
where
    K: Eq + Hash,
    R: Clone,
{
    fn insert(&mut self, key: K, value: R) -> R {
        HashMap::insert(self, key, value.clone());
        value
    }
    fn get(&self, key: &K) -> Option<R> {
        HashMap::get(self, key).cloned()
    }
}

impl<K, R> MemoizationStore<K, R> for BTreeMap<K, R>
where
    K: Ord,
    R: Clone,
{
    fn insert(&mut self, key: K, value: R) -> R {
        BTreeMap::insert(self, key, value.clone());
        value
    }
    fn get(&self, key: &K) -> Option<R> {
        BTreeMap::get(self, key).cloned()
    }
}

pub struct TryMemoizationStore<S>(S);
// where
//     S: MemoizationStore<K, Result<V, E>>;

impl<K, V, E, S> MemoizationStore<K, Result<V, E>> for TryMemoizationStore<S>
where
    S: MemoizationStore<K, V>,
{
    fn insert(&mut self, key: K, value: Result<V, E>) -> Result<V, E> {
        todo!()
    }

    fn get(&self, key: &K) -> Option<Result<V, E>> {
        todo!()
    }
}
