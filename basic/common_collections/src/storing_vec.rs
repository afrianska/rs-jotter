use std::{i32, vec};

pub fn storing_list_value_vec() {
    // Rust’s standard library includes a number of very useful data structures,
    // called collections.
    // Storing List of values with vectors

    // creating a new vector (empty)
    let _v1: Vec<i32> = Vec::new();

    // createing a new vector containing values
    let _v2 = vec![1, 2, 3];

    // updating a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // reading elements of vectors
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let _v3 = vec![1, 2, 3, 4, 5];
    //let _does_not_exist = &v3[100]; // this is will error: index out of bounds
    //let _does_not_exist = v3.get(100);

    let mut _v4 = vec![1, 2, 3, 4, 5];
    //let first = &v4[0]; // will error because immutable borrow
    //v4.push(6); // this is mutable
    //println!("The first element is: {first}");

    // Iterating over the values in a vector
    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{i}");
    }

    // iterating mutable version.
    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        *i += 50;
        println!("{i}");
    }

    // Using an enum to store multiple types
    #[derive(Debug)]
    #[allow(dead_code)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let _ = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
