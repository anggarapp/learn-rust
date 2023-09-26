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
}

fn value_in_cent(coin: Coin) -> i32 {
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
