extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;
use std::fs;
use std::fs::ReadDir;
use case::CaseExt;

use proc_macro2::{Ident, Span};

#[proc_macro_derive(UsecaseBusMacro)]
pub fn usecase_bus_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_bus_macro(&ast)
}

fn impl_bus_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut impl_trait: Vec<proc_macro2::TokenStream> = Vec::new();

    if let Ok(entries) = fs::read_dir("src/domain/usecases/") {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                if let Some(filename) = entry.file_name().to_str() {
                    let usecase_name = filename.replace(".rs", "").to_camel();
                    let input = format!("{}Input", usecase_name);
                    let output = format!("{}Output", usecase_name);
                    let impl_klass = format!("Default{}Impl", usecase_name);

                    let input = Ident::new(&input, Span::call_site());
                    let output = Ident::new(&output, Span::call_site());
                    let impl_klass = Ident::new(&impl_klass, Span::call_site());

                    impl_trait.push(quote!{
                        impl IUsecaseBus<#input, #output> for #name{}

                        impl Handler<#input, #output> for #name {
                            fn handle(&self, input: #input) -> #output{
                                println!("handle");

                                let usecase = #impl_klass::new(DefaultTodoRepositoryImpl::new(DefaultKVS::new()));
                                usecase.handle(input)
                            }
                        }
                    });
                }
            }
        }
    }

    let gen = quote! {
        #(#impl_trait)*
    };

    gen.into()
}
