// `enum` can also be used as C-like enums.

// `enum` with implicit discriminator (starts at 0).
#![allow(dead_code)]


enum Number {
    Zero,
    One,
    Two
}

// `enum` with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}


fn main() {
    // `enum` can be cast as integers.
    println!("Zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);

    println!("Roses are #{:06x}", Color::Red as i32);
    println!("Violets are #{:06x}", Color::Blue as i32);
}
