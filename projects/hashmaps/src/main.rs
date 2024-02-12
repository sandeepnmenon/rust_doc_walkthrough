use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    //Inserting
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // get method argument is &K
    // get method returns Option<&V>. Here Option<&i32>
    // copied is to convert to Option<i32>
    // unwrap_or(0) is to get 0 if Option is None.
    println!("Team: {team_name} Score: {score}");

    // Iterating
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // For fields with Copy train; like i32, values are copied into the hashmap
    // But for Strings, the hashmap gets ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // We can't use `field_name` and `field_name` after.

    // We can insert references into the hashmap. But they have to be valid as lonh as the hash map is valid. Will be discussed in lifetimes

    //Updating
    scores.insert(String::from("Blue"), 25); // Original value overwritten
    println!("{:?}", scores);

    // Inserting if Key isn't present
    scores.entry(String::from("Blue")).or_insert(50); // Does not insert 50 because key is present
    println!("{:?}", scores);

    // Updating value based on old value. Eg
    let text = "hello wonderful world, hello";
    let mut map = HashMap::new();

    for character in text.split_whitespace() {
        let count = map.entry(character).or_insert(0);
        *count += 1;
        // or_insert returns a mutable reference (&mut V) to the value
    }
    println!("{:?}", map);

    // Note: Hashmap uses the SipHash hasing function. More secure but slower.
    // Can use different hashers by implementing using the `BuildHasher` train
}
