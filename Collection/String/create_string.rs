fn main() {
    let s1 = String::from("Hello, world!");
    let s2 = String::new();
    let s3 = "Minh";
    let s4 = s3.to_string();

    let s5 = s1 + "I  am" + &s3;
    println!("{}", s5);

    println!("{}", s1);
}   