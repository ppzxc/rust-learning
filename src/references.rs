pub fn reference() {
    // immutable borrowing
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The Length Of '{}' is {}", s1, len);

    // mutable borrowing
    let mut s = String::from("Hello");
    change(&mut s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", World");
}