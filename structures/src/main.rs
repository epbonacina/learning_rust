// There are three types of structures ("structs") that can be created
// using the `struct` keyword:
// - Tuple structs, which are, basically, named tuples;
// - The classic C structs and
// - Unit structs, which are field-less and usefull for generics.

// An attribute to hide warnings for unused code.
 #![allow(dead_code)]

#[derive(Debug)]
struct Person{
    name: String,
    age: u8,
}

// A unit `struct` 
struct Unit;

// A tuple `struct`
struct Pair(i32, f32);

// A `struct` with two fields
struct Point{
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another `struct`
struct Rectangle{
    // A rectangle can be specified by where the top left and bottom
    // right corners are in space.
    top_left: Point,
    bottom_right: Point,
}


impl Rectangle {
    fn area(&self) -> f32 {
        let Rectangle { 
            top_left: Point { x: lx, y: ly },
            bottom_right: Point { x: rx, y: ry }
        } = self;
        let width = rx - lx;
        let height = ry - ly;
        width * height
    }

    fn square(top_left: Point, edge: f32) -> Rectangle{
        let bottom_right = Point { x: top_left.x + edge, y: top_left.y - edge };
        Rectangle { top_left: top_left, bottom_right: bottom_right }
    }
}


fn main(){
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug `struct`
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("Point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of
    // our other one
    let bottom_right = Point {x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y`, because we used that
    // field from `point`
    println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let rectangle = Rectangle{
        // `struct` instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    println!("Rectangle are: {}", rectangle.area());

    // Instantiate a unit `struct`
    let _unit = Unit;

    // Instantiate a tuple `struct`
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple `struct`
    println!("Pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("Pair contains {:?} and {:?}", integer, decimal);

    let edge = 3.5;
    let top_left = Point { x: 3.4, y: 2.2 };
    let square = Rectangle::square(top_left, edge);
    println!("Square ( x: {}, y: {}, edge: {} )", square.top_left.x, square.top_left.y, edge);
}
