// Variable and Data Interaction with Clone

pub fn with_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // we clone, not move it. So s1 still exist to access.

    println!("{s1}");
    println!("{s2}");
}
