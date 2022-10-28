struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let user3 = build_user("minh@gmail.com".to_string(),"zero".to_string());
}