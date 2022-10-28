struct Data1 {
    num1: i32,
    num2: i32,
    str: String,
    optional: Option<i32>
}

struct Data2 {
    num1: i32,
    num2: i32,
    str: String,
    optional: Option<i32>
}

impl Data2 {
    fn new () -> Self {
        Data2 {
            num1: 15,
            num2: 112,
            str: "Hello I am Minh!".to_string(),
            optional: None
        }
    }
}

impl Data1 {
    fn new () -> Self {
        Data1 {
            num1: 5,
            num2: 12,
            str: "Hello world!".to_string(),
            optional: None
        }
    }
}

trait Transform {
    fn revert(&self) -> String;

    fn print_data(&self){
        println!("{}", self.revert());
    }
}

impl Transform for Data1 {
    fn revert(&self) -> String { 
        self.str.chars().rev().collect::<String>()
    }
}

impl Transform for Data2 {
    fn revert(&self) -> String { 
        self.str.chars().rev().collect::<String>()
    }
}

fn main() {
    let first_data = Data1::new();
    first_data.print_data();

    let second_data = Data2::new();
    second_data.print_data();

}   

