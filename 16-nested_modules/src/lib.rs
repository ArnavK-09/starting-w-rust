#![allow(dead_code)]


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // meal.seasonal_fruit = String::from("blueberries");
}

fn deliver_order() {}

mod back_of_house {

    // if we make an enum public, all of its variants are then public.
    pub enum Drink {
        Coke,
        Fanta,
        Sprite,
    }

    fn fix_incorrect_order() {
        cook_order();
        // Using super allows us to reference an item that we know is in the parent module
        super::deliver_order();
    }

    fn cook_order() {}

    // If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private.
    pub struct Breakfast {
        // we can write and read to the toast field using dot notation
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // public associated function that constructs an instance of Breakfast
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
        // If Breakfast didn’t have such a function, we couldn’t create an instance of Breakfast in eat_at_restaurant because we couldn’t set the value of the private seasonal_fruit field in eat_at_restaurant
    }
}
