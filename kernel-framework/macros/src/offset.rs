use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{
    Error, FnArg, LitInt, Pat, PatType, Token, TraitItem, TraitItemConst, TraitItemFn, parse_quote,
};

pub fn transform(args: LitInt, item: TraitItem) -> syn::Result<TokenStream> {
    match item {
        TraitItem::Const(i) => transform_const(args, i),
        TraitItem::Fn(i) => transform_fn(args, i),
        v => Err(Error::new_spanned(v, "unsupported offset item")),
    }
}

fn transform_const(args: LitInt, mut item: TraitItemConst) -> syn::Result<TokenStream> {
    if let Some((b, _)) = item.default {
        return Err(Error::new_spanned(b, "expect `;`"));
    }

    let offset: usize = args.base10_parse()?;
    let ty = &item.ty;

    item.default = Some((
        parse_quote!(=),
        parse_quote!(unsafe { <#ty>::new(#offset) }),
    ));

    Ok(item.into_token_stream())
}

fn transform_fn(args: LitInt, mut item: TraitItemFn) -> syn::Result<TokenStream> {
    if let Some(b) = item.default {
        return Err(Error::new_spanned(b, "expect `;`"));
    }

    let offset: usize = args.base10_parse()?;
    let sig = &item.sig;
    let ret = &sig.output;
    let mut params = Punctuated::<&PatType, Token![,]>::new();
    let mut args = Punctuated::<&Pat, Token![,]>::new();

    for p in sig.inputs.iter().skip(1) {
        let p = match p {
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(v) => v,
        };

        params.push(p);
        args.push(&p.pat);
    }

    item.default = Some(parse_quote!({
        let _addr = unsafe { self.addr().add(#offset) };
        let _fp: unsafe extern "C" fn(#params) #ret = unsafe { core::mem::transmute(_addr) };
        unsafe { _fp(#args) }
    }));

    Ok(item.into_token_stream())
}