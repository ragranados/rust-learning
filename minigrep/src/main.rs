use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|_err| {
        process::exit(1);
    });

    let run_result = minigrep::run(config);

    if let Err(_e) = run_result {
        process::exit(1);
    }
}
