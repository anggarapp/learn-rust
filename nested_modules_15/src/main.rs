pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;
use a::series::of::nested_modules;

fn main() {
    of::nested_modules(); // concise import
    a::series::of::nested_modules(); // full name call
    nested_modules(); // direct functin calling with concise import
}
