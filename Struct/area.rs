#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //method: get
    fn width(&self) -> bool {
        self.width > 0
    }

    fn height(&self) -> bool {
        self.height > 0
    }
    //method: calculate area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30*scale),
        height: 0,
    };

    if rect1.width() && rect1.height() {
        println!("{:#?}", rect1); // we can use :? to print the object
        println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
        );
    } else {
        println!("Width or height is invalid");
    }
    
}
