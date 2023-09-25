enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home: IpAddr = IpAddr::V4(0, 0, 0, 0);

    let loopback: IpAddr = IpAddr::V6(String::from("::1"));
}
