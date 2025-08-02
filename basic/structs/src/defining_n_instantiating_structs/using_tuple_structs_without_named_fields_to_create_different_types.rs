struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn using_tuple_structs_without_named_fields_to_create_different_types() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{0}, {1}, {2}", black.0, black.1, black.2);
    println!("{0}, {1}, {2}", origin.0, origin.1, origin.2);
}
