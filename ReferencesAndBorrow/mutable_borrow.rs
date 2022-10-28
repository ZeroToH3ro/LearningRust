fn main() {
    let mut str = String::from("hello");

    change(&mut str);

    println!("{}", str);
}

fn change(some_string: &mut String) {
    some_string.push_str("worls");
}


