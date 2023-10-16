use std::thread;

fn main() {
    // _demo_init_threads();
    _demo_move_on_closure();
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
