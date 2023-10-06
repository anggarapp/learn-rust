use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Config {
    query: String,
    filename: String,
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(args);

    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong on reading the file");

    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}
