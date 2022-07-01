#![doc = include_str!("../README.md")]

use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

/// See [crate level documentation](crate).
pub use michie_macro::memoized;

pub trait MemoizationStore<I, R> {
    fn insert(&mut self, input: &I, return_value: R) -> R;
    fn get(&self, input: &I) -> Option<R>;
}

impl<K, I, R> MemoizationStore<I, R> for HashMap<K, R>
where
    K: Eq + Hash + Borrow<I>,
    I: Eq + Hash + ToOwned<Owned = K>,
    R: Clone,
{
    fn insert(&mut self, input: &I, return_value: R) -> R {
        HashMap::insert(self, input.to_owned(), return_value.clone());
        return_value
    }
    fn get(&self, input: &I) -> Option<R> {
        HashMap::get(self, input).cloned()
    }
}

impl<K, I, R> MemoizationStore<I, R> for BTreeMap<K, R>
where
    K: Ord + Borrow<I>,
    I: Ord + ToOwned<Owned = K>,
    R: Clone,
{
    fn insert(&mut self, input: &I, return_value: R) -> R {
        BTreeMap::insert(self, input.to_owned(), return_value.clone());
        return_value
    }
    fn get(&self, input: &I) -> Option<R> {
        BTreeMap::get(self, input).cloned()
    }
}
