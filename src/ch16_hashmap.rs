use std::collections::HashMap;

pub fn example_hashmap() {
    // example 1
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // example 2
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // example 3
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // borrowing moved
    // println!("{} {}", field_name, field_value);

    // example 4
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}