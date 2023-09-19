fn main() {
    let mut the_string: String = String::from("Some Strings");

    let slice = &the_string[0..4]; //
    let slice = &the_string[..4]; // same

    let len = the_string.len();

    let slices = &the_string[6..len]; //
    let slices = &the_string[6..]; // same

    let all_slices = &the_string[..]; //take all slices

    let _word = first_word(&the_string);
    println!("the first word is {}", _word);
    the_string.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
