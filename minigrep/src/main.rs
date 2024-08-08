use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problema: {err}");
        process::exit(1);
    });

    println!("Buscando {} en {}", config.query, config.file_path);

    let run_result = minigrep::run(config);

    if let Err(e) = run_result {
        println!("Error en la aplicacion: {e}");
        process::exit(1);
    }
}
