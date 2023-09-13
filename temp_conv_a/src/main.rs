use std::io;

fn main() {
    let farenheit = 9;
    let celcius = 5;
    let reamur = 4;
    let kelvin = 5;
    loop {
        println!("Choose Unit you want to convert from - [f] Farenheit [c] Celcius [k] Kelvin [r] Reamur");
        let mut from = String::new();
        io::stdin()
            .read_line(&mut from)
            .expect("Failed to read line");

        let from = match from.as_str().trim() {
            "f" => (farenheit, 32),
            "c" => (celcius, 0),
            "r" => (reamur, 0),
            "k" => (kelvin, 273),
            _ => continue,
        };

        println!(
            "Choose Unit you want to convert to - [f] Farenheit [c] Celcius [k] Kelvin [r] Reamur"
        );
        let mut to = String::new();
        io::stdin().read_line(&mut to).expect("Failed to read line");

        let to = match to.as_str().trim() {
            "f" => (farenheit, 32),
            "c" => (celcius, 0),
            "r" => (reamur, 0),
            "k" => (kelvin, 273),
            _ => continue,
        };

        println!("Insert the value");
        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");

        let value: i32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result =
            (value as f64) * ((to.0 as f64) / (from.0 as f64)) - (from.1 as f64) + (to.1 as f64);

        println!("you inserted: {} ", result);
        // let guess: u32 = guess.trim().parse().expect("Type a Number kid");
    }
}
