// Rust is an expression-based language.

// Statements are intrusction that doesn't return value.
// Expression evaluate to a resultant value.
pub fn statements_and_expression() {
    // this is statements.
    let _y = 6; // _ in front name y to indicated an unused variable.

    // Value 6 is expression.
    // Expression can be a part of statements.

    // Other example
    let a = {
        let b = 3;
        b + 1
    };

    println!("The value of y is: {a}");
}
