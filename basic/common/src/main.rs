mod control_flow;
mod data_types;
mod functions;
mod hello_world;
mod variables_and_mutability;

fn main() {
    // Hello World
    hello_world::hello::hello();
    println!("");

    // Variables and Mutability
    // Let mutable
    println!("-----let mutable-----");
    variables_and_mutability::let_mutable::let_mutable();
    println!("");

    // Constants
    println!("-----Constants-----");
    variables_and_mutability::constants::constants();
    println!("");

    // Shadowing
    println!("-----Shadowning-----");
    variables_and_mutability::shadowing::shadowing();
    println!("");

    // Data types
    // Scalar types: integer type
    println!("-----Data Type (Scalar): Integer-----");
    data_types::scalar_types::integer_types();
    println!("");

    // Scalar type: integer literals
    println!("-----Data Type (Scalar): Integer (literals)-----");
    data_types::scalar_types::integer_literals();
    println!("");

    // Scalar type: floating
    println!("-----Data Type (Scalar): Floating-----");
    data_types::scalar_types::floating();
    println!("");

    // Numeric operation
    println!("-----Numeric Operation-----");
    data_types::scalar_types::numeric_operation();
    println!("");

    // Scalar type: boolean
    println!("-----Data Type (Scalar): Boolean-----");
    data_types::scalar_types::boolean();
    println!("");

    // Scalar type: character
    println!("-----Data Type (Scalar): Character-----");
    data_types::scalar_types::character();
    println!("");

    // Compound type: tuple
    println!("-----Data Type (Compound): Tuple-----");
    data_types::compound_types::tuples();
    println!("");

    // Compound type: array
    println!("-----Data Type (Compound): Array-----");
    data_types::compound_types::array();
    println!("");

    // Compound type: invalid array element access
    //println!("-----Data Type (Compound): Invalid Array Element Access-----");
    //data_types::compound_types::invalid_array_element_access();
    //println!("");

    // Function
    // Parameters
    println!("-----Function: Parameters-----");
    functions::parameters::parameters();
    println!("");

    // Statements and expression
    println!("-----Function: Statements and Expression-----");
    functions::statement_and_expressions::statements_and_expression();
    println!("");

    // Function with return value
    println!("-----Function: Funtion with Retrun Value-----");
    functions::function_with_return_values::fn_return_values();
    println!("");

    // Control Flow
    // If Expressions
    println!("-----Control Flow: If Expressions-----");
    control_flow::if_expressions::if_expressions();
    println!("");

    // Loop
    println!("-----Control Flow: Repetition with Loop-----");
    control_flow::repetition_loop::repetition_loop();
    println!("");

    // While
    println!("-----Control Flow: Conditional Loop with While-----");
    control_flow::conditional_while::loop_with_while();
    println!("");

    // For In
    println!("-----Control Flow: Collection with For-----");
    control_flow::collection_for::collection_for();
    println!("");
}
