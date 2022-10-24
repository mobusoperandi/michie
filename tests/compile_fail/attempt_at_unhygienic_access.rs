use michie::memoized;
use std::collections::HashMap;

#[memoized(key_expr = a, store_type = HashMap<bool, bool>)]
fn f(a: bool) -> bool {
    key
}

fn main() {}
