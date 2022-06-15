use michie::memoized;

#[memoized(key_expr = (), dont_cache_errors)]
fn f() -> i32 { 4 }

fn main() {}
