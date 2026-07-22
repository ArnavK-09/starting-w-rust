// this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
// help: consider introducing a named lifetime parameter
// fn longest(str1: &str, str2: &str) -> &str {
//     if str1.len() > str2.len() {
//         return str1;
//     }
//     str2
// }

// Lifetime Annotation Syntax
// Lifetime annotations don’t change how long any of the references live.
// Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
fn longest2<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    // names of lifetime parameters must start with an apostrophe ('), all lowercase and very short
    if str1.len() > str2.len() {
        return str1;
    }
    str2
}

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// In Structs
#[derive(Debug)]
#[allow(dead_code)]
struct ABC<'b> {
    name: &'b str,
}

fn main() {
    let str1 = "wow";
    let str2 = "woww";
    // dbg!(longest(str1, str2));
    dbg!(longest2(str1, str2));

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest2(string1.as_str(), string2.as_str());
        println!("The longest string is:- {result}");
    }
    // shows that the lifetime of the reference in result must be the smaller lifetime of the two arguments.

    let a = ABC {
        name: string1.as_str(),
    };
    dbg!(a);

    // The Static Lifetime
    // which denotes that the affected reference can live for the entire duration of the program.
    let _s: &'static str = "I have a static lifetime.";
    // The text of this string is stored directly in the program’s binary, which is always available.
}

// The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

// The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

// The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.
