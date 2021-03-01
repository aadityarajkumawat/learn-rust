struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// This is called implementation block
// I think this is something similar
// to classes and self is like `this`
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someuser123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
      email: String::from("arkumawat78@gmail.com"),
      username: String::from("aditya98"),
      ..user1
    };

    let black = Color(0, 0, 0); // black color
    let origin = Point(0, 0, 0); // relative center

    let width1 = 30;
    let height1 = 50;

    let rect1 = (30, 50);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    let r1 = Rectangle {
        width: 30,
        height: 50,
    };

    let r2 = Rectangle {
        width: 10,
        height: 40,
    };

    let r3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", r1.can_hold(&r2));
    println!("Can rect1 hold rect2? {}", r1.can_hold(&r3));

    //println!("The area of rectangle is {} square pixels.", area(&rect1));
    println!("The area of rectangle is {:#?} square pixels.", rect2);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
  User {
      email,
      username,
      active: true,
      sign_in_count: 1,
  }
}

