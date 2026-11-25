fn main() {
    // IF EXPRESSIONS
    // Basic expression
    println!("-- IF expressions: if else");
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    println!(" ");

    // handle multple expression
    println!("-- IF expressions: multiple");
    let number = 3;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    println!(" ");

    // IF in a let statement
    println!("-- IF expressions: using if in let statement");
    let condition = true;
    let number_x = if condition { 5 } else { 6 };
    println!("The value of number_x is: {number_x}");
    println!(" ");

    // LOOP
    // -- Loop: returning values from loops
    println!("-- LOOP: returning value from loops");
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
    println!(" ");

    // -- Loop: disambiguating with loop labels
    println!("-- LOOP:  disambiguating with loop labels");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    println!(" ");

    // WHILE
    // Streamlining Conditional Loops with while
    println!("-- WHILE");
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");
    println!(" ");

    // Looping Through a Collection
    println!("-- WHILE: Looping Through a Collection");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    println!(" ");

    // Looping Through a Collection with for
    println!("-- FOR: Looping Through a Collection with for");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    println!(" ");
}
