fn main() {
    // _function_as_parameters();
    // _enum_as_initializer();
    _returning_closure();
}

fn _function_as_parameters() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

fn _enum_as_initializer() {
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

fn _returning_closure() {
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
