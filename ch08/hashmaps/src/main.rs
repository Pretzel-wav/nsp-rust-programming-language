use std::collections::HashMap;

fn main() {

    // initializing HashMap
    let mut scores = HashMap::new();

    // inserting new values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // creating via `collect`; collect returns a vector of (key, value) tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // HashMap and ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // "Favorite color" and "Blue" are now owned by map
    // println!("{}: {}", field_name, field_value); // error! field_name and field_value are now invalid

    // accessing values in HashMap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // returns Option, since key might not exist

    // iterating over HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // updating HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.insert(String::from("Blue"), 25); // overwriting Blue value
    scores.entry(String::from("Yellow")).or_insert(50); // inserting new key:value if it doesn't already exist
    scores.entry(String::from("Blue")).or_insert(50); // does nothing; Blue already exists
    let yellow_mut_ref = scores.entry(String::from("Yellow")).or_insert(0); // or_insert returns a mutable reference to the value
    *yellow_mut_ref += 1; // can update the mutable reference, but must dereference first

    println!("{:?}", scores);
}