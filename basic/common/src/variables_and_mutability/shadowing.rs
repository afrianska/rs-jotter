pub fn shadowing() {
    let x = 5;

    // shadowing is using let to re-assign or re-bind
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
