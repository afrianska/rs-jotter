pub fn refactoring_with_tuples() {
    let rect_1 = (60, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect_1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
