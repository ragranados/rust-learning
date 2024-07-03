#![allow(unused)]
fn main() {
    // We can be explicit with type
    let mut i32_vec = Vec::<i32>::new(); // turbofish <3

    // But look how clever Rust is about determining the type automatically
    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);

    // That's a beautiful macro!
    let string_vec = vec![String::from("Hello"), String::from("World")];

    for word in string_vec.iter() {
        println!("{}", word);
    }

    let signed_unsigned: u32 = 32;
    println!("Numero: {signed_unsigned}");

    let x: u8 = 255;
    let y: Option<u8> = x.checked_add(7);
    println!("{:?}", y); // Output: None

    if y.is_none() {
        println!("El resultado da overflow");
    } else {
        println!("No es overflow");
    }

    let x: u8 = 100;
    let y: Option<u8> = x.checked_add(50);
    println!("{:?}", y); // Output: Some(150)

    //Prueba!
}
