//use std::io;

pub fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Access directly
    let x_directly = tup.1;
    println!("Access directly value of tup column 1: {x_directly}");
    println!("Access directly value of tup column 2: {}", tup.2);

    // Access with destructuring style
    let (x, y, z) = tup;
    println!("The value of x: {x}");
    println!("The value of y: {y}");
    println!("The value of z: {z}");
}

pub fn array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // Access array
    println!("{}", a[0]);
    println!("{}", months[5]);
}

// remove comment to run this function
/*
pub fn invalid_array_element_access() {
    let b = [1, 2, 3, 4, 5];

    println!("Please enter an array index (0-4).");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = b[index];
    println!("The value of the element at index {index} is: {element}");
}
*/
