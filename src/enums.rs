enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrKind2 {
    V4(String),
    V6(String),
}

enum IpAddrKind3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr {

}

struct Ipv6Addr {

}

enum IpAddrKind4 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

pub fn enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    //////////////////////////////////////
    
    let home = IpAddrKind2::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind2::V6(String::from("::1"));

    //////////////////////////////////////
    
    let home = IpAddrKind3::V4(127, 0, 0, 1);
    let loopback = IpAddrKind3::V6(String::from("::1"));

    //////////////////////////////////////
    
    let m = Message::Write(String::from("Hello"));
    m.call();

    //////////////////////////////////////
    
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    //////////////////////////////////////
    // explain

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;
}

fn route(ip_type: IpAddrKind) {

}