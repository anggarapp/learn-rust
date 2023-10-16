use std::cell::RefCell;
use std::rc::Weak;
use std::{ops::Deref, rc::Rc};
enum List {
    Cons(i32, Box<List>),
    Nil,
}
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

#[derive(Debug)]
enum Listrfc {
    Cons(Rc<RefCell<i32>>, Rc<Listrfc>),
    Nil,
}
#[derive(Debug)]
enum Listc {
    Cons(i32, RefCell<Rc<Listc>>),
    Nil,
}

impl Listc {
    fn tail(&self) -> Option<&RefCell<Rc<Listc>>> {
        match *self {
            Listc::Cons(_, ref item) => Some(item),
            Listc::Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // _demo_deref();
    // _demo_cons_list();
    // _demo_deref_traits();
    // _demo_drop_traits();
    // _demo_increasing_references_count();
    // _demo_rfcell_interior_muttablity_pattern();
    // _demo_combine_rc_refcell();
    // _demo_leaking_memory();
    _demo_node();
}

fn _demo_node() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![leaf.clone()]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

fn _demo_leaking_memory() {
    let a = Rc::new(Listc::Cons(5, RefCell::new(Rc::new(Listc::Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    let b = Rc::new(Listc::Cons(10, RefCell::new(a.clone())));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(ref link) = a.tail() {
        *link.borrow_mut() = b.clone();
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // overflow the stack
    println!("a next item = {:?}", a.tail());
}

fn _demo_combine_rc_refcell() {
    let value = Rc::new(RefCell::new(5));
    let a = Listrfc::Cons(value.clone(), Rc::new(Listrfc::Nil));
    let shared_list = Rc::new(a);
    let b = Listrfc::Cons(Rc::new(RefCell::new(6)), shared_list.clone());
    let c = Listrfc::Cons(Rc::new(RefCell::new(10)), shared_list.clone());
    *value.borrow_mut() += 10;
    println!("shared_list after = {:?}", shared_list);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

fn _demo_cons_list() {
    let b = Box::new(5);
    println!("{}", b);

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
}

fn _demo_deref() {
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1
    }

    println!("{}", x);
    assert_eq!(6, x);
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

fn a_fn_that_immutably_borrows(a: &i32) {
    println!("a is {}", a);
}

fn a_fn_that_mutably_borrows(b: &mut i32) {
    *b += 1;
}

fn exec_demo(r: &RefCell<i32>) {
    a_fn_that_immutably_borrows(&r.borrow());
    a_fn_that_mutably_borrows(&mut r.borrow_mut());
    a_fn_that_immutably_borrows(&r.borrow());
}

fn _demo_rfcell_interior_muttablity_pattern() {
    let data = RefCell::new(5);
    exec_demo(&data);
}
