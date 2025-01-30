struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}

fn main() {
    let mut user1 = User { // can define struct instances as mutable
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
