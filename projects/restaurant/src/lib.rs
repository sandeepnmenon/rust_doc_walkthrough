mod front_of_house;

// bringing crate::front_of_house::hosting into scope
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();
}

// module tree structure for this crate
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

// The module tree should be defined in src/lib.rs.
//Then, any public items can be used in the binary crate by starting paths with the name of the package.
//The binary crate becomes a user of the library crate just like a completely external crate would use the library crate:
//it can only use the public API. This helps you design a good API; not only are you the author, you’re also a client!

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

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

    // If enum is public, all variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_2() {
    let mut meal = back_of_house::Breakfast::summer(("Rye"));
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    // Below line wont compile as it is accessing a private attribute
    // meal.seasonal_fruit = String::from("blueberries");

    // If enum is public, all variants are public
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

pub fn eat_at_restaurant_3() {
    hosting::add_to_waitlist();
}
