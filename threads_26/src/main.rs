use std::thread;

fn main() {
    _demo_init_threads();
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
}
