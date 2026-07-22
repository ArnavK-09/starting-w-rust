// When a function’s implementation calls something that might fail,
// instead of handling the error within the function itself you can
// return the error to the calling code so that it can decide what to do.
// This is known as propagating the error

use std::fs::File;
use std::io::{self, Read, Error};

fn read_username_from_file() -> Result<String,Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => return Err(e),
    }
}

// A Shortcut for Propagating Errors: the ? Operator
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // ? operator
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
// The ? placed after a Result value is
// defined to work in almost the same way as the match expressions we defined to handle the Result

fn main() {
    let a = read_username_from_file2();
    dbg!(a);
    let b = read_username_from_file2();
    dbg!(b);
}
