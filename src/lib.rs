#![doc = include_str!("../README.md")]

use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

/// See [crate level documentation](crate).
pub use michie_macro::memoized;

pub trait MemoizationStore<I, R> {
    fn insert(&mut self, input: &I, return_value: &R);
    fn get(&self, input: &I) -> Option<R>;
}

impl<I, K, R> MemoizationStore<I, R> for HashMap<K, R>
where
    I: Eq + Hash + ToOwned<Owned = K>,
    K: Borrow<I> + Eq + Hash,
    R: Clone,
{
    fn insert(&mut self, input: &I, return_value: &R) {
        HashMap::insert(self, input.to_owned(), return_value.clone());
    }
    fn get(&self, input: &I) -> Option<R> {
        HashMap::get(self, input).cloned()
    }
}

impl<I, K, R> MemoizationStore<I, R> for BTreeMap<K, R>
where
    I: Ord + ToOwned<Owned = K>,
    K: Borrow<I> + Ord,
    R: Clone,
{
    fn insert(&mut self, input: &I, return_value: &R) {
        BTreeMap::insert(self, input.to_owned(), return_value.clone());
    }
    fn get(&self, input: &I) -> Option<R> {
        BTreeMap::get(self, input).cloned()
    }
}
