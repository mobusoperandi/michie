//! See the parent crate: <https://docs.rs/michie>
use attribute_derive::Attribute as AttributeDerive;
use proc_macro2::{Span, TokenStream};
use quote::{quote_spanned, ToTokens};
use syn::{
    parse2, parse_quote, parse_quote_spanned, spanned::Spanned, Attribute, Block, Expr, Ident,
    ImplItemMethod, ItemFn, ReturnType, Signature, Type,
};

#[proc_macro_attribute]
pub fn memoized(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    expand(args.into(), input.into())
        .map(|expansion| expansion.into_token_stream())
        .unwrap_or_else(|e| e.into_compile_error().into_token_stream())
        .into()
}
fn expand(args: TokenStream, input: TokenStream) -> syn::Result<Box<dyn ToTokens>> {
    let attr_args = obtain_attr_args(args)?;
    if let Ok(method) = parse2::<ImplItemMethod>(input.clone()) {
        expand_method(method, attr_args)
    } else if let Ok(function) = parse2::<ItemFn>(input.clone()) {
        expand_function(function, attr_args)
    } else {
        syn::Result::Err(syn::Error::new(input.span(), "must be used on a function"))
    }
}
#[derive(AttributeDerive)]
#[attribute(ident = "fake")]
struct AttrArgs {
    key_type: Option<Type>,
    key_expr: Expr,
    store_type: Option<Type>,
    store_init: Option<Expr>,
}
fn obtain_attr_args(args: TokenStream) -> syn::Result<AttrArgs> {
    // https://github.com/ModProg/attribute-derive/issues/1
    let fake_attr: Attribute = parse_quote! {#[fake( #args )]};
    AttrArgs::from_attributes([fake_attr])
}
fn obtain_return_type(return_type: ReturnType) -> Type {
    match return_type {
        syn::ReturnType::Type(_, return_type) => *return_type,
        syn::ReturnType::Default => unimplemented!("default return types are not supported"),
    }
}
fn expand_fn_block(original_fn_block: Block, return_type: Type, attr_args: AttrArgs) -> Block {
    let AttrArgs {
        key_expr,
        key_type,
        store_type,
        store_init,
    } = attr_args;
    let key = Ident::new("key", Span::mixed_site().located_at(key_expr.span()));
    let key_ref: Expr =
        parse_quote_spanned!(Span::mixed_site().located_at(key_expr.span())=> &#key);
    let key_type = key_type.unwrap_or_else(|| parse_quote! { _ });
    let store_type = store_type.unwrap_or_else(|| parse_quote!(::std::collections::HashMap));
    let store_init =
        store_init.unwrap_or_else(|| parse_quote!(::core::default::Default::default()));
    let type_map_type = quote_spanned! {Span::mixed_site()=>
        // Generic functions and default trait implementations are supported.
        // In each memoized function the cache is stored in a static.
        // As of the writing of this comment statics cannot have generic types:
        // https://doc.rust-lang.org/reference/items/static-items.html#statics--generics
        //
        // Caches of multiple types are stored in the static and resolved at runtime.
        // This is inspired by the anymap2 crate.
        ::std::collections::HashMap::<
            ::core::any::TypeId,
            // The following `Send + Sync` bounds apply to the store type and by extension also to the
            // key type and the return type.
            // It seems that in the current implementation this `Sync` bound is entirely
            // redundant because all operations on the cache store are within a `MutexGuard`.
            // Nonetheless, if Rust ever supports generic types in statics, this type map workaround could
            // be removed and the use of `Mutex` replaced with the use of a `RwLock`.
            // In that case, multiple references of the key type and the return type could be read
            // simultaneously across threads, making `Sync` necessary.
            ::std::boxed::Box<dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync>
        >
    };
    parse_quote_spanned! { Span::mixed_site()=> {
        // A more convenient type for the `CACHE` would have been:
        // ```
        // static CACHE: MaybeUninit<RwLock<#store_type<#key_type, #return_type>>> = MaybeUninit::uninit();
        // ```
        // This crate supports generic functions. `#key_type` and `#return_type` can include
        // generic types. As of the writing of this comment generics in statics are not supported:
        // https://doc.rust-lang.org/reference/items/static-items.html#statics--generics
        // Thus a type map is used, as seen in the type of `CACHE` below.
        static mut CACHE: ::core::mem::MaybeUninit<::std::sync::Mutex<#type_map_type>> = ::core::mem::MaybeUninit::uninit();
        static CACHE_INIT: ::std::sync::Once = ::std::sync::Once::new();
        CACHE_INIT.call_once(|| {
            let cache: ::std::sync::Mutex<#type_map_type> = ::core::default::Default::default();
            unsafe {
                // safe because synchronized by `Once::call_once`
                CACHE.write(cache);
            }
        });
        let type_map_mutex: &::std::sync::Mutex<#type_map_type> = unsafe {
            // This code is in an unsafe block for 2 reasons:
            // 1. reading a `static mut`
            // 2. `MaybeUninit::assume_init_ref`
            //
            // Safe because:
            // 1. This is a read and the one and only write had already occurred using
            //    `Once::call_once`.
            // 2. Was certainly initialized in the same `Once::call_once`.
            CACHE.assume_init_ref()
        };
        let #key: #key_type = #key_expr;
        let mut type_map_mutex_guard: ::std::sync::MutexGuard<#type_map_type> = type_map_mutex
            .lock()
            .expect("handling of poisoning is not supported");
        let cache: &#store_type<#key_type, #return_type> = {
            // This function and the similar function, `obtain_mutable_cache` exist for the sole
            // purpose of allowing `key_type` to be optional. To do that, the key type must be
            // successfully inferred in several positions. The only position in which inference
            // is not possible is the type argument of `TypeId::of`. That's because the return type
            // of `TypeId::of` is concrete and not generic, effectively severing the type inference
            // chain. Ideally, a simpler language feature such as a `typeof` operator would allow
            // us to specify that `K` in `TypeId::of::<(K, R)>` is the one inferred for
            // `#key`. In lieu of such a language feature we resort to using a generic type of a
            // function and place some code in it that would otherwise be inline.
            fn obtain_immutable_cache<'a, K, R, I>(
                _key: &K,
                type_map_mutex_guard: &'a mut ::std::sync::MutexGuard<#type_map_type>,
                store_init: I,
            ) -> &'a #store_type<K, R>
            where
                K: 'static + ::core::marker::Send + ::core::marker::Sync,
                R: 'static + ::core::marker::Send + ::core::marker::Sync,
                I: ::core::ops::FnOnce() -> #store_type<K, R>
            {
                let cache: &::std::boxed::Box<dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync> = type_map_mutex_guard
                    .entry(::core::any::TypeId::of::<(K, R)>())
                    .or_insert_with(|| {
                        let store: #store_type<K, R> = store_init();
                        ::std::boxed::Box::new(store)
                    });
                let cache: &(dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync) = cache.as_ref();
                // type is known to be `#store_type<K, R>` because value is obtained via the above
                // `HashMap::entry` call with `TypeId::of::<(K, R)>`
                cache.downcast_ref::<#store_type<K, R>>().unwrap()
            }
            obtain_immutable_cache::<#key_type, #return_type, fn() -> #store_type<#key_type, #return_type>>(
                #key_ref,
                &mut type_map_mutex_guard,
                || #store_init
            )
        };
        // At this point, while an exclusive lock is still in place, a read lock would suffice.
        // However, since the concrete cache store is already obtained and since presumably the
        // following `::get` should be cheap, releasing the exclusive lock, obtaining a read lock
        // and obtaining the cache store again does not seem reasonable.
        let attempt: ::core::option::Option<#return_type> = cache.get(#key_ref).cloned();
        ::core::mem::drop(type_map_mutex_guard);
        if let ::core::option::Option::Some(hit) = attempt {
            hit
        } else {
            let miss: #return_type = #original_fn_block;
            let mut type_map_mutex_guard: ::std::sync::MutexGuard<#type_map_type> = type_map_mutex
                .lock()
                .expect("handling of poisoning is not supported");
            // see comment for `obtain_immutable_cache` above
            let cache: &mut #store_type<#key_type, #return_type> = {
                fn obtain_mutable_cache<'a, K, R>(
                    type_map_mutex_guard: &'a mut ::std::sync::MutexGuard<#type_map_type>,
                ) -> &'a mut #store_type<K, R>
                where
                    K: 'static + ::core::marker::Send + ::core::marker::Sync,
                    R: 'static + ::core::marker::Send + ::core::marker::Sync,
                {
                    let cache: &mut ::std::boxed::Box<dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync> = type_map_mutex_guard
                        .get_mut(&::core::any::TypeId::of::<(K, R)>())
                        .unwrap();
                    let cache: &mut (dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync) = cache.as_mut();
                    // type is known to be `#store_type<K, R>` because value is obtained via the above
                    // `HashMap::get_mut` call with `TypeId::of::<(K, R)>`
                    cache.downcast_mut::<#store_type<K, R>>().unwrap()
                }
                obtain_mutable_cache::<#key_type, #return_type>(&mut type_map_mutex_guard)
            };
            cache.insert(#key, ::core::clone::Clone::clone(&miss));
            miss
        }
    }}
}
fn signature_constness_none(sig: &Signature) -> syn::Result<()> {
    match sig.constness {
        Some(constness) => Err(syn::Error::new(
            Span::mixed_site().located_at(constness.span()),
            "const functions not allowed",
        )),
        None => Ok(()),
    }
}
fn expand_function(original_fn: ItemFn, attr_args: AttrArgs) -> syn::Result<Box<dyn ToTokens>> {
    signature_constness_none(&original_fn.sig)?;
    let mut expanded_fn = original_fn.clone();
    let original_fn_block = *original_fn.block;
    let return_type = obtain_return_type(original_fn.sig.output);
    expanded_fn.block = Box::new(expand_fn_block(original_fn_block, return_type, attr_args));
    Ok(Box::new(expanded_fn))
}
fn expand_method(
    original_fn: ImplItemMethod,
    attr_args: AttrArgs,
) -> syn::Result<Box<dyn ToTokens>> {
    signature_constness_none(&original_fn.sig)?;
    let mut expanded_fn = original_fn.clone();
    let original_fn_block = original_fn.block;
    let return_type = obtain_return_type(original_fn.sig.output);
    expanded_fn.block = expand_fn_block(original_fn_block, return_type, attr_args);
    Ok(Box::new(expanded_fn))
}
