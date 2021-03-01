struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
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

    println!("{}", black.0);
}

fn build_user(email: String, username: String) -> User {
  User {
      email,
      username,
      active: true,
      sign_in_count: 1,
  }
}

