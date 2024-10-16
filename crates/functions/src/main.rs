#![allow(unused)]

fn another_function(value: i32, unit_label: char){
    println!("Funcion: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn main() {
    // another_function(5, 'h');
    let x = five();

    println!("Equis: {x}");
}
