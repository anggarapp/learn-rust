fn main() {
    //muttability
    let mut x = 5;
    println!("x value is {}", x);
    x = 6;
    println!("current x value is {}", x);

    //shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("current y value is {}", y);
}
