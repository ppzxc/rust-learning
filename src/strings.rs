pub fn strings() {
    // let mut s = String::new();

    // shadowing string
    let data = "initial contents";
    let s = data.to_string();

    // directly
    let s = "initial contents".to_string();

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}