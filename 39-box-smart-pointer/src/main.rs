// `Box<T>`
//
// Boxes allow you to store data on the heap rather than the stack.
// What remains on the stack is the pointer to the heap data.

fn main() {
    let b = Box::new(5); // on heap
    println!("{b}\n\n");
    // when a box goes out of scope, as b does at the end of main, it will be deallocated.

    // The deallocation happens both for the box (stored on the stack)
    //  & the data it points to (stored on the heap)

    // Rust always knows how much space a Box<T> needs: A pointer’s size doesn’t change based on the amount of data it’s pointing to
    println!("{}", std::mem::size_of::<usize>()); // 8 on 64-bit
    println!("{}", std::mem::size_of::<Box<i32>>()); // 8 on 64-bit
    println!("{}\n\n", std::mem::size_of::<Box<[i32; 1000]>>()); // still 8
    dbg!(std::any::type_name::<i32>());
    dbg!(std::any::type_name::<Box<i32>>());
}

// we can construct cons lists made up of recursive pairs.
enum _List {
    // Cons(i32, _List), // without box, infinite size, doesnt know size
    Cons(i32, Box<_List>), // Getting a Recursive Type with a Known Size
    Nil,
}

// indirection means that instead of storing a value directly,
//  we should change the data structure to store the value indirectly by storing a pointer to the value instead.

// The Box<T> type is a smart pointer because it implements the *Deref* trait,
//  which allows Box<T> values to be treated like references.
//
// When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up
//  as well because of the *Drop* trait implementation
