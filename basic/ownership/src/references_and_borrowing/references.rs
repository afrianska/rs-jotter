pub fn references() {
    let s1 = String::from("Hi Yo!");
    let len = calculate_length(&s1); // & before s1 is references sign.

    // We call the action of creating a reference 'borrowing'

    println!("The length of '{s1}' is {len}.");
    println!("");

    //change(&s1);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
}

// function to change value that borrowed.
// this function shouldn't work because not mutable.
//fn change(some_string: &String) {
//    some_string.push_str(", world");
//}

/*
The Rules of References
Let’s recap what we’ve discussed about references:
• At any given time, you can have either one mutable reference or any
number of immutable references.
• References must always be valid.
*/
