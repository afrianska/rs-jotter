pub fn parameters() {
    println!("Hello, this is printed in function!");
    println!("");

    // another function with parameter
    fn another_function(x: i32) {
        println!("Hello, this is printed from another function.");
        println!("The value of parameter x is: {x}");
    }
    // call another function with parameter value
    another_function(5);

    // another function example
    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value} {unit_label}");
    }
    print_labeled_measurement(5, 'h'); // char ony allow with '', not "".
}
