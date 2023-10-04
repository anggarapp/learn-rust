pub fn add(left: usize, right: usize) -> usize {
    left + right
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
    fn panicking() {
        panic!("Make test fail");
    }
}
