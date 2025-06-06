#![allow(unused)]

use std::collections::HashMap;

fn main() {
    // first_exercise();
    second_exercise();
}

//Convert strings to pig latin. The first consonant of each word is moved to the end of the word
//and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
//to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8
//encoding!

fn second_exercise() {
    let mut string = String::from("apple");

    let mut string_chars = string.chars();
    let first_char = string_chars.next().unwrap_or('\0');

    if is_consonant(&first_char) {
        string = string_chars.collect();
        string.push_str(&format!("-{first_char}ay"))
    } else {
        string.push_str("-hay");
    };

    println!("{string}");
}

fn is_consonant(c: &char) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    c.is_alphabetic() && !vowels.contains(c)
}

//Given a list of integers, use a vector and return the median (when sorted, the value in the
//middle position) and mode (the value that occurs most often; a hash map will be helpful here)
//of the list.

fn first_exercise() {
    let mut lista: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 12, 12, 3, 56, 12];

    lista.sort();

    let media: f32 = obtener_media(&lista);

    println!("Median: {media}");

    let moda: i32 = obtener_moda(&lista);

    println!("Mode: {moda}");
}

fn obtener_media(lista: &Vec<i32>) -> f32 {
    let lista_len: i32 = lista.len() as i32;

    let index: usize = (lista_len / 2) as usize;

    if lista_len % 2 == 0 {
        (lista[index] + lista[index - 1]) as f32 / 2 as f32
    } else {
        lista[index] as f32
    }
}

fn obtener_moda(lista: &Vec<i32>) -> i32 {
    let mut contador: HashMap<i32, i32> = HashMap::new();

    for i in lista {
        let count = contador.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut numero_mayor: i32 = 0;
    let mut numero_de_veces = 0;

    for i in contador {
        if numero_de_veces <= i.1 {
            numero_mayor = i.0;
            numero_de_veces = i.1;
        }
    }

    numero_mayor
}
