pub mod front_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod back_of_house {
    pub enum _Appetizer {
        Soup,
        Salad,
    }

    pub struct _Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl _Breakfast {
        pub fn _summer(toast: &str) -> _Breakfast {
            _Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }
}
