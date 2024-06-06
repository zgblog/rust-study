fn main() {
    box_var();
}

fn box_var() { 
    let b = Box::new(5);
    println!("b={}", b);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn box_list() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
