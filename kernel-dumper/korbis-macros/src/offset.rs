use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, Error, Ident, LitInt, Pat, PatType, Receiver, ReturnType, Token};
use syn::token::Token;

pub fn transform(args: LitInt, items: OffsetItem) -> syn::Result<TokenStream> {
    match item {
        OffsetItem::Method(v) => transform_method(args, v),
    }

}

fn transform_method(args: LitInt, item: Method) -> syn::Result<TokenStream> {
    let offset: usize = args.base10_parse()?;
    let unsafety = item.unsafety;
    let ident = item.ident;
    let receiver = item.receiver;
    let params = item.params;
    let ret = item.ret;

    Ok(quote! {
    })
}

pub enum OffsetItem {
    Method(Method),
}

pub struct Method {
    unsafety: Option<Token![unsafe]>,
    ident: Ident,
    receiver: Receiver,
    params: Punctuated<PatType, Token![,]>,
    ret: ReturnType
}