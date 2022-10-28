#[derive(Debug)]
enum IpAddressKind {
    V4(String),
    V6(String),
}
#[derive(Debug)]
struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

impl IpAddress {
    fn some_function() {
        println!("Block chains development");
    }
}

fn main() {
    // let localhost = IpAddress {
    //     kind: IpAddressKind::V4,
    //     address: String::from("127.0.0.7")
    // };
    let localhost = IpAddressKind::V4(String::from("127.0.0.1"));
    println!("localhost = {:#?}", localhost);
}

// fn route(ip_kind: IpAddressKind) {
//    println!("ip kink = {}", ip_kind);
// }
