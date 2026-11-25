pub fn intro_iter() {
    let v1 = vec![1, 2, 3];

    // the iterator is stored in the v1_iter
    // Onve we created an iterator, we can use it in a variety of ways.
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    println!();
    // Still don't know the different between use iter or just use for loop.
    for n in v1 {
        println!("{n}");
    }
}
