use michie::memoized;
use std::rc::Rc;

fn generic_in_impl() {
    #[memoized(key_expr = ())]
    fn f() -> Rc<()> {
        Rc::new(())
    }
}

fn main() {}
