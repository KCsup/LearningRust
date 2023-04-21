fn main() {
    let version_four = IpAddress::V4;

    let my_ip = IpAddress::V4(String::from("1.0.0.1"));
    
    println!("{:?}", my_ip);
}

#[derive(Debug)]
enum IpAddress {
    V4(String),
    V6(String),
}

// #[derive(Debug)]
// struct IpAddress {
//     kind: IpAddressKind,
//     address: String,
// }
// Could define struct for coupling enum data, but it's not as concise
// as just having attributes in the enum itself
