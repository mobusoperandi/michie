use attribute_derive::Attribute;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse, parse_quote,
    punctuated::Punctuated,
    Expr,
    FnArg::{self, Receiver, Typed},
    Ident, ImplItem, ImplItemMethod, ItemImpl, Pat, PatType, Token,
};

const CRATE_NAME: &str = "caching";

#[proc_macro_attribute]
pub fn has_caching_fn(
    _attribute: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // TODO return an error or something
    let mut block: ItemImpl = parse(body).unwrap();

    let (caches, items) = match expand_impl_items(block.items) {
        Ok(items) => items,
        Err(e) => return e.into_compile_error().into_token_stream().into(),
    };

    block.items = items;
    quote! {
        // #(#caches);*
        #block
    }
    .into()
}

fn expand_impl_items(items: Vec<ImplItem>) -> syn::Result<(Vec<TokenStream>, Vec<ImplItem>)> {
    items.into_iter().map(expand_impl_item).try_fold(
        (vec![], vec![]),
        |(mut caches, mut items), fn_expansion| {
            let fn_expansion = fn_expansion?;
            match fn_expansion {
                FnExpansion::Cached { cache, fns } => {
                    caches.push(cache);
                    items.extend(fns);
                }
                FnExpansion::Uncached(item) => {
                    items.push(item);
                }
            }
            Ok((caches, items))
        },
    )
}

#[proc_macro_attribute]
pub fn cached(
    _attribute: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    body
}

#[derive(Attribute)]
#[attribute(ident = "caching")]
struct KeyMappingDef {
    skip: bool,
    key: Option<Expr>,
}

enum FnExpansion {
    Cached {
        cache: TokenStream,
        fns: [ImplItem; 2],
    },
    Uncached(ImplItem),
}

fn has_attr(fun: &ImplItemMethod) -> bool {
    fun.attrs
        .iter()
        .any(|attrs| attrs.path.is_ident(CRATE_NAME))
}

fn rm_attrs(attrs: &mut Vec<syn::Attribute>) {
    *attrs = attrs
        .drain(..)
        .filter(|attr| !attr.path.is_ident(CRATE_NAME))
        .collect();
}

fn rm_fn_attrs(fun: &mut ImplItemMethod) {
    rm_attrs(&mut fun.attrs);
    fun.sig.inputs.iter_mut().for_each(|arg| {
        rm_attrs(match arg {
            Receiver(arg) => &mut arg.attrs,
            Typed(arg) => &mut arg.attrs,
        })
    })
}

fn obtain_key_mapping_exprs(args: &Punctuated<FnArg, Token!(,)>) -> syn::Result<Vec<syn::Expr>> {
    args.iter().try_fold(
        Vec::<syn::Expr>::new(),
        |mut key_mapping_exprs, arg| -> syn::Result<_> {
            let attrs = match arg {
                Receiver(arg) => &arg.attrs,
                Typed(arg) => &arg.attrs,
            };
            let key_mapping_def = KeyMappingDef::from_attributes(attrs.to_owned())?;
            if !key_mapping_def.skip {
                let key_mapping_expr = if let Some(expr) = key_mapping_def.key {
                    expr
                } else {
                    match arg {
                        Receiver(_) => parse_quote!(self),
                        Typed(arg) => match &*arg.pat {
                            Pat::Ident(pat_ident) => {
                                let ident = &pat_ident.ident;
                                parse_quote!(#ident)
                            }
                            _ => unreachable!("expected no patterns"),
                        },
                    }
                };
                key_mapping_exprs.push(key_mapping_expr);
            }
            Ok(key_mapping_exprs)
        },
    )
}

fn replace_patterns_with_idents(inputs: &mut Punctuated<FnArg, Token!(,)>) {
    inputs.iter_mut().enumerate().for_each(|(index, arg)| {
        match arg {
            Typed(PatType { pat, .. }) if matches!(pat.as_ref(), Pat::Ident(_)) => {}
            Typed(pat_type) => {
                let ident = format_ident!("__{index}");
                pat_type.pat = Box::new(parse_quote!(#ident));
            }
            _ => (),
        };
    });
}

fn obtain_idents(inputs: &Punctuated<FnArg, Token!(,)>) -> Punctuated<Ident, Token!(,)> {
    inputs
        .iter()
        .map(|arg| -> Ident {
            match arg {
                Receiver(r) => r.self_token.into(),
                Typed(pat_typ) => match pat_typ.pat.as_ref() {
                    syn::Pat::Ident(ident) => ident.ident.clone(),
                    _ => unreachable!("expected no patterns"),
                },
            }
        })
        .collect()
}

fn expand_impl_item(impl_item: ImplItem) -> syn::Result<FnExpansion> {
    let mut original_fn = match impl_item {
        ImplItem::Method(fun) if has_attr(&fun) => fun,
        other => return Ok(FnExpansion::Uncached(other)),
    };

    let key_mapping_exprs = obtain_key_mapping_exprs(&original_fn.sig.inputs)?;
    rm_fn_attrs(&mut original_fn);

    let mut uncached_fn = original_fn.clone();
    let uncached_fn_ident = Ident::new(
        &format!("uncached_{}", uncached_fn.sig.ident),
        Span::call_site(),
    );
    uncached_fn.sig.ident = uncached_fn_ident.clone();

    let mut cached_fn = original_fn;

    replace_patterns_with_idents(&mut cached_fn.sig.inputs);

    let param_idents = obtain_idents(&cached_fn.sig.inputs);

    cached_fn.block = parse_quote! {{
        let key = (#(#key_mapping_exprs),*);
        Self::#uncached_fn_ident(#param_idents)
    }};

    Ok(FnExpansion::Cached {
        cache: quote! {},
        fns: [ImplItem::Method(cached_fn), ImplItem::Method(uncached_fn)],
    })
}
