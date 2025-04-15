use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{Error, LitStr};

pub fn transform(arg: LitStr) -> syn::Result<TokenStream> {
    let span = arg.span();
    let arg = arg.value();

    if arg.is_empty() {
        return Err(Error::new(span, "expected at least one element"));
    } else if !arg.starts_with('/') {
        return Err(Error::new(span, "expected path argument"));
    } else if arg.ends_with('/') {
        return Err(Error::new(span, "expected path argument"));
    }

    let mut sep = 0;

    Ok(quote_spanned!(span => usafe { crate::fs::VPath::new_unchecked(#arg) }))
}