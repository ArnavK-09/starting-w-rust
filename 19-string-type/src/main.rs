fn main() {
    // UTF-8 encoded
    let mut string1 = String::new();
    let mut string2 = String::new();
    let string3 = String::from("hello");

    // which is available on any type that implements the Display trait
    let string4 = "hello".to_string();

    // A String can grow in size and its contents can change, just like the contents of a Vec<T>, if you push more data into it.
    // In addition, you can conveniently use the + operator or the format! macro to concatenate String values

    string1.push_str("hell");
    // The push method takes a single character as a parameter
    string1.push('o');
    println!("string1 is {string1}");

    // Concatenation
    let string5 = string2 + &string3; // fn add(self, s: &str) -> String {}
                                      // string2 unavaiable

    // other way
    let string6 = format!("{string4}-{string1}");

    // You already know that answer will not be З, the first letter. When encoded in UTF-8, the first byte of З is 208 and the second is 151, so it would seem that answer should in fact be 208, but 208 is not a valid character on its own
    let hello = "hello";
    // let answer = &hello[0];
    // The answer, then, is that to avoid returning an unexpected value and causing bugs that might not be discovered immediately,

    // Slicing Strings
    let s = &hello[0..4]; // by bytes
    println!("{s}");

    // Methods for Iterating Over Strings
    for c in "Зд".chars() {
        println!("{c}");
    }
    // for bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
