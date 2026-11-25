fn main() {
    // Function, call function with argument.
    println!("-- Function: parameter/argument");
    another_function(5, 'h');
    println!(" ");

    // Statements and Expression
    // Statement: instruction that perform some action and do not return a value
    println!("-- Function: statement and expression");
    fn _func_statement() {
        // _ in front to avoid error because unused function, for var too.
        let _y = 6;
    }
    // Expression: evaluate to a resultant value.
    fn func_expression() {
        let y = {
            // body {} with contain inside is expression;
            let x = 3;
            x + 1
        };

        println!("The value of y is: {y}");
    }
    func_expression();
    println!(" ");

    // Function with return value.
    fn func_with_return_value() -> i32 {
        5
    }
    let data_func_rv = func_with_return_value();
    println!("-- Fucntion with return value");
    println!("The value of data_func_rv is: {data_func_rv}");
    println!(" ");
}

// function with parameter
fn another_function(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}.");
}
