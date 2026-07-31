// Treating Smart Pointers Like Regular References
use std::ops::Deref;

fn main() {
    // A regular reference
    let x = 5;
    let y = &x;
    assert_eq!(5, *y); // dereference
    // assert_eq!(5, y); // error: Comparing a number and a reference

    // A smart pointer
    let a = 5;
    let b = Box::new(a);
    assert_eq!(5, *b);
    // assert_eq!(5, b); // error: can't compare `{integer}` with `Box<{integer}>`

    // Define a custom smart pointer wrapper like Box<T> to explore how it differs from references.
    struct MyBox<T>(T); // tuple struct

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // Implementing the Deref Trait
    impl<T> Deref for MyBox<T> {
        type Target = T;

        // body of the deref method with &self.0,
        //  deref returns a reference to the value we want to access with the * operator;
        fn deref(&self) -> &Self::Target {
            // &Self::Target == &T
            // `self` means the current value/object -> MyBox
            // `Self` means the current type -> MyBox<T>
            &self.0
        }
        // &MyBox<T> -> &T

        // Without the Deref trait, the compiler can only dereference `&` references
        // `*y` ==> `*(y.deref())`
    }

    let p = 5;
    let q = MyBox::new(p);
    assert_eq!(5, *q);

    // Deref Coercion in Functions and Methods
    //
    // converts a reference to a type that implements the Deref trait into a reference to another type
    // Rust automatically unwraps smart pointers when a function expects the inside value.

    hello("Sup");

    let s = MyBox::new("Hi");
    let ss = MyBox::new(String::from("How"));
    hello(&s);
    hello(&ss);

    // without Deref Coercion
    hello(&(*s)[..]);

    // Deref Coercion with Mutable References
    //
    //  &T -> &U          when T: Deref<Target = U>
    //  &mut T -> &mut U  when T: DerefMut<Target = U>
    //  &mut T -> &U      when T: Deref<Target = U>
    //
    // `&T` cannot become `&mut T`, because immutable references do not guarantee exclusive access.
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
