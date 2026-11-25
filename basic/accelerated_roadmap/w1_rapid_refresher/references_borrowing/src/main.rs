fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The lenght of '{s1}' is {len}");

    // let s2 = String::from("hello");
    // change(&s2);

    let mut s3 = String::from("hello");
    change_mut(&mut s3);
    println!("{}", s3);

    // mutable only once at time.
    // let r1 = &mut s3;
    // let r2 = &mut s3; this mutable will error, because s3 already borrow by r1
    // The solution is using scope
    {
        let _r1 = &mut s3;
    }
    {
        let _r2 = &mut s3;
    }

    // Dangling refereces
    // let reference_to_nothing = dangle();
    let _reference_to_nothing_no_dangle = no_dangle();
}

fn calculate_length(s: &str) -> usize {
    // ampersands ("&") represent references
    // amersands not taking the ownership of the data, instead it's just refering.
    // We call the action of creating a reference borrowing.
    s.len()
}

// This will error, because refereces/borrowing can't modify, it doesn't have ownership
// fn change(some_string: &String) {
//    some_string.push_str(", world");
//}

// MUTABLE REFERENCES
// To modify the references we need to make it mutable
fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}

// DANGLING REFERNCES
// fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
// }
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// RULES OF REFERENCE
// 1. at any given time, you can have either one mutable reference or any number of immutable
// references
// 2. references must always be valid.
