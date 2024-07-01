fn main() {
    
    //constantes, por convension, se declaran en mayuscula
    const PRUEBA: i32 = 60;

    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");

    //el shadowing permite declarar una variable mas de una vez y con el mismo nombre
    //esto es util (en mi opinion) cuando se trabaja con scopes diferentes
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}