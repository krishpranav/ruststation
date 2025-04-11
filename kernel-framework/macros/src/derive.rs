use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Fields, ItemStruct, Type};

pub fn mapped_kernel(item: ItemStruct) -> syn::Result<TokenStream> {
    let fields = match item.fields {
        Fields::Unnamed(v) if v.unnamed.len() == 1 => v,
        v => {
            return Err(Error::new_spanned(
                v,
                "expect a struct with an unnamed field",
            ));
        }
    };

    let ty = match &fields.unnamed[0].ty {
        Type::Ptr(v) => v,
        v => return Err(Error::new_spanned(v, "expect a pointer")),
    };

    if let Some(t) = ty.mutability {
        return Err(Error::new_spanned(t, "expect `const`"));
    }

    match ty.elem.as_ref() {
        Type::Path(p) if p.qself.is_none() && p.path.is_ident("u8") => {}
        v => return Err(Error::new_spanned(v, "expect `u8`")),
    }

    let ident = item.ident;

    Ok(quote! {
        impl Default for #ident {
            fn default() -> Self {
                let mut rdx: usize;
                let mut rax: usize;

                unsafe {
                    core::arch::asm!(
                        "rdmsr",
                        in("ecx") 0xc0000082u32,
                        out("rdx") rdx,
                        out("rax") rax, 
                        options(pure, nomem, preserves_flags, nostack)
                    );
                }

                let aslr = ((rdx << 32) | rax) - 0xffffffff822001c0;
                let base = aslr + 0xffffffff82200000;

                Self(base as *const u8)
            }
        }

        impl okf::MappedKernel for #ident {
            fn addr(self) -> *const u8 {
                self.0
            }
        }
    })
}