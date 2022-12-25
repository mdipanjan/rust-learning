struct User {
    user_name: String,
    email: String,
}
fn main() {
    let mut user = User {
        user_name: String::from("mdipanjan"),
        email: String::from("mdipanjan@gmail.com"),
    };
    let name = user.user_name;
    user.email = String::from("user@gmail.com");

    println!("Hello {}", user.email);
}
