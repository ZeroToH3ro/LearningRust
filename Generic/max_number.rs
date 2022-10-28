fn main(){
    let number_list = vec![12,34,59,5,2,200];
    let max_number = largest_value(number_list);
    println!("{}",max_number);

    let char_list = vec!['c','t','e','a'];
    let max_char = largest_value(char_list);
    println!("{}",max_char);
}

fn largest_value<T: PartialOrd + Copy>(element_list: Vec<T>) -> T {
    let mut max_value = element_list[0];

    for element in element_list {
        if max_value < element {
            max_value = element;
        }
    }
    //return value
    max_value
}

