use michie::memoized;
use std::collections::HashMap;
use std::rc::Rc;

#[memoized(key_expr = (), store_type = HashMap<(), Rc<()>>)]
fn f() -> Rc<()> {
    Rc::new(())
}

fn main() {}
