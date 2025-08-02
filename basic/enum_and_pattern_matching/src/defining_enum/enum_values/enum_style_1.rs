#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

pub fn enum_style_1() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let lookback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:?}", home.kind);
    println!("{:?}", home.address);
    println!("{:?}", lookback);
}
