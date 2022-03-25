Every subsequent call with a particular input will be a cache hit.
Due to recursion the function will be called with the same input multiple times.

```
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

The type of the cache key and the expression for obtaining it must be specified because some functions use only some of their input.
This is especially common in methods, where `self` could have fields that are irrelevant for a particular calculation:

```
# use caching::caching;
struct Foo {
    a: usize,
    b: usize,
}
impl Foo {
    #[caching(key_type = usize, key_expr = self.a)]
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
Deriving of the `key_type` did not seem reasonable to implement.

The `key_expr` argument expands in a scope where bindings from the function's parameters are available.
Here's an example where the function has a pattern parameter:

```
# use caching::caching;
#[caching(key_type = (usize, usize), key_expr = (a_0, b))]
fn some_product((a_0, _a_1): (usize, usize), b: usize) -> usize {
    a_0 * b
}
# assert_eq!(some_product((2, 3), 4), 8);
```

Key and return types must be entirely owned:

```
# use caching::caching;
#[caching(key_type = String, key_expr = String::from(str))]
fn dash_dash_split<'a>(str: &'a str) -> Option<(String, String)> {
    str.split_once("--").map(|(a, b)| (a.into(), b.into()))
}
# assert_eq!(dash_dash_split("a--b"), Some(("a".into(), "b".into())));
```

Generic functions are supported:

```
# use caching::caching;
#[caching(key_expr = a.clone(), key_type = T)]
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
