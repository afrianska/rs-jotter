fn main() {
    // ------ RUST has 2 type data: Scalar and Compound
    // SCALAR TYPES

    // -- Scalar: Integer type
    // Integer type has signed type: i8, i16, i32, i64, i128 and isize
    // Integer type has unsigned type: u8, u16, u32, u64, u128 and usize
    let data_integer_u32: u32 = "42".parse().expect("Not a number!");
    let data_integer_i8: i8 = 42;
    println!("-- Scalar: Integer");
    println!("{data_integer_u32} and {data_integer_i8}");
    println!(" ");

    // -- Scalar: Floating-point type
    // Floating-point type has type: f32 and f64
    let data_floating_f32: f32 = 3.0;
    let data_floating_not_set = 2.0;
    println!("-- Scalar: Floating-point");
    println!("{data_floating_f32} and {data_floating_not_set}");
    println!(" ");

    // -- Scalar: Boolean type
    let data_bool_t = true;
    let data_bool_f = false;
    println!("-- Scalar: Boolean");
    println!("{data_bool_t} and {data_bool_f}");
    println!(" ");

    // -- Scalar: Char type
    // Char is single unicode scalar value (represent one character)
    // Meanwhile string is sequence of unicode characters (text)
    let data_c = 'z';
    let data_char: char = 'Z';
    let data_char_heart_eyed_cat = '😻';
    println!("-- Scalar: Char or Character type");
    println!("{data_c} and {data_char} and {data_char_heart_eyed_cat}");
    println!(" ");

    // ---------------------------
    // COMPOUND TYPES

    // -- Compound: Tuple type, variaty type of scalar in one group
    let data_tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = data_tup; // bind to variable
    let five_hundred = data_tup.0; // can access directly
    println!("-- Compound: Tuple type");
    println!("The value of y is {y}");
    println!("The value of five_hundred is {five_hundred}");
    println!(" ");

    // -- Compound: Array type, have to same type data of scalar in one group
    let data_arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("-- Compound: Array type");
    println!("{}", data_arr[0]);
    println!(" ");
}
