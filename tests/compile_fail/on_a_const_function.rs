use michie::memoized;

#[memoized]
const fn f() -> () {
    ()
}

fn main() {}
