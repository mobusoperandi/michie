[![CI status](https://github.com/mobusoperandi/michie/actions/workflows/ci.yml/badge.svg)](https://github.com/mobusoperandi/michie/actions/workflows/ci.yml)

# What

michie (pronounced /'mikɪ/) is an attribute macro that adds [memoization] to a function.

# A basic example

```rust
use michie::memoized;
#[memoized(key_expr = input)]
fn f(input: usize) -> usize {
    // expensive calculation
    # input 
}
# assert_eq!(f(5), 5);
```

A call to `f` with an `input` value that it had already been called with is a cache hit.
A cache hit means that the implementation of `f` is not executed.
Instead, the return value is obtained from cache.

# Features

- Supports
    - Plain functions
    - Generic functions
    - Functions in `impl` blocks
    - Functions in trait implementation blocks
    - Functions that are default trait implementations
- Thread safe
- Expansion depends on only `std`
- Hygienic
- Supports recursion
- Bring your own cache store (defaults to a [`HashMap`] in which values live forever)

# The cache key

The cache is a key-value map.
An expression for obtaining a key value (`key_expr`) must be provided.
The `key_type` may be specified.

## `key_expr`

One may ask why the key is not simply all of the inputs combined.
That is because some functions use only some of their input:

```rust
# use michie::memoized;
struct Foo {
    a: usize,
    b: usize,
}
#[memoized(key_expr = foo.a)]
fn f(foo: &Foo) -> usize {
    // only `foo.a` is used
    foo.a * 2 
}
let a = Foo { a: 1, b: 1 };
let b = Foo { a: 1, b: 2 }; // same `a`, different `b`
assert_eq!(f(&a), 2); // cache miss
assert_eq!(f(&b), 2); // cache hit because `a` is the same
```

This also demonstrates that the cache is shared across all instances of a type.

The `key_expr` argument does not have a default so that one could not forget to think about it.

The `key_expr` argument expands in a scope where bindings from the function's parameters are available.
Here's an example where the function has a pattern parameter:

```rust
# use michie::memoized;
#[memoized(key_expr = (a_0, b))]
fn f((a_0, _a_1): (usize, usize), b: usize) -> usize {
    a_0 * b
}
# assert_eq!(f((2, 3), 4), 8);
```

## `key_type`

While the type of the key supports inference, it may also be specified using the `key_type` argument:

```rust
# use michie::memoized;
#[memoized(key_type = u64, key_expr = input.into())]
fn f(input: u32) -> u32 {
    // expensive calculation
    # input
}
# assert_eq!(f(5), 5);
```

# `store_type` and `store_init`

The default store is implemented using a [`HashMap`] in which entries live forever.
It is provided under the assumption that it will suffice in a significant portion of cases.

In other cases the `store_type` and `store_init` arguments can be used.
The `store_type` expects a type that:

1. is generic on unbound `<K, R>`
2. provides the following functions (no trait is involved):
    - `fn insert(&mut self, key: K, value: R)` // return type ignored
    - `fn get(&self, key: &K) -> Option<&R>`

where `K` is the key type and `R` is the memoized function's return type.

By default, the `store_type` will be instantiated this way:

```text ignore
{
    use ::core::default::Default;
    StoreType::<K, R>::default()
}
```

For further customization `store_init` takes an expression.
Example:

```rust
# use michie::memoized;
# use std::marker::PhantomData;
struct Store<K, V> {
    // some fields
    # k: PhantomData<K>,
    # v: PhantomData<V>,
}
impl<K, V> Default for Store<K, V>
{
    fn default() -> Self {
        Self::new(0)
    }
}
impl<K, V> Store<K, V> {
    // the return type is irrelevant
    fn insert(&mut self, key: K, value: V) {
        // insert into cache
    }
    fn get(&self, key: &K) -> Option<&V> {
        // attempt to get from cache
        # None
    }
    fn new(size: usize) -> Self {
        // create a new cache store
        # Self {
        #     k: PhantomData,
        #     v: PhantomData,
        # }
    }
}
#[memoized(key_expr = input, store_type = Store)]
fn expensive(input: usize) -> usize {
    // expensive calculation
    # input
}
#[memoized(key_expr = input, store_type = Store, store_init = Store::new(500))]
fn expensive_and_large(input: usize) -> Vec<u8> {
    // expensive calculation with large return type
    # vec![]
}
# assert_eq!(expensive(2), 2);
# assert_eq!(expensive_and_large(2), vec![]);
```

By the way, [`BTreeMap`] happens to satisfy the above and therefore may be provided as `store_type`:

```rust
# use michie::memoized;
use std::collections::BTreeMap;
#[memoized(key_expr = input, store_type = BTreeMap)]
fn f(input: usize) -> usize {
    // expensive calculation
    # input
}
# assert_eq!(f(2), 2);
```

# Type requirements

Some bounds are imposed on the key type and the return type. 
Some of these bounds are from the general instrumentation and some are from the cache store.

## General bounds

On key type and return type:

- [`Sized`]
- [`Clone`]
- [`Send`]

## Store type requirements

Be mindful of the bounds imposed by any provided store type.
The bounds imposed by the default store type, [`HashMap`], are:

| key type | return type |
| --- | --- |
| [`'static`], [`Eq`], [`Hash`] | [`'static`] |

# Generic functions

Be mindful of the [type requirements](#type-requirements) when using on a generic function:

```rust
# use michie::memoized;
# use std::hash::Hash;
#[memoized(key_expr = input.clone())]
fn f<A, B>(input: A) -> B
where
    A: Clone + Send + 'static + Eq + Hash,
    B: Clone + Send + 'static + From<A>,
{
    input.into()
}
# assert_eq!(f::<u32, u64>(0), 0);
```

# Functions that take no input

Functions that take no input are good candidates for [compile-time evaluation],
which is usually preferred over runtime caching (such as this crate provides).
Nonetheless, some functions cannot be evaluated at compile time.
A reasonable `key_expr` for a function that takes no input is `()`:

```rust
# use michie::memoized;
#[memoized(key_expr = ())]
fn f() -> f64 {
    // expensive calculation
    # 1.0
}
# assert_eq!(f(), 1.0);
```

# Authored by Mobus Operandi

This crate is a work by [Mobus Operandi] — a community for the study of Rust in mob programming format.

[`Clone`]: https://doc.rust-lang.org/core/clone/trait.Clone.html
[`Send`]: https://doc.rust-lang.org/core/marker/trait.Send.html
[`'static`]: https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html
[`Eq`]: https://doc.rust-lang.org/core/cmp/trait.Eq.html
[`Hash`]: https://doc.rust-lang.org/core/hash/trait.Hash.html
[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[`Sized`]: https://doc.rust-lang.org/core/marker/trait.Sized.html
[`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
[compile-time evaluation]: https://doc.rust-lang.org/std/keyword.const.html#compile-time-evaluable-functions
[memoization]: https://en.wikipedia.org/wiki/Memoization
[Mobus Operandi]: https://github.com/mobusoperandi
