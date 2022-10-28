enum SheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main () {
    let row = vec![
        SheetCell::Int(5),
        SheetCell::Float(6.4),
        SheetCell::Text(String::from("Hello World!")),
    ];

    match &row[1] {
        &SheetCell::Int(i32) => println!("{}", i32),
        _ => println!("This is not a float")
    }
    
}