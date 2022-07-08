use michie::memoized;
use std::rc::Rc;
use std::collections::HashMap;

fn generic_in_impl() {
    #[memoized(key_expr = input, store_type = HashMap<Rc<()>, ()>)]
    fn f(input: Rc<()>) -> () {
        ()
    }
}

fn main() {}
