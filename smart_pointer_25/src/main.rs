enum List {
    Cons(i32, Box<List>),
    Nil,
}
use std::ops::Deref;
use List::{Cons, Nil};

struct Mp3 {
    audio: Vec<u8>,
    artist: Option<String>,
    title: Option<String>,
}

impl Deref for Mp3 {
    type Target = Vec<u8>;
    fn deref(&self) -> &Vec<u8> {
        &self.audio
    }
}

fn main() {
    let b = Box::new(5);
    println!("{}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let mut x = 5;
    {
        let y = &mut x;
        *y += 1
    }

    println!("{}", x);
    assert_eq!(6, x);
    _demo_deref_traits();
}

fn _demo_deref_traits() {
    let my_fav_song = Mp3 {
        audio: vec![1, 2, 3],
        artist: Some(String::from("Yoasobi")),
        title: Some(String::from("Hero")),
    };

    assert_eq!(vec![1, 2, 3], *my_fav_song);
}
