use michie::memoized;
use std::collections::HashMap;

#[memoized(key_expr = b, store_type = HashMap<bool, bool>)]
fn f(a: bool) -> bool {
    a
}

fn main() {}
