use std::collections::HashMap;

fn main() {
    // all of the keys must have the same type, and
    // all of the values must have the same type
    let mut scores = HashMap::new();

    scores.insert(String::from("user"), 20);

    dbg!(&scores);

    println!("{:#?}", scores.get("non existent")); // None

    // Overwriting a Value
    scores.insert(String::from("user"), 40);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Check if entry there
    dbg!(scores.entry(String::from("Yellow"))); // VacantEntry
    dbg!(scores.entry(String::from("user"))); // OccupiedEntry

    scores.entry(String::from("lol")).or_insert(100);

    dbg!(&scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        dbg!(&word);
        *count = &*count + 1;
        // The or_insert method returns a mutable reference (&mut V) to the value for the specified key.
        // Here, we store that mutable reference in the count variable, so in order to assign to that value,
        // we must first dereference count using the asterisk (*).

        // or 
        *count += 1;
    }

    println!("{map:?}");
}
