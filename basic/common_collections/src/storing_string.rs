pub fn storing_uft8_text_string() {
    // Defining Strings
    // Rust has only one string type in the core language,
    // which is the string slice str that is usually seen in its borrowed form, &str.
    // When Rustaceans refer to “strings” in Rust, they might be referring to either
    // the String or the string slice &str types, not just one of those types

    // Creating a new empty string
    let mut _s = String::new();

    // Often, we’ll have some initial data with which we want to start the string.
    // For that, we use the to_string method,
    // which is available on any type that implements the Display trait,
    // as string literals do.
    let data = "initial contents";
    let _k = data.to_string();

    // The method also works on a literal directly:
    let _z = "initial contents".to_string();

    // We can also use the function String::from to create a String from a string literal.
    let _m = String::from("initial contents");

    // Updating a string
    // A String can grow in size and its contents can change,
    // just like the contents of a Vec<T>, if you push more data into it.

    // Appending with push_str or push
    let mut l = String::from("foo");
    l.push_str("bar");

    // OR
    let mut l1 = String::from("foo");
    let l2 = "bar";
    l1.push_str(l2);
    println!("l2 is {l2}");
    println!("l1 is {l1}");

    let mut a = String::from("lo");
    a.push('l'); // use single quote
    println!("{a}");

    // Concatenating with + or format
    let p1 = String::from("Hello, ");
    let p2 = String::from("world!");
    println!("{p1} + {p2}");

    let p3 = p1 + &p2; // note p1 has been move here and can no longer be used 
    //println!("{p1}"); // p1 can't access
    println!("{p3}");

    // Indexing into strings
    let _r1 = String::from("hi");
    //let h = r1[0]; // this is will be error of usize
    // Rust strings don’t support indexing.

    // Slicing string
    // Indexing into a string is often a bad idea
    // Rather than indexing using [] with a single number, you can use [] with a range
    let hello = "Здравствуйте";
    let hl = &hello[0..4];

    // Iterating Over Strings
    for i in hl.chars() {
        println!("{i}");
    }
    for i in hl.bytes() {
        println!("{i}");
    }
}
