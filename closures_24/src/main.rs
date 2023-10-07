use std::thread;
use std::time::Duration;

fn main() {
    let simulatd_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulatd_user_specified_value, simulated_random_number);
}
fn simulated_expensive_calculation(intensity: i32) -> i32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: i32, random_number: i32) {
    let expensive_result = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result(intensity));
        println!("Next, do {} situps!", expensive_result(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!("Today, run for {} minutes!", expensive_result(intensity))
        }
    }
}
