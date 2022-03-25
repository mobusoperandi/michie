#![doc = include_str!("../readme.md")]
use attribute_derive::Attribute as AttributeDerive;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse2, parse_quote, spanned::Spanned, Attribute, Block, Expr, ImplItemMethod, ItemFn,
    ReturnType, Type,
};
#[proc_macro_attribute]
pub fn caching(
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
    if let Ok(associated_fn) = parse2::<ImplItemMethod>(input.clone()) {
        Ok(expand_assoc_fn(associated_fn, attr_args))
    } else if let Ok(non_associated_fn) = parse2::<ItemFn>(input.clone()) {
        Ok(expand_non_assoc_fn(non_associated_fn, attr_args))
    } else {
        syn::Result::Err(syn::Error::new(input.span(), "must be used on a function"))
    }
}
#[derive(AttributeDerive)]
#[attribute(ident = "fake")]
struct AttrArgs {
    key_type: Type,
    key_expr: Expr,
}
fn obtain_attr_args(args: TokenStream) -> syn::Result<AttrArgs> {
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
    let AttrArgs { key_expr, key_type } = attr_args;
    let obtain_cache = quote! {
        let mut type_map_mutex_guard = CACHE
            .lock()
            .expect("handling of poisoning is not supported");
        let type_map = <::std::sync::MutexGuard<::anymap2::Map<dyn ::anymap2::any::Any + ::core::marker::Send>> as ::std::ops::DerefMut>::deref_mut(&mut type_map_mutex_guard);
        let cache = type_map
            .entry::<::std::collections::HashMap<#key_type, #return_type>>()
            .or_insert_with(|| ::std::collections::HashMap::new());
    };
    parse_quote! {{
        let key = #key_expr;
        static CACHE: ::once_cell::sync::Lazy<
            ::std::sync::Mutex<
                ::anymap2::Map<dyn ::anymap2::any::Any + ::core::marker::Send>
            >
        > = ::once_cell::sync::Lazy::new(
            || ::std::sync::Mutex::new(::anymap2::Map::new())
        );
        #obtain_cache
        let attempt = cache.get(&key).cloned();
        drop(type_map_mutex_guard);
        if let Some(hit) = attempt {
            hit
        } else {
            let miss = #original_fn_block;
            #obtain_cache
            cache.insert(key, miss.clone());
            miss
        }
    }}
}
fn expand_non_assoc_fn(original_fn: ItemFn, attr_args: AttrArgs) -> Box<dyn ToTokens> {
    let mut expanded_fn = original_fn.clone();
    let original_fn_block = *original_fn.block;
    let return_type = obtain_return_type(original_fn.sig.output);
    expanded_fn.block = Box::new(expand_fn_block(original_fn_block, return_type, attr_args));
    Box::new(expanded_fn)
}
fn expand_assoc_fn(
    original_fn: ImplItemMethod,
    attr_args: AttrArgs,
) -> Box<dyn ToTokens> {
    let mut expanded_fn = original_fn.clone();
    let original_fn_block = original_fn.block;
    let return_type = obtain_return_type(original_fn.sig.output);
    expanded_fn.block = expand_fn_block(original_fn_block, return_type, attr_args);
    Box::new(expanded_fn)
}
