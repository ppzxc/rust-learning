pub fn func() {
    println!("Hello World!");
    another_function();
    another_function2(5);
    another_function3(5, 10);
    // expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("{}", y);

    let z = five();
    println!("{}", z);

    let s = six(5);
    println!("{}", s);
}

// argument, return
fn six(s: i32) -> i32 {
    s + 1
}

// return function
fn five() -> i32 {
    5
}

fn another_function() {
    println!("another_function");
}

fn another_function2(x: i32) {
    println!("another_function : {}", x);
}

fn another_function3(x: i32, y: i32) {
    println!("another_function : {} {}", x, y);
}
