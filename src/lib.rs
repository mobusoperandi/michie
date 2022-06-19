#![doc = include_str!("../README.md")]

use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

/// See [crate level documentation](crate).
pub use michie_macro::memoized;

pub trait MemoizationStore<I, R> {
    fn insert(&mut self, input: I, return_value: R) -> R;
    fn get(&self, input: &I) -> Option<R>;
}

impl<I, R> MemoizationStore<I, R> for HashMap<I, R>
where
    I: Eq + Hash,
    R: Clone,
{
    fn insert(&mut self, input: I, return_value: R) -> R {
        HashMap::insert(self, input, return_value.clone());
        return_value
    }
    fn get(&self, input: &I) -> Option<R> {
        HashMap::get(self, input).cloned()
    }
}

impl<I, R> MemoizationStore<I, R> for BTreeMap<I, R>
where
    I: Ord,
    R: Clone,
{
    fn insert(&mut self, input: I, return_value: R) -> R {
        BTreeMap::insert(self, input, return_value.clone());
        return_value
    }
    fn get(&self, input: &I) -> Option<R> {
        BTreeMap::get(self, input).cloned()
    }
}

#[derive(Default)]
pub struct TryMemoizationStore<S>(pub S);

impl<K, V, E, S> MemoizationStore<K, Result<V, E>> for TryMemoizationStore<S>
where
    S: MemoizationStore<K, V>,
{
    fn insert(&mut self, key: K, value: Result<V, E>) -> Result<V, E> {
        if let Ok(value) = value {
            Ok(self.0.insert(key, value))
        } else {
            value
        }
    }

    fn get(&self, key: &K) -> Option<Result<V, E>> {
        self.0.get(key).map(Ok)
    }
}
