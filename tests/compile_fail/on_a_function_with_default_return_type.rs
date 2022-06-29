use michie::memoized;

#[memoized(key_type = (), key_expr = ())]
fn f() {}

fn main() {}
