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
        // println!("{}", word);
    }

    let signed_unsigned: u32 = 32;
    // println!("Numero: {signed_unsigned}");

    let x: u8 = 255;
    let y: Option<u8> = x.checked_add(7);
    // println!("{:?}", y); // Output: None

    if y.is_none() {
        // println!("El resultado da overflow");
    } else {
        // println!("No es overflow");
    }

    let x: u8 = 100;
    let z: u32 = 102;
    let y: Option<u8> = x.checked_add(50);
    // println!("{:?}", y); // Output: Some(150)

    let y = x.checked_sub(z as u8);

    if let Some(value) = y {
        // println!("Value: {}", value);
    } else {
        // println!("Overflow");
    }

    let remainder = 10 % 3;
    // println!("Valor: {}", remainder);

    let foo = Foo { x: 42 };

    let foo = do_something(foo);

    // println!("Regreso: {}", foo.x); //la variable foo ya no se puede utilizar.

}

fn prueba_indireccion() {
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f; // get a copy of the owner's value
    *f = 13;      // set the reference's owner's value
    println!("{}", bar);
    println!("{}", foo);
}

fn do_something(f: Foo) -> Foo{
    // println!("do_something: {}", f.x);

    f
}

struct Foo {
    x: i32,
}
