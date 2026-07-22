fn main() {
    // Vectors can only store values that are of the same type.
    // Note that we added a type annotation here.
    // Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store.
    let _new_vector: Vec<i32> = Vec::new();

    // Rust will infer the type of value you want to store
    let v = vec![1, 2, 3, 4]; // Vec<i32>
    let _v2 = vec!["wow", "'c"]; // Vec<&str>
    let _v3 = ["rust"; 19];

    let mut updatable_vector = vec![1, 2];

    updatable_vector.push(3);
    updatable_vector.push(4);

    // Reading data
    let third = &updatable_vector[3]; // starts from 0
    println!("The third element is {third}");
    // Using & and [] gives us a reference to the element at the index value.

    // Safely
    let fifth: Option<&i32> = updatable_vector.get(5); // Option<&T>
    match fifth {
        Some(fifth) => println!("The fifth element is {fifth}"),
        None => println!("There is no fifth element."),
    };

    // When the get method is passed an index that is outside the vector, it returns None without panicking
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist);

    // the borrow checker enforces the ownership and borrowing

    let mut v2 = vec![1, 2, 3, 4, 5];
    let first = &v2[0];
    v2.push(6);
    // println!("The first element is: {first}"); // Error because first is borrowed

    // Iterating Over the Values in a Vector
    for i in &v {
        println!("{i}");
    }

    // We can also iterate over mutable references to each element in a mutable vector
    // in order to make changes to all the elements.
    for i in &mut updatable_vector {
        *i *= 50; // we have to use the * dereference
        println!("{i}");
    }

    // Using an Enum to Store Multiple Types
    // definitely use cases for needing to store a list of items of different types.
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        println!("{:#?}", i);
    }

    // Dropping a Vector Drops Its Elements
    {
        let v2 = vec![1, 2, 3, 4];
        // do stuff with v2
    }
    // <- v2 goes out of scope and is freed here
    // a vector is freed when it goes out of scope
    // all of its contents are also dropped
}
