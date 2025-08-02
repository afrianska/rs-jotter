pub fn intro() {
    let width_1 = 30;
    let heigh_1 = 50;

    println!(
        "The area of the rectangle is {} square pixel",
        area(width_1, heigh_1)
    );
}

fn area(width: u32, heigh: u32) -> u32 {
    width * heigh
}
