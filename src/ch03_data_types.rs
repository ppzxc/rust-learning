pub fn types() {
    let guess: u32 = "42".parse().expect("Not a number!");
    // 8-bit       i8         u8
    // 16-bit      i16        u16
    // 32-bit      i32        u32
    // 64-bit      i64        u64
    // arch        isize      usize

    // Decimal          98_222
    // Hex              0xff
    // Octal            0o77
    // Binary           0b1111_0000
    // Byte (u8 only)   b'A'

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    // char is smal apostrophe
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The Value Of Y is : {}", y);

    let q: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = q.0;
    let six_point_for = q.1;
    let one = q.2;
    println!("The Value Of Y is : {}", q.0);

    let array = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let first = array[0];
    let second = array[1];
}
