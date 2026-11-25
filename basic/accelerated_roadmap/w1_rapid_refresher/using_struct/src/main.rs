// Defining and instantiating Struct
// Example a struct that stores information about a user account
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Creating different types with Tuples Structs
#[allow(dead_code)]
struct Color(i32, i32, i32);

#[allow(dead_code)]
struct Point(i32, i32, i32);

fn main() {
    // create an instance of that struct
    let mut user1 = User {
        // put mut for mutable
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("user1 email: {0}", user1.email);

    // change a value
    user1.email = String::from("anotherperson@example.com");
    println!("user1 email update 01: {0}", user1.email);

    // create an instance of struct with function
    let user2 = build_user("haha123".to_string(), "haha123@example.com".to_string());
    println!(
        "user2 email: {0} and username: {1}",
        user2.email, user2.username
    );

    // Creating Instances with Struct Update Syntax
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("update@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("user1 -> user3 email update 02: {0}", user3.email);

    // Another shape
    let user4 = User {
        email: String::from("another_update@example.com"),
        ..user3
    };
    println!("user3 -> user4 email update 03: {0}", user4.email);

    // create instance from Tuple Struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{0} and {1}", black.0, origin.2);
}

// Using the Field Init Shorthand
fn build_user(email_field: String, username_field: String) -> User {
    User {
        active: true,
        username: username_field,
        email: email_field,
        sign_in_count: 1,
    }
}
