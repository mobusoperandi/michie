use michie::memoized;
use std::collections::HashMap;
use std::rc::Rc;

fn generic_in_impl() {
    #[memoized(key_expr = &input, store_type = HashMap<Rc<()>, ()>)]
    fn f(input: Rc<()>) -> () {
        ()
    }
}

fn main() {}
