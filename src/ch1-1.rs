pub fn variable() {
    println!("HI");

    let x: i8 = 10;
    let y: i8 = 20;

    println!("x:{} + y:{} = {}", x, y, x + y);

    let mut xx: i8 = 10;
    xx = xx + 20;

    println!("xx = {}", xx);

    let xxx: i32 = 10;

    println!("x:{} * x:{} = {}", xxx, xxx, square(xxx));

    // tuple
    let z: (i32, f64) = (10, 3.14);
    println!("{} {}", z.0, z.1);

    // array
    // let zz = [1, 2, 3];
    // println!("{}", zz);

    let zz = 1;
    let c = {
        let zz = 2;
        zz + 1
    };

    println!("{} {}", zz, c);

    let v = if zz == 1 {
        10
    } else if zz > 1 {
        20
    } else {
        30
    };

    println!("{}", v);

    match_practice(1);
    match_practice(3);
    match_practice(5);
    match_practice(7);

    // option
    let one: Option<i32> = Some(1);
    let _two = match_option(one);
    let _none = match_option(None);

    // println!("{} {} {}", one, two, none);
}

pub fn square(x: i32) -> i32 {
    x * x
}

pub fn match_practice(x: i8) {
    match x {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => (),
    }
}

pub fn match_option(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
