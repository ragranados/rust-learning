#![allow(unused)]

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    println!("Primer Valor (tup.1): {}", tup.1);

    //------------arrays------------------------

    let a = [1,2,3];

    let b: [i32; 5] = [1,2,3,4,5];

    let c = [3; 5];

    for i in c {
        println!("{i}");
    }
}