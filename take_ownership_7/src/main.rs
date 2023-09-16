fn main() {
    let s = String::from("Allo");
    take_ownership(s);
    // println!("s just taken by some_string on take_ownership function so it will be error if acces s from here : {}",s);

    let i = 2;
    make_copy(i);
    println!("i just copied : {}", i);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_int: i32) {
    println!("some_int have copied : {}", some_int);
}
