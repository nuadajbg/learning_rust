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
    let name = &ast.ident;
    if let &syn::Data::Struct(ref s) = &ast.data {
        let mut all_fields = proc_macro2::TokenStream::new();
        s.fields.iter().for_each(|f| {
            let n = f.ident.as_ref().unwrap();
            let gen = quote! {
                println!("{}={}", stringify!(#n), &self.#n);
            };
            all_fields.append_all(gen);
        });
        let gen = quote! {
            impl Printable for #name {
                fn print(&self) {
                    #all_fields
                }
            }
        };
        gen.into()
    } else {
        panic!("Expected a struct");
    }
}
