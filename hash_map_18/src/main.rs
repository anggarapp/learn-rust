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

    // Insert if Key Exist
    _late_scores.entry(String::from("Red")).or_insert(99);
    _late_scores.entry(String::from("Blue")).or_insert(101);

    println!("{:?}", _late_scores);

    let text = "hello cruel wonderful cruel despair";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); //returning reference to hashmap value
        *count += 1;
    }
    println!("{:?}", map);
}
