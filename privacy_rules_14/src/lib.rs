mod outermost {
    pub fn middle_function() {}

    pub fn middle_secret_function() {} //fix

    pub mod inside {
        //fix
        pub fn inner_function() {}
        pub fn inner_secret_function() {} //fix
    }
}

pub fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function(); // not work because function is private
    outermost::inside::inner_function(); // not work because module is private
    outermost::inside::inner_secret_function(); // not work because module & function is private
}
