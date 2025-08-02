#[derive(Debug)]
#[allow(dead_code)]
struct IpV4 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

//  we can put any kind of data inside an enum variant: strings, numeric types, or structs, or even
//  another enum
#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr {
    V4(IpV4),
    V6(String),
}

pub fn enum_style_4() {
    let home = IpAddr::V4(IpV4 {
        a: 127,
        b: 0,
        c: 0,
        d: 1,
    });
    let lookback = IpAddr::V6(String::from("::2"));
    println!("{:?}", home);
    println!("{:?}", lookback);
}
