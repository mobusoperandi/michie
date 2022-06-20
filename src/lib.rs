#![doc = include_str!("../README.md")]

use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

/// See [crate level documentation](crate).
pub use michie_macro::memoized;

pub trait MemoizationStore<I: ?Sized, R> {
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

// #[test]
// fn input_is_copy_and_key_expr_makes_a_copy() {
//     let mut store: HashMap<i32, ()> = HashMap::new();
//     let input: i32 = 0;
//     let key = input;
//     MemoizationStore::get(&store, key);
//     MemoizationStore::insert(&mut store, key, ());
// }

#[test]
fn input_is_copy_and_key_expr_makes_a_ref() {
    let mut store: HashMap<i32, ()> = HashMap::new();
    let input: i32 = 0;
    let key = input;
    MemoizationStore::get(&store, &key);
    MemoizationStore::insert(&mut store, &key, ());
}

// #[test]
// fn input_is_clone_and_key_expr_makes_a_clone() {
//     let mut store: HashMap<String, ()> = HashMap::new();
//     let input = String::new();
//     let key = input;
//     MemoizationStore::get(&store, key.clone());
//     MemoizationStore::insert(&mut store, key.clone(), ());
// }

#[test]
fn input_is_clone_and_key_expr_makes_a_ref() {
    let mut store: HashMap<String, ()> = HashMap::new();
    let input = String::new();
    let key = &input;
    MemoizationStore::get(&store, key);
    MemoizationStore::insert(&mut store, key, ());
}
