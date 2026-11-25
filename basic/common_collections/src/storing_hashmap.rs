use std::collections::HashMap;

pub fn storing_hashmap() {
    // Creating a new hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accesing value in a hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    // or with for loop
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // managing Ownership in hash map
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // update a HashMap
    // Overwriting a value
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 25);
    println!("{scores2:?}");

    // Adding a key and value only if a key isn't present
    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);
    println!("1. {scores3:?}");

    scores3.entry(String::from("Yellow")).or_insert(50);
    println!("2. {scores3:?}");

    scores3.entry(String::from("Blue")).or_insert(50);
    // The second call to entry will not change the hash map,
    // because the Blue team already has the value 10.
    println!("3. {scores3:?}");

    // Updating a value based on the old value
    let text1 = "hello world wonderful world";
    let mut map = HashMap::new();
    println!("{map:?}");
    for word in text1.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        println!("{count}");
    }
    println!("{map:?}");
}
