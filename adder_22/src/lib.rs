pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(number: i32) -> i32 {
    number + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello!")
    // format!("Hello {}!", name)
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be less than or equal to 100,
            got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to
            1, got {}.",
                value
            );
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_two_test() {
        let result = add_two(5);
        assert_eq!(result, 7);
    }

    // #[test]
    // fn panicking() {
    //     panic!("Make test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 32,
            width: 32,
        };
        let smaller = Rectangle {
            length: 12,
            width: 12,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            length: 32,
            width: 32,
        };
        let smaller = Rectangle {
            length: 12,
            width: 12,
        };
        assert!(!smaller.can_hold(&larger));
    }
    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Kafka");
    //     assert!(
    //         result.contains("Kafka"),
    //         "Greeting di not contain name, value was `{}`",
    //         result
    //     );
    // }

    #[test]
    #[should_panic]
    fn greatee_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn smaller_than_1() {
        Guess::new(0);
    }
}
