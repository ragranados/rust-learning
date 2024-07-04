use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Advina le numero!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Por favor ingresa un numero: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("Tu numero es: {guess}");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

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
