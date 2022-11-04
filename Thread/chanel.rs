use crossbeam_channel::unbounded;

fn main() {
    let (send, receiver) = unbounded();

    send.send("Hello world!");

    match receiver.recv() {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("{:?}", e),
    }
}
