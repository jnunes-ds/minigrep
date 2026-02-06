use std::{env, process};
use colored;
use colored::Colorize;

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}: {}", "Problem parsing arguments".bold().red(), err.red());
        process::exit(1);
    });

    eprintln!("searching for {}", config.query);
    eprintln!("In file {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("{}: {}", "Application error".bold().red(), e.to_string().red());
        process::exit(1);
    };
}

