fn main() {
    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let value = six;

    println!("Sum: {:#?}", six);

    if let Some(6) = value {
        println!("This is equal");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        Some(x) => Some(x+1),
        _ => None,
    }
}