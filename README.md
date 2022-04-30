[![Version](https://img.shields.io/crates/v/michie)][crates.io]
[![License](https://img.shields.io/crates/l/michie)][license]
![Downloads](https://img.shields.io/crates/d/michie)
![Recent downloads](https://img.shields.io/crates/dr/michie)
[![CI status](https://github.com/mobusoperandi/michie/actions/workflows/ci.yml/badge.svg)][ci]

michie (pronounced /'mikɪ/) — an attribute macro that adds [memoization] to a function.

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

# The cache key

The cache is a key-value map.
An expression for obtaining a key value (`key_expr`) must be provided.
The `key_type` may be specified.

## `key_expr`

The `key_expr` argument is an arbitrary expression.
It may use bindings from the function's parameters.

A `key_expr` must be provided because there is no reasonable default.
The only conceivable default is the entire input.
It might look like:

```text
(param_a, param_b, param_c)
```

This might not suffice because some parameters might not satisfy [the bounds of the key type](#type-requirements).
Even if they do, the resulting key might be a supervalue of the input of the actual calculation.
Here is an example:

```rust compile_fail
# use michie::memoized;
#[memoized]
fn f((a, _b): (usize, usize)) -> usize {
    // only `a` is used
    # a
}
```

With the theoretical `(a, _b)` default `key_expr` there could be false cache misses:

```rust ignore
f((0, 0)); // expected cache miss
f((0, 1)); // avoidable cache miss!
```

The second cache miss could have been a hit given an accurate `key_expr`: `a`.

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
The `store_type` must implement [`MemoizationStore`].
For store initialization `store_init` takes an expression that evaluates to a value of the `store_type`.
The default `store_init` is [`::core::default::Default::default()`](core::default::Default::default).
Example:

```rust
# use michie::{memoized, MemoizationStore};
# use std::marker::PhantomData;
struct Store<K, V> {
    // some fields
    # k: PhantomData<K>,
    # v: PhantomData<V>,
}
impl<K, V> Default for Store<K, V> {
    fn default() -> Self {
        Self::new(0)
    }
}
impl<K, V> MemoizationStore<K, V> for Store<K, V> {
    fn insert(&mut self, key: K, value: V) {
        // insert into cache
    }
    fn get(&self, key: &K) -> Option<&V> {
        // attempt to get from cache
        # None
    }
}
impl<K, V> Store<K, V> {
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

# Type requirements

Minimal bounds are imposed on the key type and the return type.
Some of these bounds are from the general instrumentation and some may be from the cache store.

## General bounds

On key type and return type:

- [`Sized`]: for one, the instrumentation stores the key in a `let` binding.
- [`'static`]: the cache store lives across function invocations — it cannot borrow from them.
- [`Clone`]: the key and return value are cloned for insertion into the store.
- [`Send`]: for parallelism
- [`Sync`]: for parallelism

## Store type requirements

Be mindful of the bounds imposed by the `store_type`'s implementation of [`MemoizationStore`].
For the default store type, [`HashMap`], they are for the key type: [`Eq`] and [`Hash`].

# Generic functions

Be mindful of the [type requirements](#type-requirements) when using on a generic function:

```rust
# use michie::memoized;
# use std::hash::Hash;
#[memoized(key_expr = input.clone())]
fn f<A, B>(input: A) -> B
where
    A: Clone + Send + Sync + 'static + Eq + Hash,
    B: Clone + Send + Sync + 'static + From<A>,
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
[`Sync`]: https://doc.rust-lang.org/core/marker/trait.Sync.html
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
[crates.io]: https://crates.io/crates/michie
[ci]: https://github.com/mobusoperandi/michie/actions/workflows/ci.yml
[license]: https://tldrlegal.com/license/mit-license
