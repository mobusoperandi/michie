use caching::caching;

#[caching(key_type = bool, key_expr = a)]
fn f(a: bool) -> bool {
    key
}

fn main() {}
