struct Rectangle {
    width: u32,
    heigh: u32,
}

pub fn refactoring_with_struct() {
    let rect_1 = Rectangle {
        width: 70,
        heigh: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect_1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.heigh
}
