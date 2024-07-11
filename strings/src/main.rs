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

    // first_word(&s);

    let s1 = String::from("Prueba ");
    let s2 = "de String".to_string();
    let s3 = "(de nuevo)".to_string();

    // let s3 = s1 + &s2;

    let s = format!("{s1}{s2} {s3}");
    println!("{s}");

    let chars = s.chars();

    for c in chars {
        // println!("{c}");
    }
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("Indice: {}, {}", i, item);
    }

    

    &s[..]
}
