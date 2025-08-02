mod defining_n_instantiating_structs;
mod example_program_using_structs;
mod method_syntax;

fn main() {
    defining_n_instantiating_structs::define_n_instantiating_structs::define_n_instantiating_structs();
    println!("");

    defining_n_instantiating_structs::using_the_field_init_shorthand::using_the_field_init_shorthand();
    println!("");

    defining_n_instantiating_structs::creating_instances_from_other_instances_with_struct_update_syntax::creating_instances_from_other_instances_with_struct_update_syntax();
    println!("");

    defining_n_instantiating_structs::using_tuple_structs_without_named_fields_to_create_different_types::using_tuple_structs_without_named_fields_to_create_different_types();
    println!("");

    defining_n_instantiating_structs::unit_like_structs_without_any_fields::unit_like_structs_without_any_fields();
    println!("");

    example_program_using_structs::intro::intro();
    println!("");

    example_program_using_structs::refactoring_with_tuples::refactoring_with_tuples();
    println!("");

    example_program_using_structs::refactoring_with_struct::refactoring_with_struct();
    println!("");

    example_program_using_structs::adding_useful_functionality_with_derived_traits::adding_useful_functionality_with_derived_traits();
    println!("");

    method_syntax::defining_methods::defining_method();
    println!("");

    method_syntax::methods_with_more_parameters::methods_with_more_parameters();
    println!("");

    method_syntax::associated_functions::associated_functions();
    println!("");

    method_syntax::multiple_impl_blocks::multiple_impl_blocks();
}
