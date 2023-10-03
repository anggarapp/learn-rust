use std::{fmt::format, result};

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

fn _tb_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

trait Summarizable {
    fn summmary(&self) -> String;
}

struct Article {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summarizable for Article {
    fn summmary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summarizable for Tweet {
    fn summmary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
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

    let _tweet = Tweet {
        username: String::from("pendaurulang"),
        content: String::from("Hello Kiddos!"),
        reply: false,
        retweet: false,
    };

    let _article = Article {
        headline: String::from("test"),
        location: String::from("test"),
        author: String::from("test"),
        content: String::from("test"),
    };

    println!("New Tweet here : {}", _tweet.summmary());
    println!("New Article here : {}", _article.summmary());

    let _chars = vec!['w', 'v', 'e', 'r', 'y'];

    let result = _tb_largest(&_numbers);
    println!("Largest Number is: {}", result);
    let result = _tb_largest(&_chars);
    println!("Largest Char is: {}", result);
}
