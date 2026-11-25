fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    println!("{}", word);
    s.clear();

    // String Slices
    // A string slice is a reference to a contiguous sequence of the elements of a String
    let z = String::from("hello world");
    let _hello = &z[0..5];
    let _world = &z[6..11];

    // With function
    let new_word = first_word_new(&z);
    println!("the first word is: {new_word}");

    // String literals as slices
    let _g = "Hello, world!"; // the type of g in here is &str. it's immutable reference.
    let _gh: &str = "Hello, world!";

    // Other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    for element in slice {
        println!("{element}");
    }
    assert_eq!(slice, &[2, 3, 4]);
}

// functiion without using slices
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// rewrite function abouve with [x..x]
fn first_word_new(s: &String) -> &str {
    //String slices as parameters
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        println!("{i} --> {item}");
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
