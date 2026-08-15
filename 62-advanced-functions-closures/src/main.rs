// Advanced Functions and Closures

#![allow(unused)]

fn main() {
    // Function Pointers

    // You can pass regular functions just like closures.
    // Regular functions coerce to the fn type (a function pointer), not the Fn trait.
    // Function pointers let you pass existing functions as arguments.

    // The fn type is called a function pointer.

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    // Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce),
    // meaning you can always pass a function pointer as an argument for a function that expects a closure.

    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");

    // Some APIs need fn, not closures, like C code that can't use closures.
    // map can take either an inline closure or a named function.

    // Enum variants can also act like function pointers
    enum Status {
        Value(u32),
        Stop,
    }

    // This makes a range of numbers from `0` up to `19`.
    // - `0u32` means the starting number is a `u32`
    // - `..20` means stop before 20
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    // `Status::Value` is not just a name — it behaves like a small function that takes a `u32` and builds a `Status::Value(u32)`.

    // via closure same
    let list_of_statuses: Vec<Status> = (0u32..20).map(|x| Status::Value(x)).collect();

    // Returning Closures
    // Closures are represented by traits, which means you can’t return closures directly.
    // A closure has no single concrete return type, especially if it captures values.
    // `fn` can only be returned for non-capturing closures.

    // Instead, you will normally use the impl Trait syntax
    fn returns_closure() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    // Each closure has its own unique type, even with the same signature.
    // Notice that the closures that they return are different, even though they implement the same type.
    fn returns_closure2() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
        move |x| x + init
    }

    // let handlers = vec![returns_closure(), returns_initialized_closure(123)]; // err: expected opaque type, found a different opaque type

    // Rust creates a unique opaque type,
    // a type where we cannot see into the details of what Rust constructs for us,
    // nor can we guess the type Rust will generate to write ourselves.

    // even though these functions return closures that implement the same trait,
    // the opaque types Rust generates for each are distinct

    // - "I’m returning some callable thing"
    // - Rust hides the exact concrete type
    // - It can be a closure
    // - That closure can even capture values from its environment
    fn returns_closure3() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    // - "I’m returning a function pointer"
    // - It must be a plain function-like value
    // - It cannot capture anything from outside
    // - It is just an address to code
    fn returns_closure4() -> fn(i32) -> i32 {
        |x| x + 1
    }

    fn make_adder1(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }
    // fn make_adder2(n: i32) -> fn(i32) -> i32 {
    //     move |x| x + n // error
    // }

    // When `fn` works
    fn returns_fn() -> fn(i32) -> i32 {
        |x| x + 1
    }
    // Because `|x| x + 1` does not capture anything.

    // `fn(i32) -> i32`
    // This returns a function pointer

    // `impl Fn(i32) -> i32`
    // This returns some hidden concrete type that implements `Fn(i32) -> i32`

    // A function pointer is a value that points to a function, just like a normal pointer points to data.
}
