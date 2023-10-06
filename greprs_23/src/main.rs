extern crate greprs_23;
use std::{env, process};

use greprs_23::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In File {}", config.filename);

    if let Err(e) = greprs_23::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}
