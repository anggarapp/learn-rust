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
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
