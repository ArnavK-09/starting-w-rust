//  iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.
fn main() {
    let v = vec![1, 2, 3];
    let mut v_iter = v.iter();

    println!("{:?}\n", v_iter.next());

    // Also note that the values we get from the calls to next are immutable references to the values in the vector.
    // The iter method produces an iterator over immutable references
    //
    for i in v_iter {
        println!("{i}");
    }
    // .iter().enumerate() gives you **(index, value)** pairs:
    for (index, value) in v.iter().enumerate() {
        println!("{index}: {value}");
    }

    // Methods That Consume the Iterator

    // Methods that call next are called consuming adapters because calling them uses up the iterator.
    // One example is the sum method, which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator
    let v1_iter = v.iter();
    let total: i32 = v1_iter.sum();
    println!("\nsum: {total}");
    // cant coz consumed/moved
    // for i in v1_iter {
    //     println!("{i}");
    // }

    // Methods That Produce Other Iterators
    // Iterator adapters are methods defined on the Iterator trait that don’t consume the iterator.
    // Instead, they produce different iterators by changing some aspect of the original iterator.

    let m = v.iter().map(|x| x + 1);
    dbg!(m);

    // To fix this warning and consume the iterator, we’ll use the collect
    //  This method consumes the iterator and collects the resultant values into a collection data type.

    let m2: std::collections::HashMap<_, _> = v.iter().map(|x| (x, x + 1)).collect(); // _ -> placeholder/explicit
    dbg!(m2);
    // .collect()` turns an iterator into a `Vec`, `String`, `HashMap`, or other collection
}
