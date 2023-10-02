use std::collections::HashMap;
fn main() {
    let mut _scores = HashMap::new();
    _scores.insert(String::from("Purple"), 9);
    _scores.insert(String::from("Grey"), 99);

    let teams = vec![String::from("Purple"), String::from("Grey")];
    let initial = vec![9, 99];
    let _scores_with_vec: HashMap<_, _> = teams.iter().zip(initial.iter()).collect();

    // get value of Hash Map
    let _team_name = String::from("Grey");
    let _score = _scores.get(&_team_name);
    // or
    let _score = _scores.get("Grey");

    println!("{}", _score.unwrap());

    for (key, value) in &_scores {
        println!("{}: {}", key, value);
    }

    let mut _late_scores = HashMap::new();
    _late_scores.insert(String::from("Red"), 13);
    _late_scores.insert(String::from("Red"), 54);

    println!("{:?}", _late_scores);
}
