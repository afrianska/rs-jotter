#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigh: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            heigh: size,
        }
    }
}

pub fn associated_functions() {
    let rect = Rectangle::square(50);
    println!(
        "The square has width: {} and heigh: {}",
        rect.width, rect.heigh
    );
}
