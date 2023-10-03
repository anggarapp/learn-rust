fn main() {
    let _numbers = vec![34, 55, 22, 101, 66];
    let mut _largest = _numbers[0];

    for number in _numbers {
        if number > _largest {
            _largest = number;
        }
    }

    println!("largest number is {}", _largest);
    assert_eq!(_largest, 101);
}
