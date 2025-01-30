struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User { // can define struct instances as mutable
        email: String::from("someone@example.com"), // String, not &str, because we want the struct
        username: String::from("someusername123"),  // to own its values. Using &str would require lifetimes.
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,                   // shorthand for these two rows
        sign_in_count: user1.sign_in_count,     // in the user3 section
    };

    let user3 = User {
        email: String::from("yetanother@example.com"),
        username: String::from("thirdusername"),
        ..user1 // struct update syntax; "use the rest of the values from user1"
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

}
