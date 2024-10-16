use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    //Rc::clone only increments the reference count, which doesn’t take much time.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // let z = String::from("Prueba");

    let z_rc = Rc::new(String::from("Prueba"));

    let z_rc2 = Rc::clone(&z_rc);
    let z_rc3 = Rc::clone(&z_rc);

    println!("z_rc: {}", Rc::strong_count(&z_rc));
}
