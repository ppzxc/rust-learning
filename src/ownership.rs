// 정수형과 같이 컴파일 타임에 결정되어 있는 크기의 타입은 스택에 모두 저장된다.
// String과 같이 컴파일 타임에 결정 되지 않는 크기의 타입은 clone을 명시적으로 호출해야 깊은 복사가 이루어진다.
// 따라서 clone을 호출하지 않은 복사는 복사가 아니라 move 이동이다.
// Copy Trait이 있는 타입은 모두 대입 과정 이후 예전 변수를 사용할 수 있으며, Move 이동을 통해 상호작용한다.
pub fn ownership_learning() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1={} s2={}", s1, s2);

    let i1 = 5;
    let y = i1;

    println!("i1 = {} y = {}", i1, y);

    /////////////////////////////////////
    let s = String::from("Hello");    // open s variable
    takes_ownership(s);                      // close s variable in scope
    // println!("s = {}", s);                       // error
    let x = 5;                           // open x variable
    makes_copy(x);                           // deep copy
    println!("x = {}", x);

    /////////////////////////////////////
    let ss1 = gives_ownership();
    let ss2 = String::from("Hello");
    let ss3 = takes_and_gives_back(s2);

    ////////////////////////////////////
    let sss1 = String::from("Hello");
    let (sss2, len) = calculate_length(sss1);
    println!("The Length Of '{}' is {}.", sss2, len);
}

pub fn takes_ownership(s: String) {  // open s variable
    println!("s = {}", s);              
}                                    // s drop, memory free

pub fn makes_copy(x: i32) {
    println!("x = {}", x);
}

fn gives_ownership() -> String {             // gives_ownership 함수가 반환 값을 호출한 쪽으로 이동시킵니다.
    let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.
    some_string                              // some_string이 반환되고, 호출한 쪽의 함수로 이동됩니다.
}

// takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환합니다.
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프 안으로 들어왔습니다.
    a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()함수는 문자열의 길이를 반환합니다.
    (s, length)
}