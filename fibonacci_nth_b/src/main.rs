use std::io;

fn main() {
    println!("Insert the long of fibonacci series : ");
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value: i32 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    for _n in 1..=value {
        print!("{}, ", _fibonacci_ordinary(_n));
        if _n == value {
            println!("");
        }
    }
}

fn _fibonacci_recursive(nth: i32) -> u64 {
    if nth < 0 {
        panic!("{} is negative!", nth);
    }
    match nth {
        0 => panic!("zero is not a right argument to fibonacci_reccursive()!"),
        1 | 2 => 1,
        3 => 2,
        _ => _fibonacci_recursive(nth - 1) + _fibonacci_recursive(nth - 2),
    }
}

fn _fibonacci_ordinary(nth: i32) -> u64 {
    if nth <= 0 {
        panic!("Zero and Negative is not Valid Iteration");
    } else if nth == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut current = 1;
    for _n in 1..nth {
        sum = current + last;
        last = current;
        current = sum;
    }
    sum as u64
}
