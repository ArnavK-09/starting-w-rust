// Unsafe Rust
// Safe Rust enforces memory safety at compile time
// Unsafe Rust skips these checks, giving extra powers (and risks)

// use the unsafe keyword and then start a new block that holds the unsafe code

#![allow(unused)]

// You can take five actions in unsafe Rust that you can’t in safe Rust, which we call unsafe superpowers:
//
// - Dereference a raw pointer.
// - Call an unsafe function or method.
// - Access or modify a mutable static variable.
// - Implement an unsafe trait.
// - Access fields of unions.

// unsafe doesn't disable the borrow checker
// It only unlocks 5 unchecked features:
// All other Rust safety rules still apply

// In Rust, global variables are called static variables.
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    // Dereferencing a Raw Pointer
    // Unsafe Rust has two new types called raw pointers that are similar to references
    //
    // Two types: *const T (immutable) and *mut T (mutable).
    // The asterisk is part of the type name, not the dereference operator.
    // Immutable means the pointer can't be assigned to after dereferencing.

    // Different from references and smart pointers, raw pointers:
    //
    // - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    // - Aren’t guaranteed to point to valid memory
    // - Are allowed to be null
    // - Don’t implement any automatic cleanup

    let mut num = 5;

    // we don’t include the unsafe keyword in this code. We can create raw pointers in safe code
    let r1 = &raw const num;
    let r2 = &raw mut num;
    // we just can’t dereference raw pointers outside an unsafe block

    // keyword as to cast a value instead of using the raw borrow operator

    // create a raw pointer to an arbitrary location in memory
    let address = 0x12344usize;
    let r = address as *const i32;
    println!("r points to: {:p}", r);

    unsafe {
        // Raw pointers allow *const i32 and *mut i32 to point to the same location.
        // References can't: the borrow checker forbids mixed immutable/mutable borrows.
        // Writing through the mutable pointer while both exist can cause a data race.

        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        // println!("r is: {}", *r); // err: Address boundary error)
    }

    // Creating a pointer does no harm
    // it’s only when we try to access the value that it points
    // at that we might end up dealing with an invalid value.

    // Calling an Unsafe Function or Method

    unsafe fn dangerous() {
        println!("\nunsafe function called\n")
    }

    // dangerous(); // err: call to unsafe function `dangerous` is unsafe and requires unsafe block

    unsafe {
        dangerous();
    }

    // Creating a Safe Abstraction over Unsafe Code

    // function contains unsafe code doesn’t mean we need to mark the entire function as unsafe.
    // wrapping unsafe code in a safe function is a common abstraction.

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    fn _split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();

        assert!(mid <= len);

        // Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice
        // it only knows that we’re borrowing from the same slice twice
        // (&mut values[..mid], &mut values[mid..]) // err: cannot borrow `*values` as mutable more than once at a time

        use std::slice;
        let ptr = values.as_mut_ptr(); // access the raw pointer of a slice
        unsafe {
            (
                // unsafe because it takes a raw pointer and must trust that this pointer is valid
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    // split_at_mut needs no unsafe marker; safe code can call it.
    // The unsafe inside is used safely: it creates only valid pointers
    // from data this function already has access to.
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Using extern Functions to Call External Code

    // extern creates a Foreign Function Interface (FFI).
    // Lets Rust functions be called by another programming language.
    // Or lets Rust call functions written in another language.

    unsafe extern "C" {
        fn abs(input: i32) -> i32;
        safe fn sin(x: f64) -> f64;
    }

    // extern "C" block lists names and signatures of foreign functions to call.
    // "C" is the ABI, which defines how to call at the assembly level.
    // The C ABI is the most common; more ABIs are in the Rust Reference.
    // Application binary interface (ABI)

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // some FFI functions are safe to call
    println!("sin(1.0) = {}", sin(90.0));

    // Calling Rust Functions from Other Languages

    #[unsafe(no_mangle)] // not to mangle the name of this function.
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
    // Mangling is when a compiler changes the name we’ve given a function to a different name that
    // contains more information for other parts of the compilation process to consume but is less human readable

    // Accessing or Modifying a Mutable Static Variable

    static HELLO_WORLD: &str = "Hello, world!2";
    println!("value is: {HELLO_WORLD}");

    // const vs static:
    // const values are duplicated on each use and have no fixed address.
    // static values have one fixed address; every access sees the same data.
    // statics may be mutable, consts may not.

    // Accessing and modifying mutable static variables is unsafe
    static mut COUNTER: u32 = 0;

    // Calling this from more than a single thread at a time is undefined
    /// behavior, so you *must* guarantee you only call it from a single thread at a time
    unsafe fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    unsafe {
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }

    // Implementing an Unsafe Trait
    // We can use unsafe to implement an unsafe trait
    // A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }

    // A union is similar to a struct, but only one declared field is used in a particular instance at one time
    union IntOrFloat {
        f: f32,
        i: u32,
    }
    // Initializing uses a struct-like syntax, one field only
    let u = IntOrFloat { f: 1.0 };

    // Size: NOT 8 bytes — just 4 (the largest field wins)
    println!("size: {} bytes", std::mem::size_of::<IntOrFloat>());

    unsafe {
        // Reading is only legal in danger-zone mode
        println!("read as f32: {}", u.f); // 1.0
        println!("read as u32: {}", u.i); // 1065353216 — what?!
    }

    // Using Miri to Check Unsafe Code
    // Miri: official tool for detecting undefined behavior at runtime.
    // Borrow checker = static, compile time; Miri = dynamic, runs your code.
    // Install: rustup +nightly component add miri
    // Run: cargo +nightly miri run or cargo +nightly miri test
    // Adds a tool only; your project's Rust version stays unchanged.
}
