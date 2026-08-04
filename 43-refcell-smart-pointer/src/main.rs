// RefCell<T> and the Interior Mutability Pattern
//

// With references and Box<T>, the borrowing rules’ invariants are enforced at compile time.
//  With RefCell<T>, these invariants are enforced at runtime.
// - At any given time, you can have either one mutable reference or any number of immutable references (but not both).
// - References must always be valid.
//
// With references, if you break these rules, you’ll get a compiler error.
//  With RefCell<T>, if you break these rules, your program will panic and exit.

#![allow(unused_variables)]
#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Using Interior Mutability
    let x = 1;
    // let y = &mut x; // err: cannot borrow as mutable

    // Allowing Multiple Owners of Mutable Data
    //
    // A common way to use RefCell<T> is in combination with Rc<T>.
    //  Recall that Rc<T> lets you have multiple owners of some data, but it only gives immutable access to that data.
    //  If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and that you can mutate!

    // Code from #42
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

// Recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
//
// Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
// Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
