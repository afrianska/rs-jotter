// The problem with null values is that if you try to use a null value as a
// not-null value, you’ll get an error of some kind. Because this null or not
// null property is pervasive, it’s extremely easy to make this kind of error.

// Rust does not have nulls, but it does have an enum that
// can encode the concept of a value being present or absent.

//  <T> means that the Some variant of the Option enum can hold one piece of data of any type

#[derive(Debug)]
enum Option<T> {
    None,
    Some(T),
}

pub fn enum_option_n_advantage_over_null() {
    let some_number = Option::Some(5);
    let some_char = Option::Some('e');

    let absent_number: Option<i32> = Option::None;

    println!("{:?}", some_number);
    println!("{:?}", some_char);
    println!("{:?}", absent_number);

    // this is will be error
    //let x: i8 = 5;
    //let y: Option<i8> = Option::Some(5);
    //let sum = x + y;
}

// NOTE: currenty I don't understand yet.
