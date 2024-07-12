#![allow(unused)]

use std::collections::HashMap;

fn main() {
    // first_exercise();

}

//Given a list of integers, use a vector and return the median (when sorted, the value in the
//middle position) and mode (the value that occurs most often; a hash map will be helpful here)
//of the list.

fn first_exercise() {
    let mut lista: Vec<i32> = vec![
        1,2,3,4,5,6,12,12,3,56,12
    ];

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
