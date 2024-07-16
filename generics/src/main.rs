use std::cmp::PartialOrd;
use std::io::prelude::*;

struct Point<T> {
    x: T,
    y: T,
}

fn largest<T: PartialOrd>(list: &[T]) -> &T{

    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}