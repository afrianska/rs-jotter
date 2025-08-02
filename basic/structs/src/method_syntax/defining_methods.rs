#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigh: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.heigh
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

pub fn defining_method() {
    let rect = Rectangle {
        width: 77,
        heigh: 42,
    };

    println!("The area of rectangle is {} pixels", rect.area());
    println!("");

    println!("{}", rect.width());
    if rect.width() {
        println!("The rectange has nonzero width. it is {}", rect.width);
    }
}
