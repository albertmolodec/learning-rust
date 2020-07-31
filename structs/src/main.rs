struct User {
    username: String,
    sign_in_count: usize,
    active: bool,
}

struct Color(u8, u8, u8);

fn main() {
    let user = User {
        username: String::from("hello@aaa.aa"),
        sign_in_count: 5,
        active: true,
    };

    let user2 = User {
        username: String::from("hello@bb.bb"),
        ..user
    };

    let black = Color(255, 255, 255);
}
