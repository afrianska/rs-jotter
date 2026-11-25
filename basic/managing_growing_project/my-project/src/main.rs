use crate::garden::vegetables::Aspargarus;

pub mod garden;

fn main() {
    let plant = Aspargarus {};

    println!("I'm growing {plant:?}!");
}
