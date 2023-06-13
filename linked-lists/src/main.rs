// A common way of implementing a linked-list is via `enums`.

use crate::List::*;


enum List{
    // Cons is a tuple struct that wraps an element and a pointer to the next node.
    Cons(u32, Box<List>),
    // Nil is a node that represents the end of the linked-list.
    Nil,
}

// Methods can be attached to an enum
impl List {
    fn new() -> List{
        // `Nil` has type `List`
        Nil
    }

    // Consumes a list and returns the same list with a new element at its front
    fn prepend(self, elem: u32) -> List{
        //`Cons` also has type `List`
        Cons(elem, Box::new(self))
    }

    // Returns the length of the list
    fn len(&self) -> u32{
        // `self` has to be matched, because the behavior of this method depends
        // on the variant of `self`.
        // `self` has type `&List`, and `*self` has type `List`. Matching on a
        // concrete type `T` is preferred over a match on referente `&T`.
        // After Rust 2018, you can use self here and tail (with no ref) below
        // as well. Rust will infer &s and ref tail.
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self{
            // Can't take ownership of the tail, because `self` is borrowed. Instead, take a
            // reference to the tail.
            Cons(_, ref tail) => 1 + tail.len(),
            // Base case, which is an empty list
            Nil => 0,
        }
    }


    // Returns the representation of the list as a (heap allocated) string
    fn stringify(&self) -> String{
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => { format!("Nil") }
        }
    }
}


fn main(){
    // Creates an empty linked list.
    let mut list = List::new();

    // Prepends some elements.
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Shows the final state of the list.
    println!("Linked-list length: {}", list.len());
    println!("{}", list.stringify());
}
