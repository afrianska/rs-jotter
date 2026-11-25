//mod generic_data_types_in_enum;
//mod generic_data_types_in_func;
mod defining_trait;
mod generic_data_types_in_method;
mod generic_data_types_in_struct;
mod implementing_trait_on_type;
mod performace_of_code_using_generics;
mod removing_duplication_by_extracting_function;
mod using_default_implementations_trait_on_type;
mod using_traits_as_parameters;

// Mod for Validating References with Lifetimes
mod validating_references_with_lifetimes;

//use crate::generic_data_types_in_func::function_definitions;
//use crate::defining_trait::Summary;

use crate::generic_data_types_in_method::method_definition;
use crate::generic_data_types_in_struct::struct_definition;
use crate::performace_of_code_using_generics::performance_using_generics;
use crate::removing_duplication_by_extracting_function::finding_the_largest_number;
use crate::removing_duplication_by_extracting_function::finding_the_largest_number_with_ex_func;
//use implementing_trait_on_type::{SocialPost, Summary};
//use using_default_implementations_trait_on_type::{NewArticle, SocialPost, Summary};
use using_default_implementations_trait_on_type::{SocialPost, Summary};

// Use crate from validating_references_with_lifetimes mod
use crate::validating_references_with_lifetimes::{dangling_references, no_dangling_references};

fn main() {
    println!("====Generic Data Types===");
    // Finding the largest number in a list of number_list
    finding_the_largest_number();
    finding_the_largest_number_with_ex_func();

    // Generic Data Types In Function Definitions
    //function_definitions();

    // Generic Data Types In Struct Definitions
    struct_definition();

    // Generic Data Types In Method Definitions
    method_definition();

    // ---
    performance_using_generics();

    println!();

    // Defining Shared Behavior with Traits
    println!("===Defining Shared Behavior with Traits===");

    // Implementing a Trait on a Type
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };
    println!("1 new post: {}", post.summarize());

    println!();

    // Using Default Implementations
    /*
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship"),
        location: String::from("Pittersburgs, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguin onve again are the best hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
    */

    println!();
    // Validating Reference with lifetims
    println!("===Validating References with Lifetimes===");
    dangling_references();
    no_dangling_references();

    //validating_references_with_lifetimes::generic_lifetime_fn();
    validating_references_with_lifetimes::lifetime_fn_with_annotation();
}
