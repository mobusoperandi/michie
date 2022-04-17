use michie::memoized;

#[memoized(key_expr = a)]
fn f(a: bool) -> bool {
    key
}

fn main() {}
