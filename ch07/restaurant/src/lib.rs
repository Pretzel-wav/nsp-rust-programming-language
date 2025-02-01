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

pub fn eat_at_restaurant() {
    // Absolute path; starts from crate root (lib.rs)
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path; starts at current level of module tree (crate)
    front_of_house::hosting::add_to_waitlist();
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