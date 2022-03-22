use caching::caching;

#[caching(key_type = usize, key_expr = a)]
struct A;

fn main() {}
