struct User {
    username: String,
    //username: &str, // 構造体は参照をそのままでは保持できない。ライフタイム？
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


fn main() {
    let email = String::from("someone@example.com");
    let username = String::from("someusername123");
    let mut user1 = build_user(email, username);

    user1.active = false;

    println!("Hello, {}", user1.username);
}
