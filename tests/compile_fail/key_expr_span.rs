use michie::memoized;

#[memoized(key_type = bool, key_expr = &b)]
fn f(a: bool) -> bool {
    a
}

fn main() {}
