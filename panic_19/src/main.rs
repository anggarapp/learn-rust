use std::fs::File;
fn main() {
    // panic!("farewell");

    let _file = File::open("test.txt");

    let _file = match _file {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem here Kiddos! {:?}", error)
        }
    };
}
