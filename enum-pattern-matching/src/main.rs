// Enum: Also called enumeration

enum IpAddrKind {
    V4,
    V6,
}

// This allows us eliminating
// the struct after it
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct IpAddr1 {
    kind: IpAddrKind,
    address: String,
}

impl Message {
    fn call(&self) {
        // call
    }
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    //let home = IpAddr {
      //  kind: IpAddrKind::V4,
      //  address: String::from("127.0.0.1"),
    //};

   // let loopback = IpAddr {
     //   kind: IpAddrKind::V6,
     //   address: String::from("::1"),
    //};

    //let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let another_number = Some(10);
    let some_string = Some("a string");

    let absent_number: Option<i32> = Option::None;

    println!("{:?}", absent_number);
}

