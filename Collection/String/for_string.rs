fn main() {
    let string = String::from("Hello, I am Minh");
    //print bytes
    for i in string.bytes() {
        println!("{}", i);
    }
    //print character
    for i in string.chars() {
        println!("{}", i);
    }
}