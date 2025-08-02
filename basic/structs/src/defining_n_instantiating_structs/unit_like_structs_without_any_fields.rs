struct AlwaysEqual;

pub fn unit_like_structs_without_any_fields() {
    let subject = AlwaysEqual;

    print_type_of(&subject);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
