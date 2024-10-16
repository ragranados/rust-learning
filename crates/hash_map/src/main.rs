use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // scores.insert(String::from("Blue"), 11);

    let team_name = String::from("Blue");

    //se pasa referencia, copied para que devuelva Option i32 y no con &, y unwarop para tener valor o 0 si es None
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");

    imprimir_hash_map(&scores);

    scores.entry(String::from("Hola")).or_insert(45);

    scores.entry(String::from("Blue")).or_insert(70);

    println!("--------------------");

    imprimir_hash_map(&scores);

    println!("--------------------");

    let contadas = count_words(&String::from("XD Hola esta es una prueba usando Hola XD"));

    imprimir_hash_map(&contadas);
}

fn count_words(frase: &str) -> HashMap<String, i32> {
    let mut mapa: HashMap<String, i32> = HashMap::new();

    for palabra in frase.split_whitespace() {
        let contador = mapa.entry(String::from(palabra)).or_insert(0);
        *contador += 1;
    }

    mapa
}

fn imprimir_hash_map(mapa: &HashMap<String, i32>) {
    for i in mapa {
        println!("{}\t{}", i.0, i.1);
    }
}
