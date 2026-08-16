// Macros
// refers to a family of features in Rust

#![allow(unused)]
#![allow(non_local_definitions)]
// Declarative macros with macro_rules! and three kinds of procedural macros:

// - Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
// - Attribute-like macros that define custom attributes usable on any item
// - Function-like macros that look like function calls but operate on the tokens specified as their argument

// Metaprogramming is useful for reducing the amount of code you have to write and maintain

// The Difference Between Macros and Functions

// Macros write code that writes code, so they run before compile time.
// They can take a variable number of arguments and do things functions can't.
// The tradeoff is more complex, harder-to-read definitions.
// Macros must be defined or imported before use in a file.

fn main() {
    // The most widely used form of macros in Rust is the declarative macro.

    // The #[macro_export] annotation indicates that this macro should be made available
    // whenever the crate in which the macro is defined is brought into scope.
    // Without this annotation, the macro can’t be brought into scope.

    // We then start the macro definition with macro_rules!
    // and the name of the macro we’re defining without the exclamation mark
    // followed by curly brackets denoting the body of the macro definition.
    #[macro_export]
    macro_rules! vec {
// use a dollar sign ($) to declare a variable in the macro system that will contain the Rust code matching the pattern
// $x:expr, matches any Rust expression and gives the expression the name $x
    ($($x:expr),*) => {
        // comma following $() indicates that a literal comma separator character
        // must appear between each instance of the code that matches the code in $()
        // `*` specifies that the pattern matches zero or more of whatever precedes the `*`
       {
           let mut temp_v = Vec::new();
           // Repeat this block once for every matched item
           $(
           temp_v.push($x);
           )*
           temp_v
       }
    };
}

    // When we call this macro with vec![1, 2, 3];, the $x pattern matches three times with the three expressions 1, 2, and 3.
    let v: Vec<u32> = vec![1, 2, 3];
    // within $()* is generated for each part that matches $()
    // $x is replaced with each expression matched

    // code generated that replaces this macro call will be the following:
    {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec; // return it
    }

    // `#[macro_export]` means:
    // - make this macro available from the crate root
    // - possibly export it to other crates too
    // So Rust expects it to be written at the top level of the module

    // Procedural Macros for Generating Code from Attributes
    //
    // Second form of macros is the procedural macro, which acts more like a function
    // Procedural macros accept some code as an input, operate on that code, and produce some code as an output rather than matching against patterns
    // and replacing the code with other code as declarative macros do
    //
    // Three kinds of procedural macros are custom derive, attribute-like, and function-like

    // When creating procedural macros, the definitions must reside in their own crate with a special crate type

    // ```rust
    // use proc_macro::TokenStream;
    // #[some_attribute]
    // pub fn some_name(input: TokenStream) -> TokenStream {
    // }
    // ```
    //
    // Procedural macros take TokenStream input and return TokenStream output.
    // TokenStream is a sequence of code tokens from proc_macro.
    // The attribute on the function says which procedural macro kind it is.
    // One crate can define multiple procedural macro kinds.
    // Procedural macros need to be in their own crate

    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Pancakes;

    Pancakes::hello_macro();

    // Attribute-Like Macros
    //
    // Attribute-like macros are similar to custom derive macros,
    // but instead of generating code for the derive attribute, they allow you to create new attributes.

    // They’re also more flexible:
    // derive only works for structs and enums
    // attributes can be applied to other items as well, such as functions.

    // Examples

    // This #[route] attribute would be defined by the framework as a procedural macro
    // #[route(GET, "/")]
    fn index() {}

    // The signature of the macro definition function would look like this:
    // ```rust
    // #[proc_macro_attribute]
    // pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // }
    // ```
    //
    // Two TokenStream inputs: the attribute contents and the item body.
    // The first is the attribute data, like GET and "/".
    // The second is the code item the attribute is attached to.

    // Other than that, attribute-like macros work the same way as custom derive macros:
    // You create a crate with the proc-macro crate type and implement a function
    // that generates the code you want!

    // Function-Like Macros
    //
    // Function-like macros define macros that look like function calls.
    // they’re more flexible than functions; for example, they can take an unknown number of arguments.

    // Examples

    // let sql = sql!(SELECT * FROM posts WHERE id=1);

    // This macro would parse the SQL statement inside it and check that it’s syntactically correct,
    // which is much more complex processing than a macro_rules! macro can do.

    // The sql! macro would be defined like this:
    // ```rust
    // #[proc_macro]
    // pub fn sql(input: TokenStream) -> TokenStream {
    // ```
}
