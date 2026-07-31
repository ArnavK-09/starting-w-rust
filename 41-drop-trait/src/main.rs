// Running Code on Cleanup with the Drop Trait
//
// which lets you customize what happens when a value is about to go out of scope.
//  You can provide an implementation for the Drop trait on any type, and that code can be used to release resources like files or network connections.

struct MySmartPointer {
    data: String,
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("Dropping MySmartPointer with data `{}`!", self.data);
        // Variables are dropped in the reverse order of their creation
    }
}

#[allow(unused_variables)]
fn main() {
    let a = MySmartPointer {
        data: String::from("a"),
    };
    let b = MySmartPointer {
        data: String::from("b"),
    };

    // b.drop();
    // Rust doesn’t let you call the Drop trait’s drop method manually; instead,
    // you have to call the std::mem::drop function provided by the standard library if you want to force a value to be dropped before the end of its scope.

    println!("\n\nPointers created");
    std::mem::drop(b);
    println!("MySmartPointer dropped before the end of main");
}

// destructor, general programming term for a function that cleans up an instance.
//  A destructor is analogous to a constructor, which creates an instance.
