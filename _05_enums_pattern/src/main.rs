enum IPAddressKind {
    V4(String),
    V6(String),
}

struct IPAddress {
    kind: IPAddressKind,
    ip_address: String,
}

fn main() {
    let four = IPAddressKind::V4;
    let six = IPAddressKind::V6;

    let localhost = IPAddress {
        kind: IPAddressKind::V4,
        ip_address: String::from("127.0.0.1"),
    };
}
