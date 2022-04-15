[![CI status](https://github.com/mobusoperandi/caching-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/mobusoperandi/caching-rs/actions/workflows/ci.yml)

Every subsequent call with a particular input will be a cache hit.
Due to recursion the function will be called with the same input multiple times.

```rust
# use caching::caching;
#[caching(key_expr = n)]
fn fibonacci(n: usize) -> usize {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
assert_eq!(fibonacci(5), 8);
```

The type of the cache key and the expression for obtaining it must be specified because some functions use only some of their input.
This is especially common in methods, where `self` could have fields that are irrelevant for a particular calculation:

```rust
# use caching::caching;
struct Foo {
    a: usize,
    b: usize,
}
impl Foo {
    #[caching(key_expr = self.a)]
    fn calc(&self) -> usize {
    	// only the `a` field of the input is used
        self.a * 2 
    }
}
let foo = Foo { a: 1, b: 1 };
assert_eq!(foo.calc() /* cache miss */, 2);
let foo = Foo { a: 1, b: 2 }; // `b` is different
assert_eq!(foo.calc() /* cache hit */, 2);
```

The `key_expr` argument does not have a default so that one could not forget to think about it.

The `key_expr` argument expands in a scope where bindings from the function's parameters are available.
Here's an example where the function has a pattern parameter:

```rust
# use caching::caching;
#[caching(key_expr = (a_0, b))]
fn some_product((a_0, _a_1): (usize, usize), b: usize) -> usize {
    a_0 * b
}
# assert_eq!(some_product((2, 3), 4), 8);
```

The type of the key may be specified using the `key_type` argument:

```rust
# use caching::caching;
#[caching(key_type = usize, key_expr = n)]
fn fibonacci(n: usize) -> usize {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
assert_eq!(fibonacci(5), 8);
```

Key and return types must be entirely owned:

```rust
# use caching::caching;
#[caching(key_expr = String::from(str))]
fn dash_dash_split<'a>(str: &'a str) -> Option<(String, String)> {
    str.split_once("--").map(|(a, b)| (a.into(), b.into()))
}
# assert_eq!(dash_dash_split("a--b"), Some(("a".into(), "b".into())));
```

Generic functions are supported:

```rust
# use caching::caching;
#[caching(key_expr = a.clone())]
fn f<T>(a: T, b: T) -> T
where
    T: Clone + Send + Eq + std::hash::Hash + 'static + std::ops::Add<Output = T>,
{
    a + b
}
# assert_eq!(f(1u64, 2u64), 3);
# assert_eq!(f(1u64, 2u64), 3);
# assert_eq!(f(10u8, 20u8), 30);
```

By default, the cache is stored in a `HashMap`.

The `caching_type` argument can be used to provide a caching type.
The provided type must provide some functions, as seen below:

```rust
# use caching::caching;
# use std::marker::PhantomData;
struct CachingType<K, V> {
    // some fields
    # k: PhantomData<K>,
    # v: PhantomData<V>,
}
impl<K, V> CachingType<K, V> {
    fn default() -> Self {
        // or via the `Default` trait
        # Self {
        #     k: PhantomData,
        #     v: PhantomData,
        # }
    }
    // the return type is irrelevant
    fn insert(&mut self, _key: K, _value: V) {
        // insert into cache...
    }
    fn get(&self, _key: &K) -> Option<&V> {
        // attempt to get from cache...
        # None
    }
}
#[caching(key_expr = input, caching_type = CachingType)]
fn f(input: usize) -> usize {
    input + 4
}
# assert_eq!(f(2), 6);
```

`BTreeMap` happens to provide these functions, and therefore may be provided as `caching_type`:

```rust
# use caching::caching;
use std::collections::BTreeMap;
#[caching(key_expr = b, caching_type = BTreeMap)]
fn f(_a: bool, b: usize) -> usize {
    b + 4
}
# assert_eq!(f(false, 2), 6);
```

Functions without input are good candidates for using [compile-time evaluation](https://doc.rust-lang.org/std/keyword.const.html#compile-time-evaluable-functions) instead of runtime caching. 
For cases where that is not possible, this crate does support such functions:

```rust
# use caching::caching;
#[caching(key_expr = ())]
fn f() -> f64 {
    // expensive calculation
    # 1.0
}
# assert_eq!(f(), 1.0);
```

Functions in trait implementations are supported:

```rust
# use caching::caching;
#[derive(Clone, Hash, PartialEq, Eq)]
struct Struct {
    // fields
}
impl std::ops::Add for Struct {
    type Output = Self;
    #[caching(key_expr = (self.clone(), rhs))]
    fn add(self, rhs: Self) -> Self::Output {
        // expensive calculation
        # self
    }
}
```
