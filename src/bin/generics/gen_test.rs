use std::fmt::Display;

fn main() {
    let list = vec![1, 2, 3, 4, 5];
    let max = largest(&list);

    println!("{}", max);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/**
 * 泛型
 */
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest { // TODO
            largest = item;
        }
    }
    largest
}

/**
 * 泛型结构体
 */
struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

/**
 * 示例 10-16：根据 trait bound 在泛型上有条件的实现方法
 */
struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair {
            x,
            y
        }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if(self.x >= self.y) {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

