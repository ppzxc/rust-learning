pub fn vector_example() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    // how to running?
    let v = vec![1, 2, 3, 4, 5];

    // panic
    // let does_not_exist = &v[100];

    // option
    let does_not_exist = v.get(100);
    println!("{}", does_not_exist.is_none());

    // vector loop
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    // enum vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // println!("{:?}", *row);
}


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}