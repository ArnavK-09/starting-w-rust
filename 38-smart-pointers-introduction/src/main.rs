// Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the Deref and Drop traits.
//  The Deref trait allows an instance of the smart pointer struct to behave like a reference so that you can write your code to work with either references or smart pointers.
//  The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope. In this chapter, we’ll discuss both of these traits and demonstrate why they’re important to smart pointers.

// Most common smart pointers in the standard library:
//
//  Box<T>, for allocating values on the heap
//  Rc<T>, a reference counting type that enables multiple ownership
//  Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

fn main() {}
