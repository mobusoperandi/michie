use caching::caching;

#[caching(key_expr = b)]
fn f(a: bool) -> bool {
    a
}

fn main() {}
