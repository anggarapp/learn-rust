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
