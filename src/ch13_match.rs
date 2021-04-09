enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        // Coin::Quarter(state) => {
        //     println!("State Quarter From {:?}!", state);
        //     25
        // },
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...etc
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn example() {
    let five = Some(5);
    let six = plus_one(five);
    println!("{}", six.unwrap());
    let none = plus_one(None);
    println!("{}", none.is_none());

    // boilerplate
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (), // boilerplate
    }

    // equal boilerplate code
    if let Some(3) = some_u8_value {
        println!("three")
    }

    // boilerplate2
    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State Quarter from {:?}!", state),
    //     _ => count += 1,
    // }
    // let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //         Coin::Quarter(state) => println!("State Quarter from {:?}!", state),
    // }
}