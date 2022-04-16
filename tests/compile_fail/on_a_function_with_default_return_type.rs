use caching::caching;

#[caching(key_expr = ())]
fn f() {}

fn main() {}
