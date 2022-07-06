use michie::memoized;
use std::collections::HashMap;

#[memoized(key_expr = a, store_type = HashMap<usize, bool>)]
fn f(a: bool) -> bool {
    a
}

fn main() {}
