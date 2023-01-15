use proc_macro::TokenStream;
use syn::{
    parse_macro_input, DeriveInput,
    __private::{quote::quote, ToTokens},
};

/// Creates `enum_map::Enumerated` implementation for the underlying Enum.
/// Also derives Copy and Clone.
#[proc_macro_attribute]
#[deprecated(since = "0.4.0", note = "Use #[derive(Enumerated)] instead")]
pub fn enum_collections(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generics = &input.generics;
    let name = &input.ident;
    let enum_count: usize = match &input.data {
        syn::Data::Enum(en) => en.variants.iter().count(),
        syn::Data::Struct(_) | syn::Data::Union(_) => {
            return quote! {
                #input
                compile_error!("The `enummap` macro only supports enums.");
            }
            .to_token_stream()
            .to_token_stream()
            .into();
        }
    };

    let output = quote! {
        #input

        impl #generics Enumerated for #name #generics {

            fn position(self) -> usize {
                self as usize
            }

            const fn len() -> usize{
                #enum_count
            }

        }
    };
    TokenStream::from(output.to_token_stream())
}

/// Creates `enum_map::Enumerated` implementation for the underlying Enum.
/// Also derives Copy and Clone.
#[proc_macro_derive(Enumerated)]
pub fn derive_enum_collections(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generics = &input.generics;
    let name = &input.ident;
    let enum_count: usize = match &input.data {
        syn::Data::Enum(en) => en.variants.iter().count(),
        syn::Data::Struct(_) | syn::Data::Union(_) => {
            return quote! {
                #input
                compile_error!("The `enummap` macro only supports enums.");
            }
            .to_token_stream()
            .to_token_stream()
            .into();
        }
    };

    let output = quote! {
        impl #generics Enumerated for #name #generics {

            fn position(self) -> usize {
                self as usize
            }

            fn len() -> usize{
                #enum_count
            }

        }
    };
    TokenStream::from(output.to_token_stream())
}
