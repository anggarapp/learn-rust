// Basic Struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple Struct
struct RGB(i32, i32, i32);

fn main() {
    let _user_1 = User {
        email: String::from("email@mail.com"),
        username: String::from("laplace"),
        active: true,
        sign_in_count: 1,
    };
    // create struct using function
    let _user_2 = user_builder(String::from("test"), String::from("email"));

    // create struct from other struct value
    let _user_3 = User {
        email: String::from("email@mail.com"),
        username: String::from("laplace"),
        active: _user_1.active,
        sign_in_count: _user_1.sign_in_count,
    };
    // create struct, with else fields same with other struct fields
    let _user_4 = User {
        email: String::from("email@mail.com"),
        username: String::from("laplace"),
        .._user_3
    };
}

fn user_builder(user: String, email: String) -> User {
    User {
        username: user,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}
