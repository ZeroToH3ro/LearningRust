use std::io;

fn main() {
    let mut input_line = String::new();

    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    
    let number_th_fibo: u32 = input_line.trim().parse().expect("This is not a number");

    let mut first_number :u32 = 0;
    let mut second_number :u32 = 1;
    let mut result = 0;
    let mut index = 1;

    while index <= number_th_fibo {
        index = index + 1;
        result = first_number + second_number;
        second_number = first_number;
        first_number = result;
    }
    println!("{}", result);
}