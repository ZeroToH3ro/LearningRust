use std::io;

fn first_word (str: &String) -> usize {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { 
        if item == b' ' {
            return i;
        }
    }
    str.len()
}

fn print_first_word (str: &String) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[..i];
        } 
    }
    &str[..] 
}

fn main() {
    let mut str_input = String::new();

    io::stdin()
        .read_line(&mut str_input)
        .expect("Error reading");

    let index_first_word = first_word(&mut str_input);
    println!("{}", index_first_word);

    let first_word = print_first_word(&mut str_input);
    println!("{}", first_word);
}