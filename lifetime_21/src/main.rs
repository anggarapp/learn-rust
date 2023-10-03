fn main() {
    let r;
    {
        let x = 5;
        r = &x;
        println!("r: {}", r);
    }
    // println!("r: {}", r); //the lifetime not long enough to borrowing here
}
