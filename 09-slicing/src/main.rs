fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello world");
    let res = first_word(&s);
    s.clear(); // empties string
    println!("Result:- {}", res);

    println!("\n");
  
    // slicing

    let s1 = String::from("hello world");
    let first_word_of_s1 = &s1[..5];
    let last_word_of_s1 = &s1[5..];

    println!("{first_word_of_s1} {last_word_of_s1}")
}

/// HECTIC
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("List of bytes:- {:?}", bytes);
    // enumerate wraps the result of iter and returns each element as part of a tuple instead
    for (i, &item) in bytes.iter().enumerate() {
        // iter is a method that returns each element in a collection
        println!("Item #{i}:- {}", item);
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
