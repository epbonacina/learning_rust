// Rust's formatting functionality is implemented
// via traits, and there is one trait for each
// argument type. The most common trait is `Display`,
// which handles cases where the argument type is
// left unespecified: `{}` for instance.

// The same variable can be formatted differently
// depending on which argument type is used.

// For a full list of formatting traits, you can access this link:
// https://doc.rust-lang.org/std/fmt/#formatting-traits

use std::fmt::{self, Formatter, Display};

struct City{
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted
    // string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the
        // formatted string into a buffer (the first argument).
        write!(
            f, "{}: {:.3}º{} {:.3}º{}",
            self.name, 
            self.lat.abs(), 
            lat_c, 
            self.lon.abs(), 
            lon_c
        )
    } 
}

#[derive(Debug)]
struct Color{
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        write!(
            f,
            "RGB ({red}, {green}, {blue}) 0x{red:0>2X}{green:0>2X}{blue:0>2X}",
            red = self.red,
            green = self.green,
            blue = self.blue,
        )
    }
}


fn main(){
    let cities = [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ];
    
    let colors = [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ];

    for city in cities.iter(){
        println!("{}", *city);
    }
    
    for color in colors.into_iter(){
        println!("{:?} -> {} {:p}", color, color, &color);
    }
}

