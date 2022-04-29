use core::marker::PhantomData;
use michie::{memoized, MemoizationStore};

struct Store<K, V> {
    k: PhantomData<K>,
    v: PhantomData<V>,
}
impl<K, V> MemoizationStore<K, V> for Store<K, V> {
    fn insert(&mut self, _key: K, _value: V) {}
    fn get(&self, _key: &K) -> Option<&V> {
        None
    }
}
#[memoized(key_expr = input, store_type = Store)]
fn f(input: usize) -> usize {
    input
}

fn main() {}
