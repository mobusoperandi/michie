use michie::memoized;

#[memoized(key_type = (), key_expr = &())]
struct A;

fn main() {}
