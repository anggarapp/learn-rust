struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let _user_1 = User {
        email: String::from("email@mail.com"),
        username: String::from("laplace"),
        active: true,
        sign_in_count: 1,
    };
    let _user_2 = user_builder(String::from("test"), String::from("email"));
}

fn user_builder(user: String, email: String) -> User {
    User {
        username: user,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}
