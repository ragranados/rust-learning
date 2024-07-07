fn main() {
    let s1 = String::from("Nueva String");

    let mut s2 = String::from("Otra nueva string");

    let r0 = &mut s2;

    r0.push_str(" Push 0");

    println!("{r0}");

    // let length = calculate_length(&s1);

    // println!("Length: {length}")

    // change(s1);

    // change(&mut s2);

    let r1 = &mut s2;

    // let r2 = &mut s2;

    // r2.push_str(" Segundo push");

    println!("{s2}");
}

fn prueba_referencia () {
    let mut s = String::from("Cosa");

    
}

fn change(some_string: &mut String) {
    some_string.push_str(", equis de");
}

// fn change(some_string: &String) {
//     some_string.push_str(", world"); //este codigo no funciona
// }

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn print(s: &mut String) {
    println!("{s}");
}
