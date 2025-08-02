// Scalar types: integers, floating-point numbers, Booleans, and characters.

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

// Integer
pub fn integer_types() {
    // Integer signed
    let signed_8bit: i8 = 127;
    let signed_16bit: i16 = 32767;
    let signed_32bit: i32 = 2147483647;
    let signed_64bit: i64 = 9223372036854775807;
    let signed_128bit: i128 = 170141183460469231731687303715884105727;
    println!("max value of signed integer:");
    println!("i8: {signed_8bit}, i16: {signed_16bit}, i32: {signed_32bit}, i64: {signed_64bit}, i128: {signed_128bit}");

    // Integer unsigned
    let unsigned_8bit: u8 = 255;
    let unsigned_16bit: u16 = 65535;
    let unsigned_32bit: u32 = 4294967295;
    let unsigned_64bit: u64 = 18446744073709551615;
    let unsigned_128bit: u128 = 340282366920938463463374607431768211455;
    println!("max value of unsigned integer:");
    println!("u8: {unsigned_8bit}, u16: {unsigned_16bit}, u32: {unsigned_32bit}, u64: {unsigned_64bit}, u128: {unsigned_128bit}");
}

pub fn integer_literals() {
    let decimal = 98_2222; // still understand
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // only u8
    println!("decimal: {decimal}, hex: {hex}, octal: {octal}, binary: {binary}, byte: {byte}");
}

pub fn floating() {
    let x = 2.0;
    print_type_of(&x);

    let y: f32 = 3.0;
    print_type_of(&y);
}

pub fn numeric_operation() {
    // addition
    let sum = 5 + 10;
    println!("addition of 5 + 10 = {sum}");

    // substraction
    let difference = 95.5 - 4.3;
    println!("substraction of 95.5 - 4.3 = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("multiplication of 4 * 30 = {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("division of 56.7 / 32.2 = {quotient}");
    println!("division of -5 / 3 = {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder of 43 % 5 = {remainder}");
}

pub fn boolean() {
    let t = true;

    // with explicit type annotation
    let f: bool = false;

    println!("value of t: {t}, f: {f}");
    print_type_of(&t);
}

pub fn character() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{c}");
    println!("{z}");
    println!("{heart_eyed_cat}");
    print_type_of(&heart_eyed_cat);
}
