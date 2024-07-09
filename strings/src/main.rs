#![allow(unused)]
fn main() {
    let a: &'static str = "Hola";
    let a = "Hola 🦀".chars().collect::<Vec<char>>();
    println!("{}", a.len());

    let mut s = String::from("Nueva");

    // let mut s2 = s.clone();

    // s.push_str(" String!");

    // println!("{}", s);
    // println!("{}", s2);

    first_word(&s);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("Indice: {}, {}", i, item);
    }

    &s[..]
}
