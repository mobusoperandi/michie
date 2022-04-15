use caching::caching;

#[caching(key_type = (), key_expr = a)]
fn f(a: bool) -> bool {
    a
}

fn main() {}
