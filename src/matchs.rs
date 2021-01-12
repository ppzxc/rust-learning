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
    }
}

///////////////////////////////////////////////

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... ETC
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(coin: Coin2) -> u32 {
    match coin {
        Coin2::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State Quarter From {:?}!", state);
            25
        },
    }
}

//////////////////////////////////////////////////////////////////////
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn matchs() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    //////////////////////////////////////////////////
    // if let
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // same
    if let Some(3) = some_u8_value {
        println!("Three");
    }
    // another case
    let mut count = 0;
    let coin = Coin2::Quarter(UsState::Alaska);
    match coin {
        Coin2::Quarter(state) => println!("State Quarter From {:?}!", state),
        _ => count += 1,
    }
    // same case
    let mut count = 0;
    let coin = Coin2::Quarter(UsState::Alabama);
    if let Coin2::Quarter(state) = coin {
        println!("State Quarter From {:?}!", state);
    } else {
        count += 1;
    }
}