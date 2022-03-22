use caching::caching;

#[test]
fn fn0() {
    #[caching(key_type = usize, key_expr = b)]
    fn f(_a: bool, b: usize) -> usize {
        b + 4
    }
    assert_eq!(f(false, 2), 6);
}

#[test]
fn fails_when_not_on_a_function() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/not_on_a_function.rs");
}
