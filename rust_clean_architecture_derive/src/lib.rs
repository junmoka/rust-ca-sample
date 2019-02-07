extern crate proc_macro;

use case::CaseExt;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::fs;
use syn;

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
                    let input = Ident::new(&format!("{}Input", usecase_name), Span::call_site());
                    let output = Ident::new(&format!("{}Output", usecase_name), Span::call_site());
                    let impl_klass =
                        Ident::new(&format!("Default{}Impl", usecase_name), Span::call_site());

                    impl_trait.push(quote! {
                        impl IUsecaseBus<#input, #output> for #name{}

                        impl Handler<#input, #output> for #name {
                            fn handle(&self, input: #input) -> #output{
                                let usecase = #impl_klass::new();
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

#[proc_macro_derive(UsecaseMacro)]
pub fn usecase_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_usecase_macro(&ast)
}

fn impl_usecase_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    let mut impl_new: Vec<proc_macro2::TokenStream> = Vec::new();

    if let syn::Data::Struct(body) = &ast.data {
        if let syn::Fields::Named(fields) = &body.fields {
            let it = &mut fields.named.iter();

            while let Some(t) = it.next() {
                if let Some(ident) = &t.ident {
                    if let syn::Type::Path(path_type) = &t.ty {
                        impl_new.push(quote! {
                            #ident: #path_type::new()
                        });
                    }
                }
            }
        }
    }

    let input = Ident::new(
        &format!("{}Input", name.to_string().replace("Impl", "")),
        Span::call_site(),
    );
    let output = Ident::new(
        &format!("{}Output", name.to_string().replace("Impl", "")),
        Span::call_site(),
    );

    let gen = quote! {
        impl #impl_generics Usecase<#input, #output> for #name #ty_generics #where_clause{}
    };
    gen.into()
}

#[proc_macro_derive(New)]
pub fn new_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_new_macro(&ast)
}

fn impl_new_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    let mut impl_new: Vec<proc_macro2::TokenStream> = Vec::new();

    if let syn::Data::Struct(body) = &ast.data {
        if let syn::Fields::Named(fields) = &body.fields {
            let it = &mut fields.named.iter();

            while let Some(t) = it.next() {
                if let Some(ident) = &t.ident {
                    if let syn::Type::Path(path_type) = &t.ty {
                        impl_new.push(quote! {
                            #ident: #path_type::new()
                        });
                    }
                }
            }
        }
    }

    let gen = quote! {
        impl #impl_generics New for #name #ty_generics #where_clause{
            fn new() -> Self{
                Self{ #(#impl_new),* }
            }
        }
    };
    gen.into()
}
