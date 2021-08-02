use std::env;
use std::process;

use project::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguements: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = project::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
