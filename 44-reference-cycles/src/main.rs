// Reference Cycles Can Leak Memory
//
// Rust’s memory safety guarantees make it difficult, but not impossible,
// to accidentally create memory that is never cleaned up (known as a memory leak).

// Rust allows memory leaks with Rc<T> and RefCell<T>:
//  items can refer to each other in a cycle,
//  so reference counts never reach 0 and values are never dropped.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[allow(dead_code)]
fn main() {
    // Creating a Reference Cycle
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    use List::*;

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    // Creating a Reference Cycle
    // Graph of reference counts at each step:
    //
    // Step 1: Create `a` (points to Nil)
    //   a: rc=1  ──►  Nil: rc=1
    //   (a) ──tail──► (Nil)
    //
    // Step 2: Create `b` (points to `a`)
    //   b: rc=1  ──►  a: rc=2  ──►  Nil: rc=1
    //   (b) ──tail──► (a) ──tail──► (Nil)
    //
    // Step 3: Modify `a`'s tail to point to `b` (creates cycle)
    //   b: rc=2  ◄───┐
    //   ▲            │
    //   │            ▼
    //   a: rc=2  ──► b: rc=2
    //   (a) ──tail──► (b) ──tail──► (a) ──► ... infinite cycle
    //
    // Result: Neither `a` nor `b` can be dropped (rc never reaches 0)
    // Stack overflow if we try to print the cycle (uncomment last println!)

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // println!("a next item = {:?}", a.tail());
    // err: fatal runtime error: stack overflow, aborting\

    // Preventing Reference Cycles Using Weak<T>

    // create a weak reference to the value within an Rc<T> instance by calling Rc::downgrade

    // Strong references are how you can share ownership of an Rc<T> instance.
    // Weak references don’t express an ownership relationship,
    // and their count doesn’t affect when an Rc<T> instance is cleaned up.
    // Relationship Graph between Parent (branch) and Child (leaf):

    // Weak => I know where this value is, but I do not keep it alive

    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
        parent: RefCell<Weak<Node>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()), // empty weak pointer
    });
    println!(
        "\n\nleaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // To safely access a `Weak<T>`'s inner value, you must call `upgrade()` to temporarily convert it to a strong `Rc<T>` and ensure the value hasn't been dropped.
    // A `Weak<T>` does **not** let you safely use the inner value directly.

    // children: Rc<Node>  strong ownership
    // parent: Weak<Node>  non-owning link

    {
        println!("[in scope]");
        let branch = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });
        // Node in leaf now has two owners: leaf and branch
        // Replaces the old empty weak pointer with a new one
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // create a **weak, non-owning link**

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); //  Attempts to convert a `Weak<T>` reference into a strong `Rc<T>` reference.
        // `upgrade()` is needed because a weak reference may point to already-dropped data; upgrading checks that it is alive and temporarily keeps it alive.

        println!(
            "\nbranch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        // branch will have a strong count of 1 and a weak count of 1 (for leaf.parent pointing to branch with a Weak<Node>).

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        // leaf has a strong count of 2 because branch holds a clone of its Rc<Node> in branch.children.
        // It has a weak count of 0 because no Weak<Node> references point to it.
    }
    println!("[out scope]");
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    //      ┌─────────────────┐
    //      │  branch (Node)  │◀───-┐ (Weak Parent Link)
    //      └────────┬────────┘     │
    //               │              │
    //   (Strong     │ (Rc)         │
    //    Children   ▼              │
    //    Link)  ┌────────────────┐ │
    //           │   leaf (Node)  ├─┘
    //           └────────────────┘

    // Adding a Reference from a Child to Its Parent
}
