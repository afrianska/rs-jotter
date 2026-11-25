//  Two functions that differ only in their names and in the types in their signatures
/*
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

// The largest function using generic type parameters; this doesn’t compile yet
// THIS IS WILL BE ERROR
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn function_definitions() {
    let number_list = vec![34, 50, 25, 100, 65];
    //let result = largest_i32(&number_list);
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    //let result = largest_char(&char_list);
    let result = largest(&char_list);
    println!("The largest number is {result}");
}
