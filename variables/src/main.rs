fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // Shadowing
    let x = 5;
    let x = x + 1;
    let _x = x * 2;

    let spaces = "   ";
    let spaces = spaces.len();

    let mut _spaces1 = "   ";
    // This throws an error, coz data type of let mut -> does not change
    // spaces1 = spaces1.len();

    println!("{}", spaces);
}
