struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    //create an instance
    let black = Color {red: 0, green: 0, blue: 0};
    
    //access data through dot
    println!("Black = (red:{}, green:{}, blue:{})", black.red, black.green, black.blue);

    //because struct is default immutale, add mut to change value of variable
    let mut link_color = Color {red: 0, green: 0, blue: 200};
    link_color.blue = 235;
    println!("Link color = (red:{}, green:{}, blue:{})", link_color.red, link_color.green, link_color.blue);

    // Copy elements from another instance
    let pink = Color {green: 255, .. link_color};
    println!("Pink = (red:{}, green:{}, blue:{})", pink.red, pink.green, link_color.blue);

    //implement a function to create struct
    let mid_night_blue = create_color();
    println!("Midnight blue = (red:{}, green:{}, blue:{})", mid_night_blue.red, mid_night_blue.green,mid_night_blue.blue);
}

fn create_color () -> Color {
    Color {red: 25, green: 25, blue: 30}
}
