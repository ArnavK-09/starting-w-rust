// Documentation comments use three slashes, ///, instead of two and support Markdown notation for formatting the text

// We can generate the HTML documentation from this documentation comment by running `cargo doc`.

// The style of doc comment //! adds documentation to the item that contains the comments rather than to the items following the comments.

//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
//!
//!
//! When we run cargo doc --open, these comments will display on the front page of the documentation for my_crate above the list of public items in the crate,

/// Adds one to the number given.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Running `cargo test` will run the code examples in your documentation as tests!

// Commonly Used Sections
// We used the # Examples Markdown heading in Listing 14-1 to create a section in the HTML with the title "Examples." Here are some other sections that crate authors commonly use in their documentation:
//
// Panics: These are the scenarios in which the function being documented could panic. Callers of the function who don't want their programs to panic should make sure they don't call the function in these situations.
// Errors: If the function returns a Result, describing the kinds of errors that might occur and what conditions might cause those errors to be returned can be helpful to callers so that they can write code to handle the different kinds of errors in different ways.
// Safety: If the function is unsafe to call (we discuss unsafety in Chapter 20), there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold.

// Before you can publish any crates, you need to create an account on crates.io, `cargo login`
// `cargo publish` Publish crate

// Yanking a version with `cargo yank --vers <version>` prevents new projects from adding it as a
// dependency while allowing existing ones with a Cargo.lock to continue working. You can undo this
// with `cargo yank --vers <version> --undo`. Yanking does not delete code or uploaded secrets.
