use std::{env, process};
use colored;
use colored::Colorize;

use minigrep::{Config, run};

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

