use caching::has_caching_fn;

#[derive(PartialEq, Eq, Hash)]
struct Foo(bool, usize);

#[has_caching_fn]
impl Foo {
    #[caching]
    fn method_arity_1(#[caching(key = self.0)] &self) -> bool {
        self.0
    }

    #[caching]
    fn method_arity_2(&self, input: bool) -> (bool, bool) {
        (self.0, input)
    }

    #[caching]
    fn associated_fn_arity_1(input: bool) -> bool {
        input
    }

    #[caching]
    fn associated_fn_arity_2(#[caching(skip)] _input1: bool, input2: bool) -> bool {
        input2
    }

    #[caching]
    fn associated_fn_pattern((input1, input2): (bool, bool)) -> bool {
        input1 && input2
    }
}

#[test]
fn test() {
    let f = Foo(false, 1);

    assert!(!f.method_arity_1());
    assert_eq!(f.method_arity_2(false), (false, false));
    assert!(Foo::associated_fn_arity_1(true));
    assert!(Foo::associated_fn_arity_2(true, true));
    assert!(Foo::associated_fn_pattern((true, true)));
}
