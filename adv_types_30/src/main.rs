fn main() {
    // _type_aliases();
    _type_not_returning_anything();
}

fn _type_aliases() {
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;
    let _f: Thunk = Box::new(|| println!("hi"));
    // fn takes_long_type(f: Thunk) {}
    // fn returns_long_type() -> Thunk {}

    use std::fmt;
    use std::io::Error;
    type Result<T> = std::result::Result<T, std::io::Error>;
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }
}

fn _type_not_returning_anything() {
    fn bar() -> ! {
        loop {
            print!("and ever ");
        }
    }
}
