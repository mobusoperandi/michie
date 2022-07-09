use michie::memoized;
use std::collections::HashMap;

#[memoized(key_expr = input, store_type = HashMap<&str, &str>)]
fn f(input: &str) -> &str {
    input
}

fn main() {}
