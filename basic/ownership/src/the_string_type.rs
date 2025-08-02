pub fn the_string_type() {
    // Store in heap
    let mut s = String::from("Hello");

    s.push_str(", World!"); // push_str() appends a literal to a String

    println!("{s}"); // this will print `hello, world!`
}
