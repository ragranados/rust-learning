#![allow(unused)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = (30, 50);

    //definition of the Rectangle structure
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // println!("El area es: {}", area(&rect));

    // println!("Dimensiones:\n{rect:#?}");

    println!("EL area es: {}", rect.area());

    println!("Prueba: {}", rect.width);

    println!("Can hold: {}", rect.can_hold(&rect2));
    println!("Can hold: {}", rect.can_hold(&rect3));

    let rect4 = Rectangle::square(5);
    println!("Dimensiones: {rect4:#?}",);
}

//function to calcule the area of a reactangle
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area_tuples(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}
