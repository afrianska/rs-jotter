//use std::fmt;
//use std::io;

// Providing New Names with the as Keyword
use std::fmt::Result;
use std::io::Result as IoResult;

// Using external packages
// add rand = "0.8.5" to Cargo.toml
use rand::Rng;

// Re-exporting Names with pub use
pub use crate::front_of_house::hosting; // put pub 

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// Grouping related code in modules
mod front_of_house {
    pub mod hosting {
        // pub for exposing paths
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorect_order() {
        cook_order();
        super::delivery_order(); // Starting relative with super.
    }

    fn cook_order() {}

    // Making Strucs Public
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Making Enums Public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// Bringing path into scope with the use keyword
//use crate::front_of_house::hosting;

// Creating Idiomatic use
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // Absolute path
    //crate::front_of_house::hosting::add_to_waitlist();
    // try bringing with use keyword
    //hosting::add_to_waitlist();
    // Creating idiomatic use: call
    add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Making Strucs Public: use
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Making Enums Public: use
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Re-exporting Names with pub use: call
    hosting::add_to_waitlist();

    // Using external packages: call
    let secret_number = rand::thread_rng().gen_range(1..=100);
}

fn delivery_order() {}

//fn function1() -> fmt::Result {}
//fn function2() -> io::Result<()> {}

//fn function1() -> Result {}
//fn function2() -> Result<()> {}
