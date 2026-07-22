// The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.

fn main() {
    let password: Option<&str> = Some("password"); // None
    match password {
        Some(pass) => println!("Password is there:- {}", pass),
        // None => println!("Password is not there"),
        _ => println!("Can't find password"),
    }

    // better with if else
    if let Some(pass) = password {
        println!("The password configured to:- {}", pass);
    } else {
        println!("Can't find password")
    }

    println!("{:#?}", password)
}
