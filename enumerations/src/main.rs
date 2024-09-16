// ENUMERATIONS

#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

// mod sh;
use std::mem; // standart package for memory data

enum Color {
    Red,
    Blue,
    Green,
    RgbColor(u8, u8, u8), // tuple
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8}, // struct
}

fn enums () {
    let c1: Color = Color::Red;
    let c2: Color = Color::CmykColor{cyan:0, magenta:128, yellow:0, black:0};

    match c1 {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) | Color::CmykColor{cyan:_, magenta:_, yellow:_, black:255} => println!("black"),
        Color::RgbColor(r, g, b) => println!(" r, g, b ({}, {}, {})", r, g, b),
        _ => println!("some other color..."),
    }
}

fn main() {
    enums();
}