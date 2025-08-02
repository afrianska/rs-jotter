pub fn mutable_references() {
    let mut s = String::from("Will be"); // put mut
    println!("{s}");

    change(&mut s); // put mut
    println!("{s}");

    println!("");
    let mut x = String::from("Only can mut once at time.");

    let x1 = &mut x;
    //let x2 = &mut x; // cannot borrow x as mutable multiple at once time.
    println!("x1: {x1}");
    //println!("x2: {x2}");

    println!("");
    let mut z = String::from("Mutable in scope.");
    {
        let z1 = &mut z; // Allowed to borrow mutable with z1 in scope.
        println!("{z1}");
    }
    let z2 = &mut z; // Allowed to borrow at z2.
    println!("{z2}");

    println!("");
    let mut y = String::from("Borrow in immutable and mutable at once time");

    let y1 = &y; // no problem
    let y2 = &y; // no problem
                 //let y3 = &mut y; // BIG PROBLEM
    println!("y1: {y1}, y2: {y2}");
    let y3 = &mut y; // no problem
    println!("y3: {y3}");

    println!("");
    let mut i = String::from("Borrow in immutable & mutable at once time.");

    let i1 = &i; // no problem
    let i2 = &i; // no problem
    println!("i1: {i1}, i2: {i2}"); // call immutable first before reassing mutable
    let i3 = &mut i;
    println!("i3: {i3}");
}

// function to change value that borrowed.
// this function should work because mutable right now.
fn change(some_string: &mut String) {
    // put mut
    some_string.push_str(" changed.");
}
