#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width: size,
        }
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x_shuared = f64::powi(other.x - self.x, 2);
        let y_shuared = f64::powi(other.y - self.y, 2);
        f64::sqrt(x_shuared + y_shuared)
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
    let _other_rec_struct = Rectangle {
        length: 20,
        width: 4,
    };

    println!(
        "Area based two of side is {}",
        area_ordinary(_length_1, _width_1)
    );
    println!("Area based on tuple is {}", area_tuple(_rec_tuple));
    println!("Area based on struct is {}", area_struct(&_rec_struct));
    println!("Area using method {}", _rec_struct.area());

    let point_1 = Point { x: 3.0, y: 2.0 };
    let point_2 = Point { x: 6.0, y: 6.6 };

    point_1.distance(&point_2);
    (&point_1).distance(&point_2);

    println!(
        "Can rectangle hold other_rectangle? {}",
        _rec_struct.can_hold(&_other_rec_struct)
    );
    println!(
        "Can other_rectangle hold rectangle? {}",
        _other_rec_struct.can_hold(&_rec_struct)
    );
    // associated Function
    let _other_rectangle = Rectangle::square(5);
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
