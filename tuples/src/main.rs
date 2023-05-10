// A tuple is a collection of values of different types. Tuples are
// constructed using parentheses `()`, and each tuple itself is a
// value with type signature (T1, T2, ...), where T1, T2 are types
// of its members. Functions can use tuples to return multiple values,
// as tuples can hold any number of values.


// Tuples can be used as function arguments and as return values.

use std::fmt::{self, Display, Formatter};

fn reverse(pair: (i32, bool)) -> (bool, i32){
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}


fn transpose(matrix: Matrix) -> Matrix{
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}


// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        write!(
            f,
            "( {idx_0} {idx_1} )\n( {idx_2} {idx_3} )",
            idx_0 = self.0, idx_1 = self.1,
            idx_2 = self.2, idx_3 = self.3
        )
    }
}


fn main(){
    // A tuple with a bunch of different types.
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("Tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed.
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error
    
    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them
    // apart from a literal surrounded by parantheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructed to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?} {:?} {:?} {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);

    println!("Matrix:\n{}", matrix);
    println!("Transposed matrix:\n{}", transpose(matrix));
}
