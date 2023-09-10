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

    let spaces = "    ";
    let spaces = spaces.len();
    println!("lenght of space is {}", spaces);

    // constant
    const MAX_POINTS: u32 = 100_000;
    println!("current MAX_POINT value is {}", MAX_POINTS);

    // int type
    let _decimal = 98_000;
    let _hex = 0xffffff;
    let _octal = 0o66;
    let _binary: i32 = 0b1110011;
    let _byte: u8 = b'X'; //u8 only

    // floating point type
    let _fp_default = 10.2;
    let _fp_f32: f32 = 1.2;

    // numeric operation
    let _addition = 5 + 9;
    let _substraction = 10 / 2;
    let _multiplication = 6 * 6;
    let _division = 20 / 4;
    let _remainder = 21 % 4;
}
