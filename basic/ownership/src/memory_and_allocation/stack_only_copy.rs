// Stack-only data: copy

pub fn stack_only_copy() {
    // store integer to stack
    let x = 5;
    let y = x; // copy from x, so x still can access
    println!("x: {x}, y: {y}");
}
