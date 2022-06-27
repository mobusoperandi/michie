use michie::{memoized, MemoizationStore};
use std::{borrow::Borrow, collections::BTreeMap, hash::Hash, marker::PhantomData};

#[test]
fn sanity() {
    #[memoized(key_type = usize, key_expr = &b)]
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
        #[memoized(key_type = (T, U), key_expr = &(self.a, b))]
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
    #[memoized(key_type = A, key_expr = &input)]
    fn f<A, B>(input: A) -> B
    where
        A: 'static + Copy + Send + Sync + Eq + Hash + Borrow<A>,
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
        #[memoized(key_type = (Struct, Struct), key_expr = &(self, rhs))]
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
    #[memoized(
        key_type = usize,
        key_expr = &b,
        store_type = ::std::collections::HashMap<usize, usize>,
    )]
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
        fn insert(&mut self, _input: &usize, return_value: usize) -> usize {
            return_value
        }
        fn get(&self, _input: &usize) -> Option<usize> {
            None
        }
    }
    impl Store {
        #[allow(dead_code)]
        fn default() -> Self {
            unreachable!()
        }
    }
    #[memoized(
        key_type = usize,
        key_expr = &input,
        store_type = Store,
    )]
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
        fn insert(&mut self, _input: &usize, return_value: usize) -> usize {
            return_value
        }
        fn get(&self, _input: &usize) -> Option<usize> {
            None
        }
    }
    impl Default for Store {
        fn default() -> Self {
            unreachable!()
        }
    }
    #[memoized(
        key_type = usize,
        key_expr = &input,
        store_type = Store,
        store_init = Store::new(),
    )]
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
    impl<I, R> MemoizationStore<I, R> for Store<I, R> {
        fn insert(&mut self, _input: &I, return_value: R) -> R {
            return_value
        }
        fn get(&self, _input: &I) -> Option<R> {
            None
        }
    }
    #[memoized(
        key_type = usize,
        key_expr = &input,
        store_type = Store<usize, usize>,
        store_init = Store::<usize, usize>::new(),
    )]
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
        fn insert(&mut self, _input: &usize, return_value: usize) -> usize {
            return_value
        }
        fn get(&self, _input: &usize) -> Option<usize> {
            None
        }
    }
    #[memoized(
        key_type = usize,
        key_expr = &input,
        store_type = Store<()>, store_init = Store::new(),
    )]
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
        fn insert(&mut self, _input: &(), _return_value: ()) {}
        fn get(&self, _input: &()) -> Option<()> {
            None
        }
    }
    #[memoized(
        key_type = (),
        key_expr = &(),
        store_type = Store,
    )]
    fn f() -> () {}
    f();
}
fn f() -> () {
    static mut STORES: ::core::mem::MaybeUninit<
        ::std::sync::Mutex<
            ::std::collections::HashMap<
                ::core::any::TypeId,
                ::std::boxed::Box<
                    (dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync),
                >,
            >,
        >,
    > = ::core::mem::MaybeUninit::uninit();
    static STORES_INIT: ::std::sync::Once = ::std::sync::Once::new();
    STORES_INIT.call_once(|| {
        let store: ::std::sync::Mutex<
            ::std::collections::HashMap<
                ::core::any::TypeId,
                ::std::boxed::Box<
                    (dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync),
                >,
            >,
        > = ::core::default::Default::default();
        unsafe {
            STORES.write(store);
        }
    });
    let type_map_mutex: &::std::sync::Mutex<
        ::std::collections::HashMap<
            ::core::any::TypeId,
            ::std::boxed::Box<(dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync)>,
        >,
    > = unsafe { STORES.assume_init_ref() };
    let key = &();
    let mut type_map_mutex_guard: ::std::sync::MutexGuard<
        ::std::collections::HashMap<
            ::core::any::TypeId,
            ::std::boxed::Box<(dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync)>,
        >,
    > = type_map_mutex
        .lock()
        .expect("handling of poisoning is not supported");
    let type_id: ::core::any::TypeId = ::core::any::TypeId::of::<((), ())>();
    let store: &::std::boxed::Box<
        (dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync),
    > = type_map_mutex_guard.entry(type_id).or_insert_with(|| {
        let store: _ = {
            panic!("store_init executed");
            #[allow(unreachable_code)]
            BTreeMap::<(), ()>::new()
        };
        ::std::boxed::Box::new(store)
    });
    let store: &(dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync) =
        store.as_ref();
    let store: &_ = store.downcast_ref::<_>().unwrap();
    let attempt: ::core::option::Option<()> = ::michie::MemoizationStore::get(store, key);
    ::core::mem::drop(type_map_mutex_guard);
    if let ::core::option::Option::Some(hit) = attempt {
        hit
    } else {
        let miss: () = {};
        let mut type_map_mutex_guard: ::std::sync::MutexGuard<
            ::std::collections::HashMap<
                ::core::any::TypeId,
                ::std::boxed::Box<
                    (dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync),
                >,
            >,
        > = type_map_mutex
            .lock()
            .expect("handling of poisoning is not supported");
        let store: &mut ::std::boxed::Box<
            (dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync),
        > = type_map_mutex_guard.get_mut(&type_id).unwrap();
        let store: &mut (dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync) =
            store.as_mut();
        let store: &mut _ = store.downcast_mut::<_>().unwrap();
        ::michie::MemoizationStore::insert(store, key, miss)
    }
}

#[test]
#[should_panic(expected = "store_init executed")]
fn store_init_is_used() {
    #[memoized(
        key_type = (),
        key_expr = &(),
        store_init = {
            panic!("store_init executed");
            #[allow(unreachable_code)]
            BTreeMap::<(), ()>::new()
        },
    )]
    fn f() -> () {}
    f();
}

#[test]
fn store_type_is_inferred() {
    #[memoized(
        key_type = usize,
        key_expr = &input,
        store_init = BTreeMap::<usize, usize>::new(),
    )]
    fn f(input: usize) -> usize {
        input
    }
}

#[test]
fn store_type_is_inferred_not_from_store_init_alone() {
    #[memoized(
        key_type = usize,
        key_expr = input,
        store_init = BTreeMap::new()
    )]
    fn f(input: usize) -> usize {
        input
    }
}
