#[allow(unconditional_panic)]
#[allow(unused_doc_comments)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
fn main() {
    // string to number without throwing exception
    let str_to_num: i32 = match "12q0".parse::<i32>() {
        Ok(num) => num,
        Err(err) => {
            println!("{}", err);
            0
        }
    };
    println!("{str_to_num}");

    // string to number with throwing exception
    let str_to_number: u32 = "120".parse().expect("enter a valid number");
    println!("{str_to_number}");

    ///
    /// A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters
    ///
    // Each signed variant can store numbers from -2^(n - 1) to 2^(n - 1) - 1
    let max_u8: u8 = 2_i32.pow(7) as u8;
    println!("max u8:- {} min u8: {}\nEg: - {}", u8::MAX, u8::MIN, max_u8);

    // Rust default number type is `i32`

    /// Rust’s floating-point types are f32 and f64
    ///  default is f64 , all floats are signed
    println!("{}", 232.667 as f32);

    // all manner of operations can be performed on numbers
    // https://doc.rust-lang.org/book/appendix-02-operators.html
    // let divide_by_zero= 1/0;
    // println!("{}", divide_by_zero); THROWS ERR

    ///  Booleans are one byte in size
    let xb: bool = true == false;
    println!("{xb}");

    /// char type is the language’s most primitive alphabetic type
    const EMOTE: char = '🧈'; // char literals with single quotes
    println!("{EMOTE}");
    // char type is four bytes in size and represents a Unicode Scalar Value

    /// tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    const COORDS: (u8, i32, f32) = (122, -232, 9.69);
    println!("Coordinates are:- {:?}", COORDS);

    // destructure
    let (x, y, z) = COORDS;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // asscessing  (.)
    println!("Value of z cord is: {}", COORDS.2); // index 0

    let UNIT = ();
    ///
    println!(
        "The tuple without any values has a special name, unit:-  {:?} ",
        UNIT
    );

    /// ARRAYS
    const months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("Months:- {:?}", months);
    /// let array: [<type>; <length>] = [...]

    // filled arrays
    println!("55 times '3': {:?}", ["3"; 55]);

    // item at index 36
    println!("12th mo: {}", months[11]); // index at 0
}
