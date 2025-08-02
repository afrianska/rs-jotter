#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigh: u32,
}

// impl 1
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.heigh
    }
}

// impl 2
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.heigh > other.heigh
    }
}

pub fn multiple_impl_blocks() {
    let rect_1 = Rectangle {
        width: 10,
        heigh: 20,
    };

    let rect_2 = Rectangle {
        width: 30,
        heigh: 40,
    };

    println!("{}", rect_1.area());
    println!("{}", rect_1.area());
    println!("{}", rect_2.can_hold(&rect_1));
    println!("{}", rect_1.can_hold(&rect_2));
}
