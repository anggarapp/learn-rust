fn main() {
    let mut s1 = String::from("welcome to cruel world");
    let len = calculate_length_with_ref(&s1);
    println!("The length of {} is : {}", s1, len);
    change(&mut s1);
    _borrow_rule();
}

fn calculate_length_with_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", Kiddo");
}

fn _borrow_rule() {
    let mut _r = String::from("kiddo");
    let _r1 = &_r; //two immutable reference is no problem
    let _r2 = &_r; //
    let _r3 = &mut _r; //must have either a mutable reference or immutable reference
                       // println!("immutable Ref is {} , Muttable Ref is {}", _r1, _r3) //the error occured here
}
