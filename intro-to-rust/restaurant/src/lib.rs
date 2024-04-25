// still need the absolute path
// even though it was shipped
use std::collections::HashMap;

// nested path for saving space
use std::{cmp::Ordering, io};

// nested path with subpaths
use std::arch::{self, asm};

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    // the struct is pub, but its fields are not
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

    // unlike structs, every type is now pub
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub mod global_now {
        pub fn global_fun() {}
    }
}

// changes how the call will be structured in the mod
pub use crate::back_of_house::global_now::global_fun;
use crate::front_of_house::hosting::seat_at_table;

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // seasonal_fruit is NOT pub
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    seat_at_table(); // woah that's crazy
}

mod customer {
    pub fn eat_at_restaurant() {}

    // doesn't work because mod is a different scope
    // seat_at_table();

    // so we can use:
    use crate::front_of_house::hosting::seat_at_table;

    fn now_try() {
        seat_at_table();
    }
}

mod alt_customer {
    use crate::global_fun;

    pub fn eat_at_restaurant() {}

    fn now_try() {
        super::front_of_house::hosting::seat_at_table();
        global_fun();
    }
}

// NOTE: you should avoid going all the way down, since it's
// hard to differentiate which ones are locally defined

mod extra_stuff;
