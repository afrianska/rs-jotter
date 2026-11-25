fn main() {
    // Mutable
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6; // will error if we don't set variable x as mutable (mut)
    println!("The value of x is: {x}");

    // Constant variable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // Diferent Mu and shadowing
    let spaces_shadowing = "   ";
    let spaces_shadowing = spaces_shadowing.len();
    println!("{}", spaces_shadowing);

    let mut spaces = "  ";
    // spaces = spaces.len(); // this will error
    println!("{}", spaces);
}
