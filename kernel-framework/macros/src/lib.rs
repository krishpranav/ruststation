use proc_macro::TokenStream;
use syn::{Error, ItemStruct, LitInt, TraitItem, parse_macro_input};

mod derive;
mod offset;

#[proc_macro_derive(MappedKernel)]
pub fn derive_mapped_kernel(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);

    self::derive::mapped_kernel(item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn offset(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as LitInt);
    let item = parse_macro_input!(item as TraitItem);

    self::offset::transform(args, item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}