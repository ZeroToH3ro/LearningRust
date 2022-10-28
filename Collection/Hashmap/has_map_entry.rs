use std::collections::HashMap;

fn main() {
    //hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("MU"), 10);
    scores.insert(String::from("MU"), 20);

    scores.entry(String::from("MC")).or_insert(10);
    scores.entry(String::from("MC")).or_insert(20);

    println!("{:?}", scores);
}