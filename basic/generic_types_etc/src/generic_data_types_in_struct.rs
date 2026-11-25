#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
struct Points<T, U> {
    x: T,
    y: U,
}

pub fn struct_definition() {
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };

    //let wont_work = Point { x: 6, y: 4.0 }; // This will not work because different data type.

    let _both_integer = Points { x: 5, y: 10 };
    let _both_float = Points { x: 1.0, y: 4.0 };
    let _integer_float = Points { x: 5, y: 4.0 };
}
