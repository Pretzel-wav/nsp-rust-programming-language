// crate: lib.rs

mod front_of_house; // semicolon after `mod front_of_house` tells Rust to load the module.
                    // here, front_of_house is the name of the module (front_of_house.rs)
                    // this is a relative path to the current module.

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}