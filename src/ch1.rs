mod ch1;
mod ch2;
mod ch3;

pub fn basic() {
    println!("Hello, world!");
    ch1::variable();
    ch2::ownership();
    ch2::references_and_borrowing();
    ch3::panic()
}