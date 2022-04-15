use caching::caching;

#[caching(key_expr = a)]
struct A;

fn main() {}
