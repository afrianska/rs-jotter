mod defining_enum;

fn main() {
    defining_enum::intro::intro();
    println!("");

    defining_enum::enum_values::enum_style_1::enum_style_1();
    println!("");

    defining_enum::enum_values::enum_style_2::enum_style_2();
    println!("");

    defining_enum::enum_values::enum_style_3::enum_style_3();
    println!("");

    defining_enum::enum_values::enum_style_4::enum_style_4();
    println!("");

    println!("type embedded");
    defining_enum::enum_values::enum_style_5::enum_style_5();
    println!("");

    println!("The Option Enum and Its Advantages Over Null Values");
    defining_enum::enum_option::enum_option_n_advantage_over_null();
    println!("");
}
