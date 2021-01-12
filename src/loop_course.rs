pub fn loops() {
    // loop {
    //     println!("Again!");
    // }

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    let a  = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The Value Is : {}", element);
    }
}