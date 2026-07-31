// Rc<T>, the Reference-Counted Smart Pointer
//
// cases when a single value might have multiple owners
// The Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use.
// If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

// Note that Rc<T> is only for use in single-threaded scenarios.

// We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read
//  and we can’t determine at compile time which part will finish using the data last

#![allow(dead_code)]
#![allow(unused_variables)]
use std::rc::Rc;

enum List {
    Cons(i32, Box<List>),
    Nil,
}
enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}
use crate::List2::{Cons, Nil};

fn main() {
    let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
    let b = List::Cons(3, Box::new(a));
    // let c = List::Cons(4, Box::new(a)); // err: value used here after move

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(3, Rc::clone(&a));

    // We could have called a.clone() rather than Rc::clone(&a),
    //  but Rust’s convention is to use Rc::clone in this case
    //
    //  The implementation of Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do.
    //  The call to Rc::clone only increments the reference count, which doesn’t take much time

    // Cloning to Increase the Reference Count
    //
    // Rc::strong_count() => the number of strong references / owners to the value inside `Rc`.

    let a = Rc::new(Cons(1, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(2, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(3, Rc::clone(&a));
        println!(
            "count after creating c (in scope) = {}",
            Rc::strong_count(&a)
        );
    }
    // The implementation of the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope.
    println!(
        "count after creating c (out scope) = {}",
        Rc::strong_count(&a)
    );

    // Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only
}
// Rc::strong_count(&a) => 0
