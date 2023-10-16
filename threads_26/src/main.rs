use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // _demo_init_threads();
    // _demo_move_on_closure();
    // _demo_channel_mpsc();
    _demo_view_redeiver_waiting();
}

fn _demo_init_threads() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("this is number {} from spawned threads kiddos", i);
        }
    });

    for i in 1..5 {
        println!("this is number {} from main threads kiddos", i);
    }

    handle.join();
}

fn _demo_move_on_closure() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join();
}

fn _demo_channel_mpsc() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn _demo_view_receiver_waiting() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
}
