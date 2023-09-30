fn main() {
    let mut _vectors_from_vec: Vec<i32> = Vec::new();
    let _vectors_from_macro = vec![1, 2, 3];

    _vectors_from_vec.push(5);
    _vectors_from_vec.push(6);
    _vectors_from_vec.push(7);
    _vectors_from_vec.push(8);

    let _first: &i32 = &_vectors_from_vec[0];
    let _first_option: Option<&i32> = _vectors_from_vec.get(0);

    // let dosnt_exist = &_vectors_from_vec[100]; // error because accesing value doesnt exist
    let _dosnt_exist = _vectors_from_macro.get(100); // not error because return Option -> None

    // Borrows Rules Error
    // let mut _value = vec![1, 2, 3, 4, 5];
    // let _first_value = &v[0];
    // v.push(6);
}
