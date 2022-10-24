use michie::memoized;
use std::rc::Rc;
use std::collections::HashMap;

fn generic_in_impl() {
    #[memoized(key_expr = (), store_type = HashMap<(), Rc<()>>)]
    fn f() -> Rc<()> {
        Rc::new(())
    }
}

fn main() {}
