// Variable and Data Interaction with Move

pub fn with_move() {
    // store string to heap.
    // string doesn't has fixed size like integer with type i32, i64, etc.
    // to store in heap, we need to request memory allocation.
    // and when we done we need to returning memory to allocator.
    // in other language, they have GC that automaticaly manage it.

    let s1 = String::from("hello"); // request memory then store in heap.
    let s2 = s1; // value s1 moved to s2.

    //println!("s1: {s1}"); // s1 already move, not allowed to access.
    println!("s2: {s2}");

    let a = i32::from(7);
    let b = a; // allowed to access because integer has fixed size i32.
    println!("a: {a}");
    println!("b: {b}");

    // Still confused to understand, but I will keep learn next.
}
