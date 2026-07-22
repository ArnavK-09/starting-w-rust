fn main() {
    // immutable var
    let x = 2323;
    println!("x:- {x}");
    // muttable bar
    let mut y = 2322;
    // let y = y +1; (not twice)
    // y = "y" // we’re not allowed to mutate a variable’s type:
    y = y + 1;
    println!("y:- {y}");

    // const  - always immutable, includes global scope
    // constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
    const Z: u16 = 60 * 60 * 3;
    println!("const Z:- {Z}"); // uppercase const name

    // scope
    {
        let x = x * 1000; // inferred from out of scope x var
        println!("in-scope x value:- {x}")
    }
}
