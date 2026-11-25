#[derive(Debug)]
struct Rectangle {
    width: u32,
    hight: u32,
}

// Method syntax
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.hight
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        hight: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
