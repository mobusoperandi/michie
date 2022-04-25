use michie::memoized;
use std::{hash::Hash, marker::PhantomData};

#[test]
fn fn0() {
    #[memoized(key_expr = b)]
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
        T: Clone + Send + Eq + PartialEq + Hash + 'static + Sync,
    {
        #[memoized(key_expr = (self.a.clone(), b.clone()))]
        fn f<U>(&self, b: U) -> (T, U)
        where
            U: Clone + Send + Eq + PartialEq + Hash + 'static + Sync,
        {
            (self.a.clone(), b)
        }
    }
    let concrete_struct = GenericStruct { a: false };
    assert_eq!(concrete_struct.f(4), (false, 4));
    assert_eq!(concrete_struct.f("foo"), (false, "foo"));
}

#[test]
fn trait_implementation_fn() {
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
fn store_as_path() {
    #[memoized(key_expr = b, store_type = ::std::collections::HashMap)]
    fn f2(_a: bool, b: usize) -> usize {
        b + 4
    }
    assert_eq!(f2(false, 2), 6);
}

#[test]
fn store_type_default_from_impl() {
    struct Store<K, V> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }
    impl<K, V> Store<K, V> {
        fn default() -> Self {
            Self {
                k: PhantomData,
                v: PhantomData,
            }
        }
        fn insert(&mut self, _key: K, _value: V) {}
        fn get(&self, _key: &K) -> Option<&V> {
            None
        }
    }
    #[memoized(key_expr = input, store_type = Store)]
    fn f(input: usize) -> usize {
        input
    }
    assert_eq!(f(2), 2);
}

#[test]
fn store_type_default_from_trait() {
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
    impl<K, V> Store<K, V> {
        fn insert(&mut self, _key: K, _value: V) {}
        fn get(&self, _key: &K) -> Option<&V> {
            None
        }
    }
    #[memoized(key_expr = input, store_type = Store)]
    fn f(input: usize) -> usize {
        input
    }
    assert_eq!(f(2), 2);
}

#[test]
fn store_init() {
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
        fn insert(&mut self, _key: K, _value: V) {}
        fn get(&self, _key: &K) -> Option<&V> {
            None
        }
    }
    impl<K, V> Default for Store<K, V> {
        fn default() -> Self {
            panic!("`store_init` is expected to be used")
        }
    }
    #[memoized(key_expr = input, store_type = Store, store_init = Store::new())]
    fn f(input: usize) -> usize {
        input
    }
    assert_eq!(f(2), 2);
}

#[test]
fn store_init_concrete() {
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
fn store_init_bound() {
    struct Store<K, V> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }
    impl<K, V> Store<K, V> {
        fn new() -> Self
        where
            K: Default,
        {
            Self {
                k: PhantomData,
                v: PhantomData,
            }
        }
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
