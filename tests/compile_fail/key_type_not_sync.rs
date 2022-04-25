use michie::memoized;
use std::rc::Rc;

fn generic_in_impl() {
    #[memoized(key_expr = input)]
    fn f(input: Rc<()>) -> () {
        ()
    }
}

fn main() {}
