fn main() {
    let user1 = User {
        active: true,
        username: String::from("username1"),
        email: String::from("email1@domain.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        username: String::from("AnotherUsername2"),
        ..user1
    };

    println!("{}", user1.email); // E: 'user1.email' is owned by 'user2'
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

