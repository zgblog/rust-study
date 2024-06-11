fn main() {
    drop_test1();
    drop_test2();
}

struct CustomSmartPointer {
    data: String
}

// leave life area, auto exec drop
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn drop_test1() {
    let c = CustomSmartPointer {data: String::from("my stuff")};
    let d = CustomSmartPointer {data: String::from("other stuff")};
    println!("CustomSmartPointers created.");
}

fn drop_test2() {
    let c = CustomSmartPointer {data: String::from("some data")};
    println!("CustomSmartPointer created.");
    // std::mem::drop, 提前调用drop
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
