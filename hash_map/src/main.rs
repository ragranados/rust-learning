use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    //se pasa referencia, copied para que devuelva Option i32 y no con &, y unwarop para tener valor o 0 si es None
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");

    // scores.entry(String::)
}
