pub fn example1() {
    // no struct
    let length = 50;
    let width = 30;

    println!("The Area Of The Rectangle Is {} Square Pixels.", area(length, width));

    // use struct, dimensions
    let rect = (50, 30);
    println!("The Area Of The Rectangle Is {} Square Pixels.", area_dimensions(rect));

    // more information struct
    let rect2 = Rectangle { length: 50, width: 30 };
    println!("The Area Of The Rectangle Is {} Square Pixels.", area_struct(&rect2));

    // struct info print
    let rect3 = Rectangle { length: 50, width: 30 };
    println!("{:#?}", rect3);
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn area_dimensions(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}