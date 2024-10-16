use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};

fn main() {
    println!("Advina le numero!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Por favor ingresa un numero: ");

        let mut guess = String::new();

        // io::stdin()
        //     .read_line(&mut guess)
        //     .expect("Failed to read line");

        //mejor forma de manejar el error
        if let Err(_e) = io::stdin().read_line(&mut guess) {
            continue
        }

        println!("Tu numero es: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("No se ha ingresado un numero!");
                continue
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }

    }
}
