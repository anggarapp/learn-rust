enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("0.0.0.0"),
    };

    let loopback: IpAddr = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
