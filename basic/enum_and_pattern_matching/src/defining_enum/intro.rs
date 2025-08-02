#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

pub fn intro() {
    println!("This is intro of enum chapter.");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    print_type_of(&four);
    print_type_of(&six);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
