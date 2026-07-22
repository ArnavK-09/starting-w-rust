#![allow(unused_imports)]
// Declaring module 
pub mod garden;

// importing sub module 
use garden::vegetable::Weed;
use garden::vegetable;

fn main() {
    println!("Hello, world!");
}
