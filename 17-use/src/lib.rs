mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// we can create a shortcut to a path with the use keyword once, and then use the shorter name everywhere else in the scope.
use crate::front_of_house::hosting;

// Before this change, external code would have to call the add_to_waitlist function by using the path restaurant::front_of_house::hosting::add_to_waitlist(), which also would have required the front_of_house module to be marked as pub.
// Now that this pub use has re-exported the hosting module from the root module, external code can use the path restaurant::hosting::add_to_waitlist() instead.
pub use crate::front_of_house::hosting; // re exporting

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// Note that use only creates the shortcut for the particular scope in which the use occurs.
mod customer {
    pub fn eat_at_restaurant() {
        // different scope than the use statement, so the function body won’t compile
        // hosting::add_to_waitlist();
    }
}

// idiomatic way to bring the standard library’s HashMap struct into the scope
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    use customer::*;
    eat_at_restaurant();
}

// The exception to this idiom is if we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that.

// Providing New Names with the as Keyword
use std::fmt::Result;
use std::io::Result as IoResult;

// Using Nested Paths to Clean Up Large use Lists
use std::cmp::Ordering;
use std::io;
// or
use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;
// or
use std::io::{self, Write};

// If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator:
use std::*;

