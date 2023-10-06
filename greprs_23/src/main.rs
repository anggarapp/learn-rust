extern crate greprs_23;
use std::io::prelude::*;
use std::{env, process};

use greprs_23::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // println!("Problem parsing argument: {}", err);
        writeln!(&mut stderr, "Problem parsing arguments: {}", err)
            .expect("Could not write to stderr");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In File {}", config.filename);

    if let Err(e) = greprs_23::run(config) {
        // println!("Application error: {}", e);
        writeln!(&mut stderr, "Application error: {}", e).expect("Could not write to stderr");
        process::exit(1);
    };
}
