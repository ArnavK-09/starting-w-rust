#[allow(unused_assignments)]
fn main() {
    println!("Hello, world!");

    {
        let s = "in scope";
        println!("{s}")
    }

    let s = "out scope";
    dbg!(s);

    let immutable_string = "hello, world!";
    let mut mutable_string = "hello";
    let empty_str = String::new();
    // immutable_string = "hi"; nope
    mutable_string = "hello, world!";
    // let empty_str2 = empty_str; cannot because vlue borowed
    let mut empty_str2 = empty_str.clone();
    println!("{immutable_string} | {mutable_string} | {empty_str} | {empty_str2}");

    empty_str2 = "HEllo empty".to_string();
    let owned_empty_str = takes_and_return_ownership(empty_str);
    println!("{owned_empty_str}");
    takes_ownership(empty_str2);
    // println!("{empty_str2}"); 
}

fn takes_ownership(some_string: String) {
    println!("taken ownership of: {}", some_string);
}

fn takes_and_return_ownership(some_string: String) -> String {
    println!("taken and returned ownership of: {}", some_string);
    some_string
}
