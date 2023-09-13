fn main() {
    // _ifelse();
    _ifelseifelse();
}

fn _ifelse() {
    let number = 40;
    if number < 5 && number != 0 {
        println!("this is true value");
    } else {
        println!("this is false value");
    }
}

fn _ifelseifelse() {
    let number = 42;
    if number % 10 == 0 {
        println!("Can Divided by 10");
    } else if number % 2 == 0 {
        println!("Can Divided by 2");
    } else {
        println!("Can't Divided by 10 or 2");
    }
}
