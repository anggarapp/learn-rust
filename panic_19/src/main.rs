use std::fs::File;
use std::io::ErrorKind;
fn main() {
    // panic!("farewell");

    let _file = File::open("test.txt");

    let _file = match _file {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            // panic!("Problem here Kiddos! {:?}", error)
            match File::create("test.txt") {
                Ok(fclose) => fclose,
                Err(err) => {
                    panic!("Tried to create file but there was problem: {:?}", err)
                }
            }
        }
        Err(error) => {
            panic!("there was problem opening file: {:?}", error);
        }
    };
}
