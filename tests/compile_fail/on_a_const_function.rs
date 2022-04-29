use michie::memoized;

#[memoized(key_expr = ())]
const fn f() -> () {
    ()
}

fn main() {}
