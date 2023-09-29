mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}
        fn inner_secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    // outermost::middle_secret_function();  // not work because function is private
    // outermost::inside::inner_function(); // not work because module is private
    // outermost::inside::inner_secret_function(); // not work because module & function is private
}
