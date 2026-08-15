// Advanced Types

#![allow(unused)]

// Type Safety and Abstraction with the Newtype Pattern

// Newtypes also statically prevent mixing values (units/types)
// Newtypes abstract away inner APIs; expose a custom public surface
// while hiding the private inner type (e.g., People wrapping a
// HashMap<i32, String> — callers see add_name(), not the id logic).
// Lightweight encapsulation: hide implementation details.

fn main() {
    // Type Synonyms and Type Aliases

    // declare a type alias to give an existing type another name
    type Kilometers = i32;
    // Kilometers is a synonym for i32

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
    // Type aliases are also commonly used with the Result<T, E> type for reducing repetition.

    // The Never Type That Never Returns

    // Rust has a special type named `!` that’s known
    // in type theory lingo as the empty type because it has no values
    fn _bar() -> ! {
        loop {
            // never ends
        }
    }

    // Functions that return never are called diverging functions.
    // continue has a ! value

    // Dynamically Sized Types and the Sized Trait

    // Rust usually knows a type's size at compile time.
    // Dynamically sized types (DSTs) are unsized types whose size is only known at runtime.

    // dynamically sized type example => str
    // not &str, but str on its own

    // when storing text entered by a user, we can’t know how long the string is until runtim
    // let s1: str = "Hello there!"; // err: doesn't have a size known at compile-time

    // We make the type of s1 string slice (&str) rather than str

    // although &T is a single value that stores the memory address of where the T is located,
    // a string slice is two values:
    // the address of the str & its length

    // we can know the size of a string slice value at compile time:
    //     twice the length of a usize

    // DSTs carry extra metadata, usually their size.
    // They must always be stored behind some kind of pointer.

    // `usize` is Rust’s “size of this machine” integer
    // - 32-bit machine, `usize` is 32 bits
    // - 64-bit machine, `usize` is 64 bit

    // To work with DSTs,
    // Rust provides the Sized trait to determine whether or not
    // a type’s size is known at compile time.

    // Rust implicitly adds a bound on Sized to every generic function

    fn generic<T>(t: T) {}
    // is actually treated as though we had written this:
    fn _generic<T: Sized>(t: T) {}
    // By default, generic functions will work only on types that have a known size at compile time.

    // However, you can use the following special syntax to relax this restriction:
    fn __generic<T: ?Sized>(t: &T) {}
    // A trait bound on ?Sized means “T may or may not be Sized,”
}
