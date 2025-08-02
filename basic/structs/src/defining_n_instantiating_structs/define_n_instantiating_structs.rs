struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn define_n_instantiating_structs() {
    let user1 = User {
        active: true,
        username: String::from("artemis"),
        email: String::from("artemis@x.com"),
        sign_in_count: 1,
    };
    println!(
        "{0}, {1}, {2}, {3}",
        user1.active, user1.username, user1.email, user1.sign_in_count,
    );

    let mut user2 = User {
        active: true,
        username: String::from("artemis"),
        email: String::from("artemis@x.com"),
        sign_in_count: 1,
    };
    user2.active = false;
    user2.username = String::from("athena");
    user2.email = String::from("athena@x.com");
    user2.sign_in_count = 2;
    println!(
        "{0}, {1}, {2}, {3}",
        user2.active, user2.username, user2.email, user2.sign_in_count,
    );

    let user3 = create_user("zeus".to_string(), "zeus@x.com".to_string());
    println!(
        "{0}, {1}, {2}, {3}",
        user3.active, user3.username, user3.email, user3.sign_in_count,
    );
}

fn create_user(username: String, email: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
