fn main() {
    // _if_else();
    // _if_elseif_else();
    // _if_in_let();
    // _mugen_loop();
    // _while_loop();
    // _break_out_six();
    // _skip_four();
    _for_iter();
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

fn _while_loop() {
    let mut number: i32 = 9;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("ARISE!");
}

fn _break_out_six() {
    let mut number: i32 = 0;
    while number != 10 {
        println!("{}!", number);
        if number == 6 {
            break;
        }
        number = number + 1;
    }
    println!("Released!");
}

fn _skip_four() {
    let mut number: i32 = 11;
    while number != 0 {
        number = number - 1;
        if number == 4 {
            continue;
        }
        println!("{}!", number);
    }
    println!("Where 4?");
}

fn _for_iter() {
    let collection = [10, 20, 30, 40, 50];

    for element in collection.iter() {
        println!("Element value is {}", element);
    }
}
