use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Failed to parse arguments: {}", err);
        process::exit(1);
    });
    println!("Finding {} in file {}", config.query, config.filename);
    // two points:
    // 1. Err(e)
    // 2. if let
    if let Err(e) = minigrep::run(config){
        eprintln!("Application error! {}", e);
        process::exit(1);
    }

}

