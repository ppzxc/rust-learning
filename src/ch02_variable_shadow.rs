const MAX_CHAPTERS: u32 = 100_000; // 상수는 const 키워드로 생성

pub fn mut_variable() {
    // mut keywords는 불변을 변경 가능한 변수로 만듦
    let mut x = 5;
    println!("The Value Of X is : {}", x);
    x = 6;
    println!("The Value Of X is : {}", x);

    let y = 5;
    let y = y + 1; // shadowing

    println!("The Value Of Y is : {}", y);

    let spaces = "       ";
    let spaces = spaces.len(); // shadowing
    println!("Shadowed spaces variable is : {}", spaces);

    // let z = 3;
    // z = 8; // errors
}
