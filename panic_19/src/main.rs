use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
fn main() {
    // panic!("farewell");

    // create file on error
    // _create_file_on_error();

    // using unwrap
    // _using_unwrap();

    // using expect
    // _using_expect();

    // using ? on Result type return value
    let _a_string = _read_from_file();
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
