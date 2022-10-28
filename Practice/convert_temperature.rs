use std::io;

fn convert_f_to_c (input_f: f32) -> f32 {
    input_f - 32.0 * 0.555
}

fn convert_c_to_f (input_c: f32) -> f32 {
    input_c + 32.0 * 1.8
}

fn main () {
    println!("please enter a number: ");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");    
    //convert number into integer
    let number: f32 = number
        .trim()
        .parse()
        .expect("Number entered was not a number");
    println!("1: F to C\n2:C to F");
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");    

    let choice: i32 = choice 
        .trim()
        .parse()
        .expect("This is not a number");
        
    if choice == 1 {
        let result_f_to_c = convert_f_to_c(number);
        println!("The number f to c is {result_f_to_c}");
    } else {
        let result_c_to_f = convert_c_to_f(number);
        println!("The number c to f is {result_c_to_f}");
    }
}