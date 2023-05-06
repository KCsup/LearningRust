mod front_of_house {

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

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    // same applies for local path
    front_of_house::hosting::add_to_waitlist();
}
