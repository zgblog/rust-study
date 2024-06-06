fn main() {
    deref_test1();
    deref_test2();
    deref_test3();
}

fn deref_test1() {
    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    // 需要解引用
    assert_eq!(*y, 5);
}

fn deref_test2() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn deref_test3() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
