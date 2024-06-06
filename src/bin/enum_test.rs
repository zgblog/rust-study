enum IpAddrKind {
    V4,
    V6
}

enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl IpAddrKind2 {
    fn call(self) {}
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
}

fn route(ip_kind: IpAddrKind) {

}
