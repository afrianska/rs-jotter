pub fn if_expressions() {
    let number_a = 3;
    if number_a < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    println!("");

    // Handling multiple condition
    let number_b = 6;
    if number_b % 4 == 0 {
        println!("number is divisible by 4");
    } else if number_b % 3 == 0 {
        println!("number is divisible by 3");
    } else if number_b % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    println!("");

    // If in let statement
    let condition = true;
    let number_c = if condition { 5 } else { 6 };
    println!("The value of number is: {number_c}");
}
