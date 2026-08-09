#[allow(dead_code)]
#[allow(unused)]
fn main() {
    //  Matching Literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Named Variables
    // Named variables are irrefutable patterns that match any value
    let x = Some(15);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // will match any value inside a Some value.
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    // Matching Multiple Patterns
    // you can match multiple patterns using the | syntax,
    //  which is the pattern or operator.
    match y {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Ranges of Values with ..=
    // you can use the ..= syntax to match a range of values.
    match y {
        1..=3 => println!("one through three"),
        _ => println!("anything2"),
    }
    //  ranges are only allowed with numeric or char values.
    let y = 'f';
    match y {
        'a'..='z' => println!("a through z"),
        _ => println!("anything3"),
    }

    // Destructuring to Break Apart Values

    // Structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // We can also destructure with literal values as part of the struct pattern
    // rather than creating variables for all the field

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(color) => match color {
            Color::Rgb(r, g, b) => println!("Change color to red {r}, green {g}, and blue {b}"),
            Color::Hsv(h, s, v) => println!("Change color to hue {h}, saturation {s}, value {v}"),
            _ => (),
        },
    }

    // Nested Structs and Enums
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }

    // Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Ignoring Values in a Pattern
    // There are a few ways to ignore entire values or parts of values in a pattern:
    // - using the _ pattern,
    // - using the _ pattern within another pattern,
    // - using a name that starts with an underscore,
    // - using .. to ignore remaining parts of a value.

    // An Entire Value with _
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {y}");
    }

    // Parts of a Value with a Nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");
    // We can also use underscores in multiple places within one pattern to ignore particular values
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    // An Unused Variable by Starting Its Name with _
    let _x = 5;

    // The syntax _x still binds the value to the variable, whereas _ doesn’t bind at all.
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    // println!("{s:?}"); // err: s value will still be moved into _s

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }
    println!("{s:?}");

    // Remaining Parts of a Value with ..
    // .. pattern ignores any parts of a value that we haven’t explicitly matched in the rest of the pattern
    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x, .. } => println!("x is {x}"),
    }
    // The syntax .. will expand to as many values as it needs to be.
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // Adding Conditionals with Match Guards
    // A match guard is an additional if condition, specified after the pattern in a match arm,
    // that must also match for that arm to be chosen.
    // they are only available in match expressions, not if let or while let expressions.

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
    // The downside of this additional expressiveness is that the
    // compiler doesn’t try to check for exhaustiveness when match guard expressions are involved.
    let x = 4;
    let y = false;

    // You can also use the or operator | in a match guard to specify multiple patterns
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // Using @ Bindings
    // The at operator @ lets us create a variable that
    // holds a value at the same time we’re testing that value for a pattern match.

    enum Message2 {
        Hello { id: i32 },
    }

    let msg = Message2::Hello { id: 5 };
    // We also want to bind the value to the variable id so that we can use it in the code associated with the arm.\
    match msg {
        Message2::Hello { id: id @ 3..7 } => {
            println!("Found an id in range: {id}")
        }
        // where we only have a range specified in the pattern,
        // the code associated with the arm doesn’t have a variable that contains the actual value of the id field
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message2::Hello { id } => println!("Found some other id: {id}"),
    }

    // Using @ lets us test a value and save it in a variable within one pattern.
}
