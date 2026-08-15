// Advanced Traits

#![allow(dead_code)]

// Associated types connect a type placeholder with a trait
// such that the trait method definitions can use these placeholder types in their signatures.
//
// That way, we can define a trait that uses some types without needing
// to know exactly what those types are until the trait is implemented.
//
// example of a trait with an associated type is the `Iterator` trait in std lib
// The associated type is named `Item` and stands in for the type of the values
// the type implementing the Iterator trait is iterating over.

fn main() {
    pub trait Iterator {
        // Item is a placeholder
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    // Difference of Associated types & Generics

    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            unimplemented!()
        }
    }

    pub trait IteratorGenerics<T> {
        fn next(&mut self) -> Option<T>;
    }

    // Generic trait params can be implemented multiple times,
    // so calls need type annotations to pick the impl.
    // Associated types allow only one impl, so no annotations needed.

    struct Counter {
        count: u32,
    }
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    // Using Default Generic Parameters and Operator Overloading
    // This eliminates the need for implementors of the trait to specify a concrete type if the default type works
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    // std implementation:

    // syntax is called default type parameters => <PlaceholderType=ConcreteType>
    // trait Add<Rhs=Self> {
    //     type Output;
    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }

    struct Millimeters(u32);
    struct Meters(u32);

    // set the value of the Rhs type parameter (Meters) instead of using the default of Self
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    // Disambiguating Between Identically Named Methods

    // Multiple traits can have same method names; a type can implement both.
    // A type can also have an inherent method with the same name as trait methods.

    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;

    // defaults to calling the method that is directly implemented on the type
    person.fly();

    // To call the identical methods from either
    Pilot::fly(&person);
    Wizard::fly(&person);
    // Because the fly method takes a self parameter

    // However, associated functions that are not methods don’t have a self parameter
    // When there are multiple types or traits that define non-method functions with the same function name,
    // Rust doesn’t always know which type you mean unless you use fully qualified syntax.

    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name());

    // providing Rust with a type annotation within the angle brackets
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);

    // Using Supertraits
    // A supertrait is a trait a trait requires; the relying trait can use its items.
    // To impl the first trait, the type must also impl the required supertrait.

    use std::fmt;

    // will only work for types that also implement Display
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {output} *");
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point2 {
        x: i32,
        y: i32,
    }

    // impl OutlinePrint for Point2 {} // err: the trait `std::fmt::Display` is not implemented for `Point`

    impl OutlinePrint for Point2 {}
    impl fmt::Display for Point2 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let p = Point2 { x: 1, y: 3 };
    p.outline_print();

    // Requiring a supertrait makes the parent's methods usable on Self.
    println!("{}", p.to_string());

    // Implementing External Traits with the Newtype Pattern

    // Orphan rule: a trait may be implemented for a type only if either
    // the trait or the type is local to this crate.
    // Circumvent this with the newtype pattern: wrap the foreign type
    // in a local one-field tuple struct, then implement the trait on it.
    // The wrapper has zero runtime cost; it's erased at compile time.

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // uses self.0 to access the inner Vec<T> because Wrapper is a tuple struct
            // Vec<T> is the item at index 0 in the tuple.
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");

    // Downside of newtype: the wrapper is opaque, so it hides
    // the inner type's methods. Delegate every needed method via
    // self.0 (manual delegation), OR implement Deref on the wrapper
    // to forward into the inner type.
    // Choose manual impls if you want to restrict the inner type's full API.
}
