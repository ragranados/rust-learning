fn main() {
    // --- IF ---

    let number = 70;

    if number < 5 {
        // println!("Se cumplio la condicion");
    } else {
        // println!("No se cumplio");
    }

    // --- LOOPS ---

    loop_en_loop();

}

fn prueba_return() -> i32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            return counter * 2;
        }
    };

    return 0;

    // println!("The result is {result}");
}

fn loop_en_loop() {
    let mut count = 0;

    let x = 'loop_externo: loop {

        println!("Loop Externo: {count}");

        loop {
            println!("Loop Interno: {count}");
            count += 1;

            if count == 9 {
                println!("Saliendo...");
                break 'loop_externo;
            }
        }


    };
}
