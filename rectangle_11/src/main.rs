#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let _width_1 = 30;
    let _length_1 = 40;
    let _rec_tuple = (10, 23);
    let _rec_struct = Rectangle {
        length: 60,
        width: 11,
    };

    println!(
        "Area based two of side is {}",
        area_ordinary(_length_1, _width_1)
    );
    println!("Area based on tuple is {}", area_tuple(_rec_tuple));
    println!("Area based on struct is {}", area_struct(&_rec_struct));
    println!("Area using method {}", _rec_struct.area());
}

fn area_ordinary(length: u32, width: u32) -> u32 {
    length * width
}

fn area_tuple(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
