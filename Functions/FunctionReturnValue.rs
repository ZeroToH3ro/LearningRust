fn four() -> i32 {
    4
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = four();
    let y = plus_one(1);

    println!("The value of x is {x}");
    println!("The value of y is {y}");
}
