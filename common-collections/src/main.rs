enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let _third: &i32 = &v[2];
    // println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }

    // let ifs: Option<i32> = Option::Some(45);

    let mut userinput = get_user_input(4);

    userinput = Option::None::<i32>;

    match userinput {
        Some(num) => {
            // ifs = Option::Some(num);
            println!("The num is {}", num);
        }
        None => {
            // ifs = Option::None;
            println!("The num doesn't exist");
        }
    };

    // println!("The first element is: {}");

    let newv = vec![100, 32, 57];
    for i in &newv {
        println!("{}", i);
    }

    let mut vv = vec![100, 32, 57];
    for i in &mut vv {
        *i += 50;
    }

    for i in &vv {
        println!("{}", i);
    }

    // To store different types in a vector use an enum to wrap those types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // match row.get(0) {
    //     Some(sm) => println!("{}", sm),
    //     None => println!("cool im none"),
    // }
}

fn get_user_input(x: i32) -> Option<i32> {
    Option::Some(x)
}
