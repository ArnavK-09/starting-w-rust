use std::env;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Unit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Celsius => write!(f, "Celsius"),
            Unit::Fahrenheit => write!(f, "Fahrenheit"),
            Unit::Kelvin => write!(f, "Kelvin"),
        }
    }
}

fn parse_unit(unit: char) -> Option<Unit> {
    match unit {
        'c' | 'C' => Some(Unit::Celsius),
        'f' | 'F' => Some(Unit::Fahrenheit),
        'k' | 'K' => Some(Unit::Kelvin),
        _ => None,
    }
}

const COMMANDS: [&str; 2] = ["help", "convert"];

fn main() {
    // Collect args into a Vec, then skip the first element (the program path)
    let args: Vec<String> = env::args().skip(1).collect();
    let help_message = format!(
        "
        Beginneer rust project!
        This cli is made to convert temperatures between different units.

        Available commands:- {:?}
    ",
        COMMANDS
    );

    if args.is_empty() {
        println!("No arguments provided");
        return;
    }
    let command = &args[0];

    if !COMMANDS.contains(&command.as_str()) {
        println!("Invalid commands, available commands:- {:?}", COMMANDS);
        return;
    }

    if command == "help" {
        println!("{}", help_message);
        return;
    }

    if args.len() < 2 {
        println!("Invalid number of arguments");
        return;
    }
    if command == "convert" {
        println!();
        if args.len() < 3 {
            println!("Usage: convert <temperature> <target_unit>");
            println!("Example: convert 100C F");
            return;
        }
        let temperature = &args[1];

        let last_chr = match temperature.chars().last() {
            Some(a) => a,
            None => {
                println!("Empty temperature value");
                return;
            }
        };
        let temperature = &temperature[..temperature.len() - 1];
        let temperature_unit = match parse_unit(last_chr) {
            Some(a) => a,
            None => {
                println!(
                    "Invalid temperature unit, available units:- {:?}",
                    [Unit::Celsius, Unit::Fahrenheit, Unit::Kelvin]
                );
                return;
            }
        };

        let unit = match parse_unit(args[2].chars().next().unwrap()) {
            Some(a) => a,
            None => {
                println!(
                    "Invalid target temperature unit, available units:- {:?}",
                    [Unit::Celsius, Unit::Fahrenheit, Unit::Kelvin]
                );
                return;
            }
        };

        if unit == temperature_unit {
            println!(
                "Invalid target unit, target unit must be different from the temperature unit"
            );
            return;
        }

        println!(
            "Converting temperatures from {} to {}",
            temperature_unit, unit
        );

        let temp = temperature.parse::<f64>();
        if temp.is_err() {
            println!("Invalid temperature value");
            return;
        }
        let temp = temp.unwrap();

        let result = match (temperature_unit, unit) {
            (Unit::Celsius, Unit::Kelvin) => temp + 273.15,
            (Unit::Celsius, Unit::Fahrenheit) => (temp * (9.0 / 5.0)) + 32.0,

            (Unit::Kelvin, Unit::Celsius) => temp - 273.15,
            (Unit::Kelvin, Unit::Fahrenheit) => ((temp - 273.15) * (9.0 / 5.0)) + 32.0,

            (Unit::Fahrenheit, Unit::Kelvin) => ((temp - 32.0) * (5.0 / 9.0)) + 273.15,
            (Unit::Fahrenheit, Unit::Celsius) => (temp - 32.0) * (5.0 / 9.0),
            _ => unreachable!(),
        };

        println!("Converted Temperature:- {:?} {}", result, unit);
    }
}
