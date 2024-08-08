use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let run_result = minigrep::run(config);

    if let Err(e) = run_result {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
