use std::io;

fn main() {
    let farenheit = (9.0, 32.0);
    let celcius = (5.0, 0.0);
    let reamur = (4.0, 0.0);
    let kelvin = (5.0, 273.15);
    loop {
        println!("Choose Unit you want to convert from - [f] Farenheit [c] Celcius [k] Kelvin [r] Reamur");
        let mut from = String::new();
        io::stdin()
            .read_line(&mut from)
            .expect("Failed to read line");
        let from = from.trim();

        let from_value = match from {
            "f" => farenheit,
            "c" => celcius,
            "r" => reamur,
            "k" => kelvin,
            _ => continue,
        };

        println!(
            "Choose Unit you want to convert to - [f] Farenheit [c] Celcius [k] Kelvin [r] Reamur"
        );
        let mut to = String::new();
        io::stdin().read_line(&mut to).expect("Failed to read line");
        let to = to.trim();

        let to_value = match to {
            "f" => farenheit,
            "c" => celcius,
            "r" => reamur,
            "k" => kelvin,
            _ => continue,
        };

        println!("Insert the value");
        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");

        let value: f64 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = ((value - from_value.1) * (to_value.0 / from_value.0)) + to_value.1;

        println!(
            "The Result of {} to {} Conversion is : {} ",
            from, to, result
        );
        // let guess: u32 = guess.trim().parse().expect("Type a Number kid");
    }
}
