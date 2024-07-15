use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(_error) => 
        match _error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("No se pudo crear el archivo"),
            },
            other_error => {
                panic!("Otro tipo de error: {other_error:?}")
            }
        },
    };
}
