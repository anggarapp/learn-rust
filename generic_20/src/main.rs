struct Point<T> {
    x: T,
    y: T,
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

    let _int = Point { x: 5, y: 12 };
    let _flt = Point { x: 6.6, y: 12.9 };
}
