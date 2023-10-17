fn main() {
    _very_simple_match();
}

fn _very_simple_match() {
    let a = 11;
    match a {
        1 => println!("One"),
        2 => println!("More Than One"),
        3 => println!("Three"),
        5 => println!("Five"),
        _ => println!("Four or More than Five"),
    }
}
