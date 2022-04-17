[![CI status](https://github.com/mobusoperandi/michie/actions/workflows/ci.yml/badge.svg)](https://github.com/mobusoperandi/michie/actions/workflows/ci.yml)

## What

Attribute macro that adds [memoization] to a function.

## A basic example

```rust
# use michie::memoized;
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

## Features

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
- Bring your own caching type (uses [`HashMap`] by default)

## The cache key

The cache is a key-value map.
An expression for obtaining a key value (`key_expr`) must be provided.

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

## Type requirements

### Key type

- [`Sized`]
- [`Clone`]
- [`Send`]

Additionally, if the default caching type, [`HashMap`], is used:

- [`'static`]
- [`Eq`]
- [`Hash`]

### Return type

- [`Sized`]
- [`Clone`]
- [`Send`]

Additionally, if the default caching type, [`HashMap`], is used:

- [`'static`]

## Generic functions

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

## `caching_type`

The `caching_type` argument can be used to provide a type that implements caching behavior.
It defaults to [`HashMap`].
It must provide some functions as in the following example:

```rust
# use michie::memoized;
# use std::marker::PhantomData;
struct CachingType<K, V> {
    // some fields
    # k: PhantomData<K>,
    # v: PhantomData<V>,
}
impl<K, V> CachingType<K, V> {
    // or via the `Default` trait
    fn default() -> Self {
        // produce default
        # Self {
        #     k: PhantomData,
        #     v: PhantomData,
        # }
    }
    // the return type is irrelevant
    fn insert(&mut self, key: K, value: V) {
        // insert into cache
    }
    fn get(&self, key: &K) -> Option<&V> {
        // attempt to get from cache
        # None
    }
}
#[memoized(key_expr = input, caching_type = CachingType)]
fn f(input: usize) -> usize {
    // expensive calculation
    # input
}
# assert_eq!(f(2), 2);
```

Be mindful of the type requirements imposed by your caching type.

By the way, [`BTreeMap`] happens to satisfy the above and therefore may be provided as `caching_type`:

```rust
# use michie::memoized;
use std::collections::BTreeMap;
#[memoized(key_expr = input, caching_type = BTreeMap)]
fn f(input: usize) -> usize {
    // expensive calculation
    # input
}
# assert_eq!(f(2), 2);
```

## Functions that take no input

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
