extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
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

    impl_trait.push(quote!{
        impl IUsecaseBus<CreateTodoInput, CreateTodoOutput> for #name{}

        impl Handler<CreateTodoInput, CreateTodoOutput> for #name {
            fn handle(&self, input: CreateTodoInput) -> CreateTodoOutput{
                println!("handle CreateTodo");

                let usecase = DefaultCreateTodoImpl::new(DefaultTodoRepositoryImpl::new(DefaultKVS{}));
                usecase.handle(input)
            }
        }
    });

    impl_trait.push(quote!{
        impl IUsecaseBus<ShowTodoInput, ShowTodoOutput> for #name{}

        impl Handler<ShowTodoInput, ShowTodoOutput> for #name {
            fn handle(&self, input: ShowTodoInput) -> ShowTodoOutput{
                println!("handle ShowTodo");
                let usecase = DefaultShowTodoImpl::new(DefaultTodoRepositoryImpl::new(DefaultKVS{}));
                usecase.handle(input)
            }
        }
    });

    let gen = quote! {
        #(#impl_trait)*
    };

    gen.into()
}
