fn largest(list: &[i32]) -> i32 {
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
    let mut _largest = largest(&_numbers);
    println!("largest number is {}", _largest);
    assert_eq!(_largest, 101);
}
