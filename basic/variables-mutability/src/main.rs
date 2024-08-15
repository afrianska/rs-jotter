fn main() {
    // LET
    let x = 5;
    println!("The value of x is: {x}");

    // MUTABLE LET
    let mut y = 1; // add "mut" to make mutable variable
    println!("The value of y is: {y}");
    y = 6; // this will be error, because x isn't mutable variable. You have to put "mut" before "x" in the let declaration variable
    println!("The value of y is: {y}");

    // CONSTANT
    // constant aren't just immutable by default, it's always imutable.
    // naming constant use upercase with underscores
    const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECOND);

    // SHADOWING
    let z = 4;

    // shadowing doesn't send error result because we use let to reasign a variable.
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }
    println!("The value of z is: {z}");

    // if we put "mut", this will throw error because shadowing not allowed with mut. exmple:
    // let mut spaces = "    ";
    // spaces = spaces.len();

    // DATA TYPES
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}")

    // SCALAR TYPES
    // Rust has 4 primary scalar type: integers, floating-point numbers, booleans, characters.

    // INTEGER TYPES
}
