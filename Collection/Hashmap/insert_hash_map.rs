use std::collections::HashMap;

fn main() {
    let text = "hello world this is a rust language";

    let mut map = HashMap::new();
    for i in text.split_whitespace() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map );
}