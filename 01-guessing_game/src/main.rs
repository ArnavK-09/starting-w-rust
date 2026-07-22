// standard i/o
use rand::Rng;
use std::cmp::Ordering;
use std::io; // The Rng trait defines methods that random number generators implement

fn main() {
    // welcome msg
    println!(
        "
    █▀▀ █░█ █▀▀ █▀ █▀   █▀▀ ▄▀█ █▀▄▀█ █▀▀
    █▄█ █▄█ ██▄ ▄█ ▄█   █▄█ █▀█ █░▀░█ ██▄
    "
    );

    loop {
        println!("\n\nGuess the number:- ");

        // variable to store user guess || new is an associated function of the String type.
        let mut guess_by_user = String::new(); // empty string

        // ask user for a number
        io::stdin()
            .read_line(&mut guess_by_user)
            .expect("No number found");

        // parsing string to number 
        let guess_by_user: u32 = match guess_by_user.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        // making target number
        let target_number = rand::thread_rng().gen_range(1..=101);
        println!("\nYour Target number was: {}", target_number);

        // matching values
        match guess_by_user.cmp(&target_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Equal => {
                println!("POG! You guessed it!\n");
                break;
            }
            Ordering::Greater => println!("Too big!\n"),
        }
    }
}
