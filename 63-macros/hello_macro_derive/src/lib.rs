use proc_macro::TokenStream;
use quote::quote;

// `hello_macro_derive` runs when a type uses `#[derive(HelloMacro)]`.
// `proc_macro_derive(HelloMacro)` ties the macro to that derive name.
// Matching the trait name is the usual convention.

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // `#name` like a placeholder in a template:
    // - `#name` = fill in this spot with the actual type name
    // - `quote!` = the template engin
    let generated = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}
