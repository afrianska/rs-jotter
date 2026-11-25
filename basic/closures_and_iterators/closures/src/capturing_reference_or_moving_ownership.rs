use std::thread;

pub fn capturing_ref_or_move_ownership() {
    /*
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    */

    // Change the closure body, so it adds an element
    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list2);
    let mut borrows_mutably = || list2.push(7);
    borrows_mutably();
    println!("After calling closure: {:?}", list2);
    println!();

    // Spawning a new thread using a closure
    let list3 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list3);
    thread::spawn(move || println!("From thread: {:?}", list3))
        .join()
        .unwrap();
}
