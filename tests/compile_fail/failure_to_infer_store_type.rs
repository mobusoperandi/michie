use michie::memoized;

#[memoized(key_type = (), key_expr = (), store_init = Default::default())]
fn f() -> () {}

fn main() {}
