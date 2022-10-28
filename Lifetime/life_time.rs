fn main() {
    let num1 = 20;
    let num2 = 30;

    println!("Who is greater ? {}", get_ref(&num1, &num2));
}

fn get_ref<'life_time_1, 'life_time_2: 'life_time_1>(
    num1: &'life_time_1 i32,
    num2: &'life_time_2 i32,
) -> &'life_time_1 i32 {
    if num1 > num2 {
        num1
    } else {
        num2
    }
}

//case use life time
fn test_1(param1: Vec<f64>) -> Vec<f64> {
    param1 
} //life time don't apply

fn test_2<'life_time>(param1: &'life_time Vec<f64>) -> Vec<f64> {
    param1 
} //life time don't apply

fn test_3<'life_time>(param1: &'life_time Vec<f64>) -> &Vec<f64> {
    param1 
} //life time apply