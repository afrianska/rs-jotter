struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn using_the_field_init_shorthand() {
    let user = create_user("aphrodite".to_string(), "aphrodite@x.com".to_string());
    println!(
        "{0}, {1}, {2}, {3}",
        user.active, user.username, user.email, user.sign_in_count
    );
}

fn create_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
