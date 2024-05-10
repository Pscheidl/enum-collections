use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

/// Creates `enum_map::Enumerated` implementation for the underlying Enum.
/// Also derives Copy and Clone.
#[proc_macro_derive(Enumerated)]
pub fn derive_enum_collections(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generics = &input.generics;
    let name = &input.ident;
    let syn::Data::Enum(en) = input.data else {
        return quote_spanned! {
            input.span() => compile_error!("The `Enumerated` macro only supports enums.");
        }
        .into();
    };

    let enum_len = en.variants.len();
    let mut variants = proc_macro2::TokenStream::new();
    for variant in en.variants {
        if let Some((_, discriminant)) = variant.discriminant {
            return quote_spanned! {
                discriminant.span() => compile_error!("`Enumerated` doesn't support discriminants");
            }
            .into();
        }
        let variant_name = variant.ident;
        variants.extend(quote! { Self::#variant_name, });
    }

    quote! {
        impl #generics Enumerated for #name #generics {

            fn position(self) -> usize {
                self as usize
            }

            const SIZE: usize = #enum_len;
            #[cfg(feature = "variants")]
            const VARIANTS: &'static [Self] = &[#variants];
        }
    }
    .into()
}
