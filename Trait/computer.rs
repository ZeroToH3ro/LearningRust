trait Clicky {
    fn click(&self) -> String;
}

struct Keyboard;
impl Clicky for Keyboard {
    fn click(&self) -> String {
        // println!("Keyboard click");
        "Keyboard Input".to_owned()
    }
}

struct Mouse;
impl Clicky for Mouse {
    fn click(&self) -> String {
        // println!("Mouse click");
        "Mouse Input".to_owned()
    }
}

fn main() {
    // let x = Keyboard;
    // x.click();
    // let y = Mouse;
    // y.click();

    let x: Box<dyn Clicky> = Box::new(Keyboard);
    let y: Box<dyn Clicky> = Box::new(Mouse);
    let clickers = vec![x, y];
    for i in clickers {
        // i.click();
        if i.click() == "Mouse Input".to_owned() {
            println!("This is mouse input");
        } else {
            println!("This is Keyboard input");
        }
    }
}
