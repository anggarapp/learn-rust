extern crate rand;

use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please Input your guess");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Type a Number kid");
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small Kid!"),
        Ordering::Greater => println!("Too Big Kid!"),
        Ordering::Equal => println!("Farewell Kiddo"),
    }
}
