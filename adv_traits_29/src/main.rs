fn main() {
    _associated_type_on_trait();
}

fn _associated_type_on_trait() {
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    // impl Iterator for Counter {
    //     type Item = u32;

    //     fn next(&mut self) -> Option<Self::Item> {}
    // }

    // like

    // pub trait Iterator<T> {
    //     fn next(&mut self) -> Option<T>;
    // }
}
