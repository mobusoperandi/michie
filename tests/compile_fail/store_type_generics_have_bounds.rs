// This test does not test a feature. Quite the opposite.
// It proves an undesired limitation.
use core::marker::PhantomData;
use michie::memoized;

#[derive(Default)]
struct Store<K: Eq, V: Ord> {
    k: PhantomData<K>,
    v: PhantomData<V>,
}
impl<K: Eq, V: Ord> Store<K, V> {
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
