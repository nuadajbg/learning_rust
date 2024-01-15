extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use quote::TokenStreamExt;

/// Derives the Printable trait for a structure
#[proc_macro_derive(Printable)]
pub fn printable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_printable(&ast)
}

/// Derives the Printable trait for a structure
fn impl_printable(ast: &syn::DeriveInput) -> TokenStream {
    let gen = quote! { };
    gen.into()
}
