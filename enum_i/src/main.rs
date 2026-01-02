enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// alternative i
/* enum IpAddr {
    V4(Stirng),
    V6(String),
} */

// alternative ii
/* enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
} */

// alternative iii
/*struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
} */

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // alternative i
    // let home2 = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback2 = IpAddr::V6(String::from("::1"));

    // alternative ii
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));
}

fn route(ip_kind: IpAddrKind) {}
