#[derive(Debug)]
enum IpAddrKind {
    v4,
    v6,
}


fn route(ip_type: &IpAddrKind) {
    println!("got_type: {:#?}",ip_type)
}

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;
    route(&four);
    println!("4:{:#?}", four);
    println!("6:{:#?}", six);
}
