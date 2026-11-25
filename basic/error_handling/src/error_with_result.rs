//use std::fs::{self, File};
//use std::io::{self, Read};
use std::fs;
use std::io;

// The T and E are generic type parameters
/*
#[allow(dead_code)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

pub fn error_result() {
    /*
    // This will panic! no matter why File::open failed.
    // However, we want to take different actions for different failure reasons.
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
    */

    /*
    // Matching on Different Errors
    let greeting_file_result2 = File::open("hello.txt");
    let _greeting_file_2 = match greeting_file_result2 {
            Ok(file2) => file2,
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem opening the file: {e:?}"),
                },
                _ => {
                    panic!("Problem opening the file: {error:?}");
                }
            },
        };
    */

    // Shortcut for pacic on error. BUT BE CAREFUL
    //let _greeting_file_result3 = File::open("hello.txt").unwrap();

    // With expect
    //let _greeting_file_result4 = File::open("hello.txt").expect("hello.txt should be included in this project");
}

// Propagating Errors
/*
pub fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
*/

// The ? Operator shortcut
/*
pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;

    Ok(username)
}
*/

// We could even shorten this code further by chaining method calls immediately after the ?
/*
pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
*/

// make this even shorter using fs::read_to_string
/*
pub fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
*/
