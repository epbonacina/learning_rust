// Variables can always be type annotaded. Numbers may additionally be anno-
// taded via a suffix or by default. Integers default to `i32` and floats to
// `f64`. Note that Rust can also infer types from context.


fn main(){
    let logical: bool = true;

    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation
    
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable i32
    mutable = 21;

    // Varibles can be overwritten with shadowing.
    let mutable = true;
}
