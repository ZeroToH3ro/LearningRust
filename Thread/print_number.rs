use std::thread;

fn main() {
    let a = thread::spawn(|| {
        for i in 1..=10 {
            println!("A: {}", i);
        }
    });

    let b = thread::spawn(|| {
        for i in 1..=10 {
            println!("B: {}", i);
        }
    });

    a.join();
    b.join();
}
