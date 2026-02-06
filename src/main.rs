use std::{fs, env, process, error};
use colored;
use colored::Colorize;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}: {}", "Problem parsing arguments".bold().red(), err.red());
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("{}: {}", "Application error".bold().red(), e.to_string().red());
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With thext:\n{}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename})
    }

}

