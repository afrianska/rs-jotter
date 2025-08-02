#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigh: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.heigh
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.heigh > other.heigh
    }
}

pub fn methods_with_more_parameters() {
    let rect_1 = Rectangle {
        width: 10,
        heigh: 20,
    };

    println!("{}", rect_1.area());

    let rect_2 = Rectangle {
        width: 30,
        heigh: 40,
    };

    let rect_3 = Rectangle {
        width: 50,
        heigh: 60,
    };

    println!("");
    println!("Can rect_1 hold rect_2? {}", rect_1.can_hold(&rect_2));
    println!("Can rect_3 hold rect_2? {}", rect_3.can_hold(&rect_2));
}
