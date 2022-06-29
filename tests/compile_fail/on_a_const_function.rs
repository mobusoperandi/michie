use michie::memoized;

#[memoized(key_type = (), key_expr = ())]
const fn f() -> () {
    ()
}

fn main() {}
