fn main() {
    let mut _init_string = String::new();
    let _data: &str = "initial contents";
    let _s = _data.to_string();
    let _s = "initial contents".to_string();
    let _s = String::from("initial contents");
    println!("{}", _s);

    let mut _klinkklank = String::from("Klink ");
    _klinkklank.push_str("Klank");
    println!("{}", _klinkklank);
    let klunk = String::from(" Klunk");
    _klinkklank.push_str(&klunk);
    println!("{}", _klinkklank);
}
