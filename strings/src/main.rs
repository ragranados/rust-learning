fn main() {
    let mut s = String::from("Nueva");

    let mut s2 = s.clone();

    s.push_str(" String!");

    println!("{}", s);
    println!("{}", s2);
}
