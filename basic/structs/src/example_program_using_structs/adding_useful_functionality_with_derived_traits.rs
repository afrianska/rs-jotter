#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigh: u32,
}

pub fn adding_useful_functionality_with_derived_traits() {
    let rect_1 = Rectangle {
        width: 30,
        heigh: 80,
    };
    println!("rect_1 is {:#?}", rect_1);
    println!("rect_1 is {} * {}", rect_1.width, rect_1.heigh);
    println!("");

    let scale = 2;
    let rect_2 = Rectangle {
        width: dbg!(20 * scale),
        heigh: 50,
    };

    println!("");
    dbg!(rect_2);
}
