pub fn add(left: usize, right: usize) -> usize {
    left + right
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
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
}
