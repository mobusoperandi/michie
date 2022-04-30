use michie::{memoized, MemoizationStore};

struct Store;
impl MemoizationStore<usize, usize> for Store {
    fn insert(&mut self, _key: usize, _value: usize) {}
    fn get(&self, _key: &usize) -> Option<&usize> {
        None
    }
}
#[memoized(key_expr = input, store_type = Store)]
fn f(input: usize) -> usize {
    input
}

fn main() {}
