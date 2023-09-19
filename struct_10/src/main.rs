struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user_1 = User {
        email: String::from("email@mail.com"),
        username: String::from("laplace"),
        active: true,
        sign_in_count: 1,
    };
}
