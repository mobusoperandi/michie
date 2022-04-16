use caching::caching;
use std::hash::Hash;

#[test]
fn fn0() {
    #[caching(key_expr = b)]
    fn f(_a: bool, b: usize) -> usize {
        b + 4
    }
    assert_eq!(f(false, 2), 6);
}

#[test]
fn generic_in_impl() {
    struct GenericStruct<T> {
        a: T,
    }

    impl<T> GenericStruct<T>
    where
        T: Clone + Send + Eq + PartialEq + Hash + 'static,
    {
        #[caching(key_expr = (self.a.clone(), b.clone()))]
        fn f<U>(&self, b: U) -> (T, U)
        where
            U: Clone + Send + Eq + PartialEq + Hash + 'static,
        {
            (self.a.clone(), b)
        }
    }
    let concrete_struct = GenericStruct { a: false };
    assert_eq!(concrete_struct.f(4), (false, 4));
    assert_eq!(concrete_struct.f("asdf"), (false, "asdf"));
}

#[test]
fn trait_implementation_fn() {
    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    struct Struct;
    impl core::ops::Add for Struct {
        type Output = Self;
        #[caching(key_expr = (self.clone(), rhs))]
        fn add(self, rhs: Self) -> Self::Output {
            self
        }
    }
    assert_eq!(Struct + Struct, Struct)
}

#[test]
fn fails_when_not_on_a_function() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/not_on_a_function.rs");
}

#[test]
fn attempt_at_unhygienic_access_fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/attempt_at_unhygienic_access.rs");
}

#[test]
fn key_type_mismatch() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/key_type_mismatch.rs");
}

#[test]
fn key_expr_span() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/key_expr_span.rs");
}

#[test]
fn caching_type_as_path() {
    #[caching(key_expr = b, caching_type = ::std::collections::HashMap)]
    fn f2(_a: bool, b: usize) -> usize {
        b + 4
    }
    assert_eq!(f2(false, 2), 6);
}
