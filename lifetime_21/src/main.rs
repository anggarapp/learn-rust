use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please {}", announcement);
        self.part
    }

    fn separate_lifetime<'b>(string: &'b str) -> &'b str {
        string
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn longest_with_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let r;
    {
        let x = 5;
        r = &x;
        println!("r: {}", r);
    }
    // println!("r: {}", r); //the lifetime not long enough to borrowing here

    let string_1 = String::from("despair");
    let string_2 = "emptiness";

    let result = longest(string_1.as_str(), string_2);
    println!("The longest string is {}", result);

    let _novel = String::from("Come Forth Despair. Find my emptiness...");

    let _first_sentence = _novel.split('.').next().expect("Couldnt find a '.'");

    let _i = ImportantExcerpt {
        part: _first_sentence,
    };

    let _static: &'static str = "Static lifetime";
}
