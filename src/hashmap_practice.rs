use std::collections::HashMap;

pub fn hashmap_examples() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Green"), 45);

    println!("Scores hashmap contains: {:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    hashmap_ownership_example();
    hashmap_updating_examples();
}

fn hashmap_ownership_example() {
    let field_name: String = String::from("Favorite Color");
    let field_value: String = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{}, {}", field_name, field_value);
}

fn hashmap_updating_examples() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("Updated hashmap is: {:?}", scores);

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("Scores is currently: {:?}", scores);

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);

    println!(
        "Scores already has key 'Blue' and does not update its value: {:?}",
        scores
    );

    let text: &str = "Hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("map is now: {:?}", map);
}
