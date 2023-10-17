fn main() {
    // _very_simple_match();
    // _if_let();
    _while_let();
}

fn _while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn _if_let() {
    // let favorite_color: Option<&str> = Some("Grey");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
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
