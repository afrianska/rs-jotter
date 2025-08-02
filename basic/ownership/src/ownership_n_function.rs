pub fn ownership_n_function() {
    let s = String::from("hello, from ownership."); // s comes into scope

    // s's value moves into the function and so is no longer valid here
    takes_ownership(s);

    // println!("s: {s}"); // Access s here will be error. s already move to function.

    let x = 8; // x comes into scope

    // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward
    makes_copy(x);

    println!("x: {x}"); // Access x here will allowed because it stored in stack.
} // Here, x goes out of scope, then s. However, because s's value was moved, nothing special
  // happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer gous out of scope. Nothing special happens.

// Current undersanding.
// Store to heap:
// - Integer can be store easily because primitively fixed size.
// - String need to request memory allocation because unfixed size.

// Store to stack:
// Not special, everything can store easily.
