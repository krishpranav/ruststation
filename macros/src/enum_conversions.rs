use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemEnum;

pub fn transform(arg: ItemEnum) -> syn::Result<TokenStream> {
    let enum_name = &arg.ident;
    let mut stream = TokenStream::new();

    Ok(stream);
}