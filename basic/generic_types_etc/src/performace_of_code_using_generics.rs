#[derive(Debug)]
#[allow(dead_code)]
enum Optioni32 {
    Some(i32),
    None,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Optionf64 {
    Some(f64),
    None,
}

pub fn performance_using_generics() {
    let integer = Optioni32::Some(5);
    let float = Optionf64::Some(5.0);

    println!("integer: {integer:?}, float: {float:?}");
}
