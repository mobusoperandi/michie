use itertools::Itertools;
use michie::{memoized, MemoizationStore};
use std::{fs::read_dir, hash::Hash, marker::PhantomData, path::Path};

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
    drop(t);

    // workaround for https://github.com/dtolnay/trybuild/issues/169
    if Path::new("wip").exists() {
        let files = read_dir("wip")
            .unwrap()
            .map_ok(|entry| entry.file_name())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(files, vec![".gitignore"])
    }
}

#[test]
fn store_type_provided_as_path() {
    #[memoized(key_expr = b, store_type = ::std::collections::HashMap)]
    fn f2(_a: bool, b: usize) -> usize {
        b + 4
    }
    assert_eq!(f2(false, 2), 6);
}

#[test]
fn store_init_is_omitted() {
    struct Store<K, V> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }
    impl<K, V> Default for Store<K, V> {
        fn default() -> Self {
            Self {
                k: PhantomData,
                v: PhantomData,
            }
        }
    }
    impl<K, V> MemoizationStore<K, V> for Store<K, V> {
        fn insert(&mut self, _key: K, _value: V) {}
        fn get(&self, _key: &K) -> Option<&V> {
            None
        }
    }
    impl<K, V> Store<K, V> {
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
    struct Store<K, V> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }
    impl<K, V> Store<K, V> {
        fn new() -> Self {
            Self {
                k: PhantomData,
                v: PhantomData,
            }
        }
    }
    impl<K, V> MemoizationStore<K, V> for Store<K, V> {
        fn insert(&mut self, _key: K, _value: V) {}
        fn get(&self, _key: &K) -> Option<&V> {
            None
        }
    }
    impl<K, V> Default for Store<K, V> {
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
    struct Store<K, V> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }
    impl<K, V> Store<K, V> {
        fn new() -> Self {
            Self {
                k: PhantomData,
                v: PhantomData,
            }
        }
    }
    impl<K, V> MemoizationStore<K, V> for Store<K, V> {
        fn insert(&mut self, _key: K, _value: V) {}
        fn get(&self, _key: &K) -> Option<&V> {
            None
        }
    }
    #[memoized(key_expr = input, store_type = Store, store_init = Store::<usize, usize>::new())]
    fn f(input: usize) -> usize {
        input
    }
    assert_eq!(f(2), 2);
}

#[test]
fn store_init_includes_function_from_impl_block_that_has_bound_on_k_and_v() {
    struct Store<K, V> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }
    impl<K, V> Store<K, V> {
        fn new() -> Self
        where
            K: Default,
            V: Default,
        {
            Self {
                k: PhantomData,
                v: PhantomData,
            }
        }
    }
    impl<K, V> MemoizationStore<K, V> for Store<K, V> {
        fn insert(&mut self, _key: K, _value: V) {}
        fn get(&self, _key: &K) -> Option<&V> {
            None
        }
    }
    #[memoized(key_expr = input, store_type = Store, store_init = Store::new())]
    fn f(input: usize) -> usize {
        input
    }
    assert_eq!(f(2), 2);
}
