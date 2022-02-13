#![allow(dead_code)]

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff_00_00,
    Green = 0x00_ff_00,
    Blue = 0x00_00_ff,
}

fn main() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}