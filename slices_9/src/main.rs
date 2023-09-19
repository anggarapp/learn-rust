fn main() {
    let mut the_string: String = String::from("Some Strings");

    // let _slice = &the_string[0..4]; //
    // let _slice = &the_string[..4]; // same

    // let len = the_string.len();

    // let _slices = &the_string[6..len]; //
    // let _slices = &the_string[6..]; // same

    // let _all_slices = &the_string[..]; //take all slices

    let word = first_word(&the_string);
    println!("the first word is {}", word);

    let literal = "Literal String";
    let literal_first_word = first_word(&literal[..]);

    println!("the first word is {}", literal_first_word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
