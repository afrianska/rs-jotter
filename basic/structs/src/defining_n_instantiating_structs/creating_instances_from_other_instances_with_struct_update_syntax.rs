struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn creating_instances_from_other_instances_with_struct_update_syntax() {
    let user_1 = User {
        active: true,
        username: String::from("hera"),
        email: String::from("hera@x.com"),
        sign_in_count: 1,
    };
    println!(
        "{0}, {1}, {2}, {3}",
        user_1.active, user_1.username, user_1.email, user_1.sign_in_count
    );

    let user_2 = User {
        active: user_1.active,
        username: user_1.username,
        email: String::from("another@example.com"),
        sign_in_count: user_1.sign_in_count,
    };

    println!(
        "{0}, {1}, {2}, {3}",
        user_2.active, user_2.username, user_2.email, user_2.sign_in_count
    );

    let user_3 = User {
        username: String::from("demeter"),
        ..user_2
    };

    println!(
        "{0}, {1}, {2}, {3}",
        user_3.active, user_3.username, user_3.email, user_3.sign_in_count
    );
}
