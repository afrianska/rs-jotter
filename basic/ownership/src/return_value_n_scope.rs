pub fn return_value_n_scope() {
    let s1 = gives_ownership(); // gives_owndership moce its return value into s1
    println!("{s1}");

    let s2 = String::from("Hi"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also
                                       // moves its return value
                                       // into s3
    println!("{s3}");

    let s4 = String::from("Hoi!");
    let (s5, len) = calculate_lenght(s4);
    println!("The length of '{s5}' is {len}");
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of
  // scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its retrun value into the function
    // that call it.

    let some_string = String::from("yours"); // some_string comes into scope.

    some_string // some_string is returned and move out to the calling function.
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope.

    a_string // a_string is returned and moves out to the calling function.
}

// return multiple value using tuple;
fn calculate_lenght(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
