// Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}
// There are more differences between functions and closures.
// Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do.
// Type annotations are required on functions because the types are part of an explicit interface exposed to your users.
// Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns.
// Closures, on the other hand, aren’t used in an exposed interface like this:
// They’re stored in variables, and they’re used without naming them and exposing them to users of our library.

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // a closure
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let _expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        num
    };

    // Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter:
    // borrowing immutably, borrowing mutably, and taking ownership.

    let list = vec![1, 2, 3];
    println!("\n\nBefore defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // closure now captures a mutable reference.

    let mut list2 = vec![1, 2, 3];

    println!("\n\nBefore defining closure: {list2:?}");

    let mut borrows_mutably = || list2.push(7);

    // cannot borrow `list2` as immutable because it is also borrowed as mutable
    // println!("Before calling closure: {list2:?}");

    borrows_mutably();
    borrows_mutably();
    println!("After calling closure: {list2:?}");
    // When borrows_mutably is defined, it captures a mutable reference to list.
    // We don’t use the closure again after the closure is called, so the mutable borrow ends.
    // Between the closure definition and the closure call, an immutable borrow to print isn’t allowed,
    //  because no other borrows are allowed when there’s a mutable borrow

    // If you want to force the closure to take ownership of the values it uses in the environment even though the body of the closure doesn't strictly need ownership,
    // you can use the move keyword before the parameter list.

    let list3 = vec![1, 2, 3];
    println!("\n\nBefore defining closure: {list3:?}");

    // We spawn a new thread, giving the thread a closure to run as an argument.
    // The closure only captures list using an immutable reference (least access needed to print).
    // However, we must use `move` to transfer ownership of list into the closure.
    // This ensures the closure owns the data, so its reference remains valid even if the main
    // thread finishes before the spawned thread
    std::thread::spawn(move || println!("From thread: {list3:?}"))
        .join()
        .unwrap();

    // A closure body can do any of the following:
    // Move a captured value out of the closure, mutate the captured value, neither move nor mutate the value, or capture nothing from the environment to begin with.

    // The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use
    //
    // Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:
    //
    // - FnOnce; applies to closures that can be called once. All closures implement at least this trait because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits because it can only be called once.
    // - FnMut; applies to closures that don’t move captured values out of their body but might mutate the captured values. These closures can be called more than once.
    // - Fn; applies to closures that don't move captured values out of their body and don't mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

    // FnOnce: moves a value out, can only be called ONCE
    println!();
    let name = String::from("Alice");
    let greet_once = || {
        let moved_name = name; // ownership moves OUT of the closure
        println!("Hello, {moved_name}!");
    };
    greet_once();
    // greet_once(); // ERROR: cannot use closure again — name is gone

    // FnMut: mutates captured values, can be called MANY times
    println!();
    let mut counter = 0;
    let mut count = || {
        counter += 1; // mutates the captured variable
        println!("Counter is now: {counter}");
    };
    count();
    count();
    count();
    println!("FnMut demo: closure ran 3 times, counter = {counter}");

    // Fn: only reads captured values, can be called MANY times, safe for concurrency
    println!();
    let message = String::from("Rust is fun");
    let print_msg = || {
        println!("Message: {message}"); // only borrows immutably
    };
    print_msg();
    print_msg();
    print_msg();
    println!("Fn demo: closure ran 3 times, message is still available: {message}");
}
