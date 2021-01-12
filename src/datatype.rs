pub fn datatypes() {
    let guess: u32 = "42".parse().expect("Not A Number!");

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("{} {} {} {} {} {}", guess, sum, difference, product, quotient, remainder);

    let t = true;
    let f: bool = false;

    println!("{} {}", t, f);

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    println!("{} {} {}", c, z, heart_eyed_cat);

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);

    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup2.0;
    let six_point_four = tup2.1;
    let one = tup2.2;

    println!("{} {} {}", five_hundred, six_point_four, one);

    let a = [1, 2, 3, 4, 5];
    let months = ["January", "Febuary", "March", "April", "May", "June", "July", "August", "September", "October", "Nuvember", "December"];

    let first = a[0];
    let second = a[1];

    println!("Hello World!");

    another_function(5);
    another_function2(5, 6);

    // expression
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The Value Of Y Is : {}", y);

    let fiveNumber = five();
    println!("The Value Of X Is : {}", fiveNumber);

    let plus = plus_one(5);
    println!("The Value Of Plus Is {}", plus);
}


fn another_function(x: i32) {
    println!("Another Function. {}", x);
}

fn another_function2(x: i32, y: i32) {
    println!("Another Function. {} {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}