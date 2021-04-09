pub fn string_example() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    let mut s1 = String::from("foo");
    s1.push_str("bar");

    let mut s2 = String::from("foo");
    let s3 = "bar";
    s2.push_str(&s3);
    println!("s2 is {}", s2);

    // loop
    for c in "안녕하쇼".chars() {
        println!("{}", c);
    }

    for b in "안녕하쇼".bytes() {
        println!("{}", b);
    }
}