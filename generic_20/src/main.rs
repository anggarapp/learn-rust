struct ExPoint<T> {
    x: T,
    y: T,
}

impl<T> ExPoint<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn _largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let _numbers = vec![34, 55, 22, 101, 66];
    let mut _largest = _largest(&_numbers);
    println!("largest number is {}", _largest);
    assert_eq!(_largest, 101);

    let _point_1 = ExPoint { x: 1, y: 2 };
    let _point_2 = ExPoint { x: 8.9, y: 0.6 };
    println!("x Of _point_1 is {}", _point_1.x());

    let _int = Point { x: 5, y: 12 };
    let _flt = Point { x: 6.6, y: 12.9 };
    let _bth = Point { x: 99, y: 11.3 };

    let _mix = _bth.mixup(_flt);
    println!("Mix 1 X = {}, Mix 1 Y = {}", _mix.x, _mix.y);
}
