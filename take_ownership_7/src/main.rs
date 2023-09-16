fn main() {
    let s = String::from("Allo");
    take_ownership(s);
    // println!("s just taken by some_string on take_ownership function so it will be error if acces s from here : {}",s);

    let i = 2;
    make_copy(i);
    println!("i just copied : {}", i);

    let s1: String = give_ownership();
    println!("got s1 ownership : {}", s1);

    let s2: String = takes_and_gives_back(String::from("Okay"));
    println!("got s2 ownership after giving ownership : {}", s2);

    let (s3, s3_len): (String, usize) = calculate_length(String::from("AYE"));
    println!("the Length of {} is : {}", s3, s3_len);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_int: i32) {
    println!("some_int have copied : {}", some_int);
}

fn give_ownership() -> String {
    String::from("Here!")
}

fn takes_and_gives_back(value: String) -> String {
    value
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
