// improt the modules
//mod capturing_env_with_closures;
//mod closure_type_inference_and_annotation;
mod capturing_reference_or_moving_ownership;
mod example_closure;
mod moving_caputred_values_move_out_of_closures_n_the_fn_traits;

// bring the module to this main
//use capturing_env_with_closures::capturing_environment_with_closures;
use capturing_reference_or_moving_ownership::capturing_ref_or_move_ownership;
use example_closure::ex_closure;

fn main() {
    // Capturing the Environment
    //capturing_environment_with_closures();

    // Closure Type Inference and Annotation

    // example closure
    ex_closure();

    // Capturing reference or moving ownnership
    capturing_ref_or_move_ownership();
}
