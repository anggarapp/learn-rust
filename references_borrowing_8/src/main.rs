fn main() {
    let mut s1 = String::from("welcome to cruel world");
    let len = calculate_length_with_ref(&s1);
    println!("The length of {} is : {}", s1, len);
    change(&mut s1)
}

fn calculate_length_with_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", Kiddo");
}
