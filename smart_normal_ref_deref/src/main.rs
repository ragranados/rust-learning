use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 42;
    let y = 42;
    let z = &y;
    println!("{}", x == *z);

    let x = 42;
    let z = MyBox::new(x);
    println!("{}", x == *z);

    //----------------------------------------

    let m = MyBox::new(String::from("Hello"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
