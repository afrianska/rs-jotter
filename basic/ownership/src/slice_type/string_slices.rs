pub fn string_slice() {
    let s = String::from("Hello World");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello}");
    println!("{world}");

    let len = s.len();
    println!("{len}");

    let slice = &s[3..len];
    print_type_of(slice);
    println!("{slice}");

    let slice = &s[3..];
    println!("{slice}");

    let slice = &s[0..len];
    println!("{slice}");

    let slice = &s[..];
    println!("{slice}");

    first_word(&s);

    let test = first_word(&s);
    println!("{test}");

    let s1 = String::from("summerplace");
    let type_s1 = &s1;
    print_type_of(type_s1);

    let test1 = first_word(&s1);
    println!("{test1}");

    //let mut s2 = String::from("winter place");
    //let winter = firs_word(&s2);
    //s2.clear(); // error!
    //println!("the first word is: {winter}");

    let my_string = String::from("hello world");
    let type_my_string = &my_string[..];
    print_type_of(type_my_string);

    // 'fisrt_word' works on slice of 'String's, whether partial
    // or whole.
    let word2 = first_word2(&my_string[0..6]);
    println!("{word2}");
    let word2 = first_word2(&my_string[..]);
    println!("{word2}");
    // 'first_word' also works on reference to 'String's, which
    // are equivalent to whole slices of 'String's.
    let word2 = first_word2(&my_string);
    println!("{word2}");

    let my_string_literal = "hello world";
    let type_my_string_literal = &my_string_literal[..];
    print_type_of(type_my_string_literal);

    // 'first_word' works on slices of string literals
    // whether partial or whole.
    let word3 = first_word2(&my_string_literal[0..6]);
    println!("{word3}");
    let word3 = first_word2(&my_string_literal[..]);
    println!("{word3}");

    // Because string literals are string slices already
    // this works too, without the slice syntax
    let word3 = first_word2(my_string_literal);
    println!("{word3}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn print_type_of<T: ?Sized>(_: &T) {
    println!("the type is: {}", std::any::type_name::<T>())
}
