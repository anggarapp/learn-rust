use std::collections::HashMap;
fn main() {
    let mut _scores = HashMap::new();
    _scores.insert(String::from("Purple"), 9);
    _scores.insert(String::from("Grey"), 99);

    let teams = vec![String::from("Purple"), String::from("Grey")];
    let initial = vec![9, 99];
    let _scores_with_vec: HashMap<_, _> = teams.iter().zip(initial.iter()).collect();
}
