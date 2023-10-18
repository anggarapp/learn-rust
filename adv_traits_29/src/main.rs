fn main() {
    _associated_type_on_trait();
}

fn _associated_type_on_trait() {
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    // impl Iterator for Counter {
    //     type Item = u32;

    //     fn next(&mut self) -> Option<Self::Item> {}
    // }

    // like

    // pub trait Iterator<T> {
    //     fn next(&mut self) -> Option<T>;
    // }
}

fn _generic_type_parameters_operator_overloading() {
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
}
