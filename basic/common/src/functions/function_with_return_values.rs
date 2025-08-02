pub fn fn_return_values() {
    // some examples of function return value.
    // example 1
    fn five() -> i32 {
        5 // no semicolone becasue this is expression to return as value.
    }
    let x = five();
    println!("The value of x is: {x}");

    // example 2
    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    let y = plus_one(9);
    println!("The value of y is: {y}");
}
