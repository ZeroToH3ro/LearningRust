fn main() {
    let x: i32 = 5;
    let y: Option<i32> = Some(6);

    let sum: i32 = x + y.unwrap_or(0);
    println!("sum = {}", sum);
}