use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must between 1 and 100, got {}", value);
        }
        Guess { value: value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
fn main() {
    // panic!("farewell");

    // create file on error
    // _create_file_on_error();

    // using unwrap
    // _using_unwrap();

    // using expect
    // _using_expect();

    // using ? on Result type return value
    // let _a_string = _read_from_file();

    // custom type for validation
    // let _guess_wrong = Guess::new(201);
    let _guess_right = Guess::new(2);
    println!("{}", _guess_right.value());
}

fn _read_from_file() -> Result<String, io::Error> {
    let mut _string = String::new();
    let mut _file = File::open("test.txt")?.read_to_string(&mut _string)?;
    Ok(_string)
}

fn _using_unwrap() {
    let _file = File::open("test.txt").unwrap();
}

fn _using_expect() {
    let _file = File::open("test.txt").expect("Failed to Open test.txt");
}

fn _create_file_on_error() {
    let _file = File::open("test.txt");
    let _file = match _file {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("test.txt") {
            Ok(fclose) => fclose,
            Err(err) => {
                panic!("Tried to create file but there was problem: {:?}", err)
            }
        },
        Err(error) => {
            panic!("there was problem opening file: {:?}", error);
        }
    };
}
