use michie::memoized;

#[memoized(key_expr = a)]
struct A;

fn main() {}
