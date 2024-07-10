pub mod garden;

use crate::garden::vegetables::Asparagus;

fn main() {
    let plant = Asparagus {};
    println!("Planta: {plant:#?}");
}
