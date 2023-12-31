use std::io::Write;
use std::time::Duration;
use std::{env, process, thread};

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        Ok(Config { query, filename })
    }
}

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

fn _generate_workout(intensity: i32, random_number: i32) {
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
#[ignore = "fail test"]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

fn _move_keyword_closure() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z: Vec<i32>| z == x;
    // println!("can't use x here: {:?}", x);  // cant use x here
    let y = vec![1, 2, 3];
    // assert!(equal_to_x(y));
}

fn _basic_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn _iterator_adaptors() {
    let _vec_1 = vec![1, 2, 3];
    let _maped_vec: Vec<_> = _vec_1.iter().map(|x| x + 1).collect();
    assert_eq!(_maped_vec, vec![2, 3, 4]);
}

fn _run_config() {
    let mut stderr = std::io::stderr();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        writeln!(&mut stderr, "Problem parsing arguments: {}", err)
            .expect("Could not write to stderr");
        process::exit(1);
    });
}

fn main() {
    let simulatd_user_specified_value = 10;
    let simulated_random_number = 7;

    // _generate_workout(simulatd_user_specified_value, simulated_random_number);

    let x = 4;
    let equal_to_x = |z: i32| z == x; //do
                                      // dont
                                      // fn equal_to_x(z: i32) -> bool {
                                      //     z == x
                                      // }
    let y = 4;
    assert!(equal_to_x(y));

    // move keyword on closure
    _move_keyword_closure();

    //basic iter
    _basic_iterator();

    // iterator adaptors
    _iterator_adaptors();

    //
    _run_config();
}
