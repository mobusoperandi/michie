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
- Supports recursive functions
- Bring your own store

# A basic example

```rust
use michie::memoized;
#[memoized(key_expr = input)]
fn f(input: usize) -> usize {
    // expensive calculation
    # unimplemented!()
}
```

# `key_expr`

The `key_expr` argument is an arbitrary expression.
It may use bindings from the function's parameters.

# `key_type`

While the type of the key supports inference, it may also be specified using the `key_type` argument:

```rust
use michie::memoized;
#[memoized(key_type = u64, key_expr = input.into())]
fn f(input: u32) -> u32 {
    // expensive calculation
    # unimplemented!()
}
```

# `store_type`

The default store is [`HashMap`].
It is provided under the assumption that it will frequently suffice.

A store type may be provided via the `store_type` argument.
The provided type must implement [`MemoizationStore`].

```rust
use michie::memoized;
use std::collections::BTreeMap;
#[memoized(key_expr = input, store_type = BTreeMap<usize, usize>)]
fn f(input: usize) -> usize {
    // expensive calculation
    # unimplemented!()
}
```

# `store_init`

For store initialization `store_init` takes an expression that returns a store.
If omitted, [`Default::default()`](core::default::Default::default) is used.

```rust
use michie::{memoized, MemoizationStore};
use std::collections::HashMap;
#[memoized(key_expr = input, store_init = HashMap::with_capacity(500))]
fn f(input: usize) -> usize {
    // expensive calculation
    # unimplemented!()
}
```

# Type requirements

Minimal bounds are imposed on the key type and the return type.
Some of these bounds are from the general instrumentation and some from the store.

## General bounds

On key type and return type:

- [`Sized`]: for one, the instrumentation stores the key in a `let` binding.
- [`'static`]: the key and return values are inserted into the store, which lives across function invocations, therefore the store cannot borrow from these functions.
- [`Clone`]: the key and return values are cloned for insertion into the store.
- [`Send`] and [`Sync`]: for parallel access.

## Store bounds

Be mindful of the bounds imposed by the `store_type`'s implementation of [`MemoizationStore`].
The default store type, [`HashMap`], imposes [`Eq`] and [`Hash`] on the key.

# Generic functions

Be mindful of the [type requirements](#type-requirements) when using on a generic function:

```rust
use michie::memoized;
use std::hash::Hash;
#[memoized(key_expr = input.clone())]
fn f<A, B>(input: A) -> B
where
    A: Clone + Send + Sync + 'static + Eq + Hash,
    B: Clone + Send + Sync + 'static + From<A>,
{
    input.into()
}
```

# Functions that take no input

Functions that take no input are good candidates for [compile-time evaluation],
which is usually preferred over runtime caching (such as this crate provides).
Nonetheless, some functions cannot be evaluated at compile time.
A reasonable `key_expr` for a function that takes no input is `()`:

```rust
use michie::memoized;
#[memoized(key_expr = ())]
fn f() -> f64 {
    // expensive calculation
    # unimplemented!()
}
```

# How it works

The original function expands into something similar to this:

```rust ignore
fn f(input: Input) -> Output {
    static STORE = Mutex::new(#store_init);
    let key = #key_expr;
    if let Some(hit) = STORE.lock().unwrap().get(&key) {
        return hit;
    } else {
        let miss = #original_fn_body;
        STORE.lock().unwrap().insert(key, miss.clone());
        return miss;
    };
}
```

# Why must `key_expr` be provided?

The only conceivable default is the entire input.
In theory, that default could look like:

```text
(param_a, param_b, param_c)
```

This might not suffice because some parameters might not satisfy [the bounds of the key type](#type-requirements).
Even if they do, this still might not be accurate, because the resulting key might be a supervalue of _the input of the actual calculation_.
To explain what that means, here is an example:

```rust compile_fail
use michie::memoized;
#[memoized]
fn f(a: usize, _b: usize) -> usize {
    // only `a` is used
    # unimplemented!()
}
```

With the theoretical `(a, _b)` default `key_expr` there could be false misses:

```rust ignore
f(0, 0); // expected miss
f(0, 1); // avoidable miss!
```

Had an accurate `key_expr = a` been provided, the second execution would be a hit.

# Support and feedback

In [the GitHub Discussions].

# Authored by Mobus Operandi

This crate is a work by [Mobus Operandi] — a community for the study of Rust in mob programming format.

[`Clone`]: https://doc.rust-lang.org/core/clone/trait.Clone.html
[`Sync`]: https://doc.rust-lang.org/core/marker/trait.Sync.html
[`Send`]: https://doc.rust-lang.org/core/marker/trait.Send.html
[`'static`]: https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html#trait-bound
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
[the GitHub Discussions]: https://github.com/mobusoperandi/michie/discussions
