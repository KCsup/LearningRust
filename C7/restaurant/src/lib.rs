mod front_of_house {

    pub struct Breakfast {
        pub toast: String,
        fruit: String,
    }

    impl Breakfast {
        pub fn certain_season(toast: &str) -> Self {
            // Associated function; No '&self' as param

            // Needs associated function for struct instance since one of the
            // values is private

            Breakfast {
                toast: toast.to_string(),
                fruit: String::from("SomeFruit :)"),
            }
        }
    }

    pub enum Appetizer {
        // All enum variants are public when the parent is set so
        Soup,
        Salad,
    }

    fn get_front_name() {}

    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}

        fn get_name() {
            super::get_front_name()
        }
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    // same applies for local path
    front_of_house::hosting::add_to_waitlist();

    // Or with the use scope
    hosting::add_to_waitlist();

    // Order

    let mut meal = front_of_house::Breakfast::certain_season("Wheat");

    meal.toast = "OtherToastType".to_string();

    println!("Now getting {}, with some fruit", meal.toast);
    // Cannot access 'meal.fruit' since it's not public

    // Appetizers :)

    let app_one = front_of_house::Appetizer::Soup;
    let app_two = front_of_house::Appetizer::Salad;
}
