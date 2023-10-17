fn main() {
    // _very_simple_match();
    // _if_let();
    // _while_let();
    // _for_loops();
    // _let_deconstruct();
    _function_parameter();
}
fn _function_parameter() {
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    let point = (3, 5);
    print_coordinates(&point);
}

fn _let_deconstruct() {
    let (x, y, z) = (1, 2, 3);
}

fn _for_loops() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
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
