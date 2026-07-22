#[allow(unused_parens)]
fn main() {
    if true {
        println!("Hello, world!");
    } else {
        println!("Bye, world!")
    }

    // no truthly false values
    if 3 == 0 {
        println!("Number is zero")
    } else if 3 != 0 {
        println!("number is not zero")
    } else {
        println!("idk man")
    }

    // Using if in a let Statement
    let x = if true { 5 } else { 6 };
    // let x = if condition { 5 } else { "six" }; !! RETURN TYPE SHOULD BE SAME IN LADDER
    println!("{x}");

    // loops
    // loop {}

    let mut y = 0;
    let result = loop {
        y = y + 1;
        if (y == 5) {
            // using brackets like js
            break y;
        };
    };
    println!("result is {:?}", result);

    // loop labels
    'tooo: loop {
        println!("loop label");
        // continue 'tooo;
        break 'tooo;
    }

    // conditional loops
    while (y != 0) {
        println!("y:- {y}!");
        y -= 1;
    }

    // CAN'T
    // while let mut i = 0; i != 0 {
    // println!("i is {}", i);
    // }\

    // rev = reverse arrray
    for i in (0..=5).rev() {
        // 0,1,2,3,4, 5
        println!("i is {}", i);
    }
}
