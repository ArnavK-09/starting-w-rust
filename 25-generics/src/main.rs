fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// To parameterize the types in a new single function, we need to name the type parameter,
// just as we do for the value parameters to a function.
// You can use any identifier as a type parameter name. But we’ll use T because, by convention, type parameter names in Rust are short, often just one letter, and Rust’s type-naming convention is UpperCamelCase.

// INFO: Traits later on

// fn findlargest<T>(list: &[T]) ->T {
//     let mut largest  = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// Generics for structs
struct Point<T, MAT> {
    name: T,
    lastname: MAT,
}

// Generics for enum
enum Type<G> {
    Some(G),
    None,
}

// For methods, generics
impl<G> Point<G, G> {
    fn f(&self) -> &G {
        &self.name
    }
}

fn main() {
    let numbs = vec![12, 1, 2, 12, 1, 21, 2, 69];
    print!("{} is largest", largest(&numbs));
}
