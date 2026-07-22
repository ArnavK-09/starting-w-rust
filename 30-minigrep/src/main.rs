use minigrep::*;
use std::{env, error::Error, fs, process};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

fn main() {
    // Note that std::env::args will panic if any argument contains invalid Unicode. If your program needs to accept arguments containing invalid Unicode, use std::env::args_os instead.
    let args: Vec<String> = env::args().collect(); // collect method on an iterator to turn it into a collection, such as a vector, which contains all the elements the iterator produces.

    println!("\n");

    let config = Config::build(&args).unwrap_or_else(|err| {
        // The standard library provides the eprintln! macro that prints to the standard error stream
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let query = &config.query;
    let file_path = &config.file_path;
    println!("Searching for \"{query}\" in {file_path}\n\n");

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?; //std::io::Result<String>

    // ?` is the **error propagation operator**. It does two things:
    // 1. If `Result` is `Ok(value)` → unwraps and gives you the value
    // 2. If `Result` is `Err(e)` → **returns early** from the function with that error

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        // The env::var function returns a Result
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        return Ok(Config {
            query,
            file_path,
            ignore_case,
        });
    }
}
