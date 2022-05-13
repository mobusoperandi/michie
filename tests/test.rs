use michie::{memoized, MemoizationStore};
use std::{collections::BTreeMap, hash::Hash, marker::PhantomData};

#[test]
fn sanity() {
    #[memoized(key_expr = b)]
    fn f(_a: bool, b: usize) -> usize {
        b + 4
    }
    assert_eq!(f(false, 2), 6);
}

#[test]
fn on_a_generic_fn_in_an_impl_block() {
    struct GenericStruct<T> {
        a: T,
    }

    impl<T> GenericStruct<T>
    where
        T: 'static + Clone + Send + Sync + Eq + Hash,
    {
        #[memoized(key_expr = (self.a.clone(), b.clone()))]
        fn f<U>(&self, b: U) -> (T, U)
        where
            U: 'static + Clone + Send + Sync + Eq + Hash,
        {
            (self.a.clone(), b)
        }
    }
    let concrete_struct = GenericStruct { a: false };
    assert_eq!(concrete_struct.f(4), (false, 4));
    assert_eq!(concrete_struct.f("foo"), (false, "foo"));
}

#[test]
fn key_type_does_not_need_to_be_clone() {
    #[memoized(key_expr = input)]
    fn f<A, B>(input: A) -> B
    where
        A: 'static + Copy + Send + Sync + Eq + Hash,
        B: 'static + Clone + Send + Sync + From<A>,
    {
        input.into()
    }
}

#[test]
fn on_a_fn_in_a_trait_impl_block() {
    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    struct Struct;
    impl core::ops::Add for Struct {
        type Output = Self;
        #[memoized(key_expr = (self.clone(), rhs))]
        fn add(self, rhs: Self) -> Self::Output {
            self
        }
    }
    assert_eq!(Struct + Struct, Struct)
}

#[test]
fn errors() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}

#[test]
fn store_type_provided_as_path() {
    #[memoized(key_expr = b, store_type = ::std::collections::HashMap<usize, usize>)]
    fn f2(_a: bool, b: usize) -> usize {
        b + 4
    }
    assert_eq!(f2(false, 2), 6);
}

#[test]
fn store_init_is_omitted() {
    struct Store;
    impl Default for Store {
        fn default() -> Self {
            Self
        }
    }
    impl MemoizationStore<usize, usize> for Store {
        fn insert(&mut self, _key: usize, _value: usize) {}
        fn get(&self, _key: &usize) -> Option<&usize> {
            None
        }
    }
    impl Store {
        #[allow(dead_code)]
        fn default() -> Self {
            unreachable!()
        }
    }
    #[memoized(key_expr = input, store_type = Store)]
    fn f(input: usize) -> usize {
        input
    }
    assert_eq!(f(2), 2);
}

#[test]
fn store_init_is_used_instead_of_implementation_of_the_default_trait() {
    struct Store;
    impl Store {
        fn new() -> Self {
            Self
        }
    }
    impl MemoizationStore<usize, usize> for Store {
        fn insert(&mut self, _key: usize, _value: usize) {}
        fn get(&self, _key: &usize) -> Option<&usize> {
            None
        }
    }
    impl Default for Store {
        fn default() -> Self {
            unreachable!()
        }
    }
    #[memoized(key_expr = input, store_type = Store, store_init = Store::new())]
    fn f(input: usize) -> usize {
        input
    }
    assert_eq!(f(2), 2);
}

#[test]
fn store_init_includes_a_concrete_store_type() {
    struct Store<K, R> {
        k: PhantomData<K>,
        r: PhantomData<R>,
    }
    impl<K, R> Store<K, R> {
        fn new() -> Self {
            Self {
                k: PhantomData,
                r: PhantomData,
            }
        }
    }
    impl<K, R> MemoizationStore<K, R> for Store<K, R> {
        fn insert(&mut self, _key: K, _value: R) {}
        fn get(&self, _key: &K) -> Option<&R> {
            None
        }
    }
    #[memoized(key_expr = input, store_type = Store<usize, usize>, store_init = Store::<usize, usize>::new())]
    fn f(input: usize) -> usize {
        input
    }
    assert_eq!(f(2), 2);
}

#[test]
fn store_init_includes_function_from_impl_block_that_has_bound_on_k_and_v() {
    struct Store<T> {
        p: PhantomData<T>,
    }
    impl<T: Default> Store<T> {
        fn new() -> Self {
            Self { p: PhantomData }
        }
    }
    impl MemoizationStore<usize, usize> for Store<()> {
        fn insert(&mut self, _key: usize, _value: usize) {}
        fn get(&self, _key: &usize) -> Option<&usize> {
            None
        }
    }
    #[memoized(key_expr = input, store_type = Store<()>, store_init = Store::new())]
    fn f(input: usize) -> usize {
        input
    }
    assert_eq!(f(2), 2);
}

#[test]
fn trait_functions_are_called_explicitly() {
    #[derive(Default)]
    struct Store;
    impl Store {
        #[allow(dead_code)]
        fn get(&self, _key: &()) -> Option<&()> {
            unreachable!()
        }
        #[allow(dead_code)]
        fn insert(&mut self, _key: (), _value: ()) {
            unreachable!()
        }
    }
    impl MemoizationStore<(), ()> for Store {
        fn insert(&mut self, _key: (), _value: ()) {}
        fn get(&self, _key: &()) -> Option<&()> {
            None
        }
    }
    #[memoized(key_type = (), key_expr = (), store_type = Store)]
    fn f() -> () {}
    f();
}

#[test]
#[should_panic(expected = "store_init executed")]
fn store_init_is_used() {
    #[memoized(key_expr = (), store_init = {
        panic!("store_init executed");
        #[allow(unreachable_code)]
        BTreeMap::<(), ()>::new()
    })]
    fn f() -> () {}
    f();
}

#[test]
fn store_type_is_inferred() {
    #[memoized(key_expr = input, store_init = BTreeMap::<usize, usize>::new())]
    fn f(input: usize) -> usize {
        input
    }
}

#[test]
fn store_type_is_inferred_not_from_store_init_alone() {
    #[memoized(key_expr = input, store_init = BTreeMap::new())]
    fn f(input: usize) -> usize {
        input
    }
}
