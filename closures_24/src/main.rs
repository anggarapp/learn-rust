use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(i32) -> i32,
{
    calculation: T,
    value: Option<i32>,
}

impl<T> Cacher<T>
where
    T: Fn(i32) -> i32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, args: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(args);
                self.value = Some(v);
                v
            }
        }
    }
}

fn _simulated_expensive_calculation(intensity: i32) -> i32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: i32, random_number: i32) {
    let mut expensive_result = Cacher::new(|num: i32| -> i32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            )
        }
    }
}

#[test] // fail test
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

fn main() {
    let simulatd_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulatd_user_specified_value, simulated_random_number);

    let x = 4;

    let equal_to_x = |z: i32| z == x; //do
                                      // dont
                                      // fn equal_to_x(z: i32) -> bool {
                                      //     z == x
                                      // }
    let y = 4;
    assert!(equal_to_x(y));
}
