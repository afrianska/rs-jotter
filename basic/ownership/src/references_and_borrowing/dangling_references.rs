pub fn dangling_references() {
    //let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
    println!("{reference_to_nothing}");
}

//fn dangle() -> &String {
// dangle returns a reference to a String
//    let s = String::from("Hallo!"); // s is a new String

//    &s // we return a reference to the String, s
//} // Here, s goes out of scope and is dropped, so its memory goes away.
// Danger!

// no problem
fn no_dangle() -> String {
    let s = String::from("Hallo!");

    s
}
