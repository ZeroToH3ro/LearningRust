fn main() {
  let string = String::from("Hello world!");
  move_into_function(string);

  let number = 21052002;
  copy_into_function(number);
} 

fn move_into_function(string: String) {
  println!("String value: {}", string);  
}

fn copy_into_function(number: i32) {
  println!("Number value: {}", number);
}


