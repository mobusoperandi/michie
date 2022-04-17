use michie::memoized;

#[memoized(key_expr = ())]
fn f() {}

fn main() {}
