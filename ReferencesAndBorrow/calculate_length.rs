fn main() {
    let first_string = String::from("Hello, world! I am Minh");

    let len = calculate_length(first_string);
    println!("The length of '{}' is {}.", first_string, len);
}

fn calculate_length(str: String) -> usize {
    // str is a reference to a String
    str.len() // Here, str goes out of scope. But because it does not have ownership of what
              // it refers to, it is not dropped.
}
