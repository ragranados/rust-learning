mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let mut meal = front_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Francesa");

    println!("Desayuno: {}", meal.toast);

    hosting::add_to_waitlist();

    // //absoluta
    // crate::front_of_house::hosting::add_to_waitlist();

    // //relativa
    // front_of_house::hosting::add_to_waitlist();
}
