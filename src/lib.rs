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

pub trait KeyBorrow<Q: Copy> {
    fn borrow(&self) -> Q;
}

pub trait ToKey: Copy {
    type Key: KeyBorrow<Self>;
    fn to_key(&self) -> Self::Key;
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

mod question {
    use std::{borrow::Borrow, fmt::Display};

    fn takes_borrow<T: Borrow<B>, B: Display>(b: B) {
        println!("{b}")
    }
}
