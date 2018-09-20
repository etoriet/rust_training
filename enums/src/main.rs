enum IpAddrKind {
    V4,
    V6
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    let v4 = IpAddrKind::V4;

    println!("Hello, world");
}
