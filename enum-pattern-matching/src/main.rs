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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Option::None => Option::None,
        Option::Some(i) => Option::Some(i + 1),
    }
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

    //println!("{:?}", absent_number);

    let k = value_in_cents(Coin::Quarter(UsState::Alaska));
    //println!("{}", k);
    
    let five = Option::Some(5);
    let six = plus_one(five);

    let none = plus_one(Option::None);

    let some_u8_value = Option::Some(3);
    //match some_u8_value {
      //  Some(3) => println!("three"),
      //  _ => (),
    //}
    if let Option::Some(3) = some_u8_value {
        println!("three");
    }
}

