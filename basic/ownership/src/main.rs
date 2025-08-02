mod memory_and_allocation;
mod ownership_n_function;
mod references_and_borrowing;
mod return_value_n_scope;
mod slice_type;
mod the_string_type;

fn main() {
    the_string_type::the_string_type();
    println!("");

    memory_and_allocation::with_move::with_move();
    println!("");

    memory_and_allocation::with_clone::with_clone();
    println!("");

    memory_and_allocation::stack_only_copy::stack_only_copy();
    println!("");

    ownership_n_function::ownership_n_function();
    println!("");

    return_value_n_scope::return_value_n_scope();
    println!("");

    references_and_borrowing::references::references();
    println!("");

    references_and_borrowing::mutable_references::mutable_references();
    println!("");

    references_and_borrowing::dangling_references::dangling_references();
    println!("");

    slice_type::intro::intro();
    println!("");

    slice_type::string_slices::string_slice();
    println!("");

    slice_type::other_slices::other_slices();
    println!("");
}
