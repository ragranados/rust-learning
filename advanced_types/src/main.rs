fn main() {
    println!("Hello, world!");
    never_type();
}

fn never_type() -> ! {
    panic!("This function never returns!");
}
