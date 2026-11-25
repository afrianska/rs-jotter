mod defining_enum;
mod match_control_flow_construct;

use match_control_flow_construct::matching_option_t::plus_one;
use match_control_flow_construct::patterns_bind_to_values::{value_in_cents_new, CoinNew, UsState};
use match_control_flow_construct::value_in_cents::{value_in_cents, Coin};

fn main() {
    defining_enum::intro::intro();
    println!();

    defining_enum::enum_values::enum_style_1::enum_style_1();
    println!();

    defining_enum::enum_values::enum_style_2::enum_style_2();
    println!();

    defining_enum::enum_values::enum_style_3::enum_style_3();
    println!();

    defining_enum::enum_values::enum_style_4::enum_style_4();
    println!();

    println!("type embedded");
    defining_enum::enum_values::enum_style_5::enum_style_5();
    println!();

    println!("The Option Enum and Its Advantages Over Null Values");
    defining_enum::enum_option::enum_option_n_advantage_over_null();
    println!();

    println!("The match control flow Construct");
    let c = value_in_cents(Coin::Penny);
    println!("{:?}", c);
    println!();

    println!("The match control flow Construct: Patterns that bind to values");
    let z = value_in_cents_new(CoinNew::Quarter(UsState::Alaska));
    println!("{:?}", z);
    println!();

    println!("The match control flow Construct: Matching with Option T");
    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);
    println!("{:?}", six);
    println!();
}
