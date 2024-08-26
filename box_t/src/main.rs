enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

    let prueba = Box::new(Some(3));
}
