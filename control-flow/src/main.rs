fn main() {
    // let number = 3;

    // if number < 4 {
    //     println!("Condition is true");
    // } else {
    //     println!("Condition is false");
    // }

    // // Since if is an expression

    // let num = if true { 5 } else { 7 };

    // println!("Num is: {}", num);

    // let mut num = 0;
    // loop {
    //     num = num + 1;
    //     println!("{}", num);
    //     if num > 9 {
    //         break;
    //     }
    // }

    let a = [1, 2, 3, 4, 5, 6];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for num in 1..4 {
        println!("the num is: {}", num);
    }
}
