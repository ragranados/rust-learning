use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problema: {err}");
        process::exit(1);
    });

    run(config);
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //"Should have been able to read the file"
    let contents = fs::read_to_string(config.file_path)?;

    println!("Con el texto:\n{contents}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
