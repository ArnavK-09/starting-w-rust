fn main() {
    println!("Hello, world!");
    // Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.

    another_function("Hello, World!");

    // express func
    let y = {
        let x = 3;
        x + 1 //  Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
    };
    // x scope
    println!("The value of y is: {y}");
}

/// Using fn to define func
fn another_function(msg: &str) {
    let a = five();
    println!("{msg} from Another function {a}");
    // main(); // LOL
}

/// returning values
fn five() -> u8 {
    5
}
