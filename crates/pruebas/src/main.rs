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

    // let foo = do_something(foo);

    // println!("Regreso: {}", foo.x); //la variable foo ya no se puede utilizar.

    prueba_string();

}

fn prueba_string(){
    let mut s = String::from("Nueva");

    s.push_str(" String!");

    println!("{}", s);
}

fn prueba_indireccion() {
    let x = 5;
    let y = &x; // y is a reference to x

    println!("x: {}", x);
    println!("y: {}", y); // prints the reference to x
    println!("*y: {}", *y); // dereferences y to access the value of x
}

fn prueba_referencia_en_referencia() {
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x is dropped here allow us to create a non-mutable reference
    let y = do_something2(&foo);
    println!("{}", y);
    println!("x: {}", foo.x);
}

fn do_something2(a: &Foo) -> &i32 {
    return &a.x;
}

fn do_something(f: Foo) -> Foo{
    // println!("do_something: {}", f.x);

    f
}

struct Foo {
    x: i32,
}
