fn main() {
    box_var();
}

fn box_var() { 
    let b = Box::new(5);
    println!("b={}", b);
}

enum List {
    Cons(i32, 
