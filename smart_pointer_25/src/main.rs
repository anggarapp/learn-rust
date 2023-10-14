enum List {
    Cons(i32, Box<List>),
    Nil,
}
use std::{ops::Deref, rc::Rc};
// use List::{Cons, Nil};

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

enum Listrc {
    Cons(i32, Rc<Listrc>),
    Nil,
}

use Listrc::{Cons, Nil};

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Droping CustomSmartPointer");
    }
}

fn main() {
    let b = Box::new(5);
    println!("{}", b);

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let mut x = 5;
    {
        let y = &mut x;
        *y += 1
    }

    println!("{}", x);
    assert_eq!(6, x);
    _demo_deref_traits();
    _demo_drop_traits();
    _demo_increasing_references_count();
}

fn _demo_deref_traits() {
    let my_fav_song = Mp3 {
        audio: vec![1, 2, 3],
        artist: Some(String::from("Yoasobi")),
        title: Some(String::from("Hero")),
    };

    assert_eq!(vec![1, 2, 3], *my_fav_song);
}

fn _demo_drop_traits() {
    let c = CustomSmartPointer {
        data: String::from("will droped"),
    };

    println!("CustomSmartPointer created");
    println!("Wait for droping....");
    drop(c);
    println!("Already droped");
}

fn _demo_reference_counting() {
    let a = Rc::new(Listrc::Cons(5, Rc::new(Listrc::Cons(10, Rc::new(Nil)))));
    let b = Cons(3, a.clone());
    let c = Cons(4, a.clone());
}

fn _demo_increasing_references_count() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("rc = {}", Rc::strong_count(&a));
    let b = Cons(3, a.clone());
    println!("rc after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, a.clone());
        println!("rc after creating c = {}", Rc::strong_count(&a));
    }
    println!("rc after c goes out of scope = {}", Rc::strong_count(&a));
}
