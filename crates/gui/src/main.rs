use gui::{Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing Select Box...")
    }
}

fn main() {
    let select_box_one = SelectBox {
        width: 50,
        height: 50,
        options: vec!["Hola".to_string(), "Adios".to_string()],
    };

    let screen = Screen {
        components: vec![Box::new(select_box_one)],
    };

    screen.run();
}
