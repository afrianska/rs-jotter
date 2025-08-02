#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr {
    V4(String),
    V6(String),
}

pub fn enum_style_2() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let lookback = IpAddr::V6(String::from("::2"));
    println!("{:?}", home);
    println!("{:?}", lookback);
}
