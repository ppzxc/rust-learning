pub fn example() {
    let mut s = String::from("Hello World!");
    let word = first_world(&s);
    s.clear();

    // s string is null
    // but word is valid....

    // string slice pointer
    let s1 = String::from("Hello World!");

    let hello = &s1[0..5];
    let world = &s1[6..11];

    let s2 = String::from("hello");

    let slice = &s2[0..2]; // same
    let slice = &s2[..2]; // same

    let s3 = String::from("hello");
    let len = s3.len();
    let slice = &s3[0..len]; // same
    let slice = &s3[..]; // same
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_world2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
