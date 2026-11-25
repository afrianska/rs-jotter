fn main() {
    // Ownership rules
    // 1. each value in rust has an owner
    // 2. there can only be one owner at a time
    // 3. when the owner goes out of scope, the value will be dropped

    // Variable scope
    let _s: &str = "hello"; // this is called string literal
    {
        // t is not valid here, since it's not yet declared
        let _t = "hello"; // t is valid from this point forward

        // do stuff with t
    } // this scope is now over, and t is no longer valid

    // The String type
    let mut s = String::from("hello"); // this is called string
    println!("{s}");

    s.push_str(". world!");
    println!("{s}");

    // Memory and allocation
    // example from above
    {
        let _z = String::from("hello");
    }

    // Variable and data interacting with move
    // example 1
    {
        let x = 5;
        let y = x; // this is will work because integers is fixed size.
        println!("{x} and {y}.");
    }
    // example 2
    {
        let s1 = String::from("hello");
        println!("{}", s1);
        let s2 = s1; // this will make error, the value from s1 moved to s2.
        // remember the ownership rules, only one owner in the same time.
        // println!("{s1} and {s2}."); // 2 variable run with exacly pointed the same data.
        // Also the type of data is important, if it integer the size is fixed, not changed.
        println!("{}", s2);
    }

    // Scope and assignment
    let mut k = String::from("hello");
    println!("{k}, world!");
    //k = String::from("ahoy"); // new value have differenct size, so it will error.
    k = String::from("olleh");
    println!("{k}, world!");

    // Variable and data interaction with clone
    let a1 = String::from("hello");
    let a2 = a1.clone(); // this is clone value, not move the value like change the ownership
    println!("a1 = {a1}, a2 = {a2}");

    // Stack-only data: copy
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}"); // This is will work since the size data integer is fixed.

    // Ownership and function
    let j = String::from("hello");
    takes_ownership(j);

    let g = 5;
    make_copy(g);

    // Return values and scope
    let _p1 = give_ownership();
    let p2 = String::from("hello");
    let _p3 = takes_and_give_back(p2);

    let o1 = String::from("hello");
    let (o2, len) = calculate_length(o1);
    println!("There lenght of '{o2}' is {len}.");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn make_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string
    //`String::from("yours")`
}

fn takes_and_give_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
