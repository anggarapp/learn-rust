fn main() {
    let s1 = String::from("ello cruel world");
    let len = calculate_length_with_ref(&s1);
    println!("The length of {} is : {}", s1, len);
}

fn calculate_length_with_ref(s: &String) -> usize {
    s.len()
}
