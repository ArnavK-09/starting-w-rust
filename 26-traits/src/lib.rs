// A type’s behavior consists of the methods we can call on that type.
// Different types share the same behavior if we can call the same methods on all of those types.
// Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

// Public trait
pub trait Summary {
    fn summarize(&self) -> String;

    // Default func
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }
}

// Demo structs
#[derive(Default)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: Option<String>,
}
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: Option<bool>,
    pub repost: bool,
}

// Implementing trait on type
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    // WARN: Not member for trait
    // fn name(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

// Using Traits as Parameters
pub fn notify(item: &impl Summary) {
    // parameter accepts any type that implements the specified trait
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
// The impl Trait syntax works for straightforward cases
// but is actually syntax sugar for a longer form known as a trait bound

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

use std::fmt::{Debug, Display};

// Multiple Trait Bounds with the + Syntax
pub fn notify3(item: &(impl Summary + Display)) {}
pub fn notify4<T: Summary + Display>(item: &T) {}

// Clearer Trait Bounds with where Clauses
fn clear_bounds<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    unimplemented!()
}
// Returning Types That Implement Traits
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: Some(false),
        repost: false,
    }
}

// However, you can only use impl Trait if you’re returning a single type.
//
// For example, this code that returns either a NewsArticle or a SocialPost with the return type specified as impl Summary wouldn’t work
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         SocialPost {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             repost: false,
//         }
//     }
// }

// Using Trait Bounds to Conditionally Implement Methods
// By using a trait bound with an impl block that uses generic type parameters,
// we can implement methods conditionally for types that implement the specified traits.
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: PartialOrd + Display> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements another trait.
// Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations

// impl<T: Display> ToString for T {
//     // --snip--
// }
