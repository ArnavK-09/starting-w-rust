#![allow(dead_code)]
#[derive(Debug)]
enum Oauth {
    Github,
    Google,
    Facebook, // capitlal first letter
}

impl Oauth {
    fn login_in(&self) {
        println!("Logging in using, {:?}", &self);
    }
}
#[derive(Debug)]
struct custom_id {}

// using custom values
#[derive(Debug)]
enum ID {
    Nanoid(String),
    Uuid(u32),
    Coords(u8, u8, u32), // using tuples,
    Custom(custom_id),   // using structs
}

// with structs
struct User {
    logged_in_using: Oauth,
    id: ID,
}

impl User {
    fn info(&self) {
        dbg!(&self.logged_in_using);
        dbg!(&self.id);
    }
}

fn main() {
    println!("Hello, world!");

    let login: Oauth = Oauth::Google;

    let p = User {
        id: ID::Coords(121, 255, 6565),
        logged_in_using: login,
    };

    p.info();

    // enum funcs
    p.logged_in_using.login_in();

    // built in option enum
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    dbg!(some_char, absent_number);
}
