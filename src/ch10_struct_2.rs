pub fn structs() {
    // 1
    let length1 = 50;
    let width1 = 30;

    println!(
        "The Area Of The Rectangle Is {} Square Pixels.",
        area(length1, width1)
    );

    // 2
    let rect1 = (50, 30);

    println!(
        "The Area Of The Rectangle Is {} Square Pixels.",
        area2(rect1)
    );

    // 3
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    println!(
        "The Area Of The Rectangle Is {} Square Pixels.",
        area3(&rect1)
    );
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
