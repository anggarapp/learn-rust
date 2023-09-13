fn main() {
    // _if_else();
    // _if_elseif_else();
    // _if_in_let();
    _mugen_loop();
}

fn _if_else() {
    let number = 40;
    if number < 5 && number != 0 {
        println!("this is true value");
    } else {
        println!("this is false value");
    }
}

fn _if_elseif_else() {
    let number = 42;
    if number % 10 == 0 {
        println!("Can Divided by 10");
    } else if number % 2 == 0 {
        println!("Can Divided by 2");
    } else {
        println!("Can't Divided by 10 or 2");
    }
}

fn _if_in_let() {
    let condition = false;
    let number = if condition { 666 } else { 999 };
    println!("This is the number Kid! {}", number);
}

fn _mugen_loop() {
    loop {
        println!("Blue!");
    }
}
