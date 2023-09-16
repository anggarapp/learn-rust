fn main() {
    // _using_clone();
    _default_copy_of_number_data();
}

fn _using_clone() {
    let mut s = String::from("Aye");
    s.push_str(", Kiddos!");

    let t = s.clone();
    println!("s = {} must same with t = {}", s, t);
}

fn _default_copy_of_number_data() {
    let int: i32 = 2;
    let re_int: i32 = int;

    assert_eq!(int, re_int);

    let float: f64 = 2.2;
    let re_float: f64 = float;

    assert_eq!(float, re_float);

    let tuple_of_number: (i32, i32) = (2, 3);
    let re_tuple_of_number: (i32, i32) = tuple_of_number;

    assert_eq!(tuple_of_number, re_tuple_of_number);

    //it will error
    // let tuple_of_mix: (i32, String) = (2, String::from("Error"));
    // let re_tuple_of_mix: (i32, String) = tuple_of_mix;
}
