enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Struct QuitMessage;
// Struct MoveMessage{
//     x: i32,
//     y: i32,
// };
// Struct WriteMessage(String);
// Struct ChangeColorMessage(i32,i32,i32);

// enum Message {
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32,i32,i32),
// }
#[derive(Debug)]
enum IdState {
    EastJava,
    WestJava,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(IdState),
}

fn main() {
    let _home: IpAddr = IpAddr::V4(0, 0, 0, 0);
    let _loopback: IpAddr = IpAddr::V6(String::from("::1"));

    let _five = Some(5);
    let _six = _plus_one(_five);
    let _none = _plus_one(None);

    _other_pattern(2);
    _other_pattern(10);

    // match and if let comparision

    let some_u8 = Some(0u8);

    match some_u8 {
        Some(3) => println!("Tiga"),
        _ => (),
    }

    if let Some(3) = some_u8 {
        println!("Tiga");
    }
}

fn _value_in_cent(coin: Coin) -> i32 {
    match coin {
        Coin::Dime => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Penny => 10,
        Coin::Quarter(state) => {
            println!("State Quarter From {:?}!", state);
            25
        }
    }
}

fn _plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn _other_pattern(x: u8) {
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => (),
    }
}
