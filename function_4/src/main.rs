fn main() {
    println!("Hello, Kid!");

    // function
    another_function(15, 2);

    // statement
    let first_statement = 3;
    let second_statement = {
        let x = 52;
        x / 2
    };
    println!(
        "The first statement is: {}, and second is: {}",
        first_statement, second_statement
    );

    let six_statement = six();
    println!("its already: {}", six_statement);
}

fn another_function(x: i32, y: i32) {
    println!("first another_function parameter is : {}", x);
    println!("second another_function parameter is : {}", y);
}

fn six() -> i32 {
    6
}
