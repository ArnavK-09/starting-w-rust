#[allow(unused_variables)]
fn main() {
    let str1 = String::from("hello");
    let len = calculate_length(&str1); // &str1 syntax lets us create a reference that refers to the value of s1 but does not own it
    println!("The length of '{}' is {}.", str1, len);

    // Mutable References
    let mut str2 = String::from("hello");
    fix_string(&mut str2);
    println!("str2 is {}", str2);

    // if you have a mutable reference to a value, you can have no other references to that value.
    let s1 = &mut str2;
    let s2 = &mut str2;
    // we cannot borrow str2 as mutable more than once at a time.
    // println!("{s1} {s2}"); // cannot use them
}

/// that has a reference to an object as a parameter instead of taking ownership of the value
fn calculate_length(str: &String) -> usize {
    str.len()
}

fn fix_string(some_string: &mut String) {
    some_string.push_str(", world");
}
