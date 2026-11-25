use std::net::IpAddr;

pub fn panic_or_not_panic() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}
