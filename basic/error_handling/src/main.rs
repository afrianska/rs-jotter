//use std::fs::File;

//mod error_with_panic;
//mod error_with_result;
mod to_panic_or_not_to_panic;

fn main() {
    //error_with_panic::error_panic();

    //error_with_result::error_result();
    //let n = error_with_result::read_username_from_file();
    //println!("{n:?}");

    //let greeting_file = File::open("hello.txt")?;

    // To panic_or_not_panic
    to_panic_or_not_to_panic::panic_or_not_panic();
}
