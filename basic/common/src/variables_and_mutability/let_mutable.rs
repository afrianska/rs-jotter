pub fn let_mutable() {
    // let primitively immutable, put "mut" keyword to make it mutable
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");
}
