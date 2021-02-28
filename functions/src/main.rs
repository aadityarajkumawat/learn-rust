fn main() {
    // another_function(5, 6);
    let _z = {
        let x = 3;
        x + 1
    };

    let five = five();
    let six = plus_one(five);

    println!("\nsix is: {}", six);
}

fn _another_function(x: i32, y: i32) {
    println!("The values of x and y are: {} and {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
