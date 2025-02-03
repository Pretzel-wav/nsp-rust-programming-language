// crate: lib.rs

// module: front_of_house
mod front_of_house {
    // module: hosting
    pub mod hosting { // note that modules can be defined within a parent module
        pub fn add_to_waitlist() {} // adding pub to an item only lets its ancestors refer to it, not its contents.

        fn seat_at_table() {}

        fn eat_at_restaurant() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order () {}

        fn take_payment() {}
    }
}

mod back_of_house {
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
}

use crate::front_of_house::hosting;     // can also be relative
// pub use crate::front_of_house::hosting; // public declaration
use std::io::Result as IoResult;        // with alias
// use std::{io, cmp::Ordering};           // nested paths
// use std::io::{self, Write};             // nested paths with self

pub fn eat_at_restaurant() {
    // Absolute path; starts from crate root (lib.rs)
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path; starts at current level of module tree (crate)
    front_of_house::hosting::add_to_waitlist();

    // With path from `use` keyword
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); // error! `seasonal_fruit` is private
}

// Module Tree
// crate
// |----front_of_house
//      |----hosting
//      |    |----add_to_waitlist
//      |    |----seat_at_table
//      |----serving
//           |----take_order
//           |----serve_order
//           |----take_payment
