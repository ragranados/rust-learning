mod front_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {

        fn take_orver() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    
    let mut meal = front_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Francesa");

    println!("Desayuno: {}",meal.toast);
    
    // //absoluta
    // crate::front_of_house::hosting::add_to_waitlist();

    // //relativa
    // front_of_house::hosting::add_to_waitlist();
}
