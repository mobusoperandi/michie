use michie::memoized;

#[memoized(key_expr = (), dont_cache_errors)]
fn f<T>() -> (Result<i32,u32>) { Ok(4) }

fn main() {}
