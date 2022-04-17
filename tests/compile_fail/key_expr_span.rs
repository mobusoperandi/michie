use michie::memoized;

#[memoized(key_expr = b)]
fn f(a: bool) -> bool {
    a
}

fn main() {}
