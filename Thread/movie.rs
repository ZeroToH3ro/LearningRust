use std::thread::JoinHandle;
use std::time::Duration;
use std::thread;

fn main() {
    let a: JoinHandle<usize> = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        50
    });
    // let b: JoinHandle<usize> = thread::spawn(move ||{
    //     for i in 1..=10 {
    //         println!("B: {}", i);
    //     }
    // });
    println!("Waiting for getting data...");

    match a.join() {
        Ok(value) => println!("Value: {}", value),
        Err(e) => println!("{:?}", e),
    }
}