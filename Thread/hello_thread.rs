use std::time::Duration;
use std::thread;

fn msg_hello() -> &'static str {
    std::thread::sleep(Duration::from_secs(1));
    "Hello"
}

fn msg_myname() -> &'static str {
    std::thread::sleep(Duration::from_secs(1));
    "My name is"
}

fn msg_name() -> &'static str {
    std::thread::sleep(Duration::from_secs(1));
    "Trinh Ngoc Minh"
}

fn msg_excited() -> &'static str {
    std::thread::sleep(Duration::from_secs(1));
    "!"
}

fn main() {
    let msg_one = thread::spawn(move || msg_hello());
    let msg_two = thread::spawn(move || msg_myname());
    let msg_three = thread::spawn(move || msg_name());
    let msg_four = thread::spawn(move || msg_excited());

    let msg_one = msg_one.join().expect("msg one error");
    let msg_two = msg_two.join().expect("msg two error");
    let msg_three = msg_three.join().expect("msg three error");
    let msg_four = msg_four.join().expect("msg four error");

    println!("{}", msg_one);
    println!("{}", msg_two);
    println!("{}", msg_three);
    println!("{}", msg_four);
    println!("{} {} {} {}", msg_one, msg_two, msg_three, msg_four);
}