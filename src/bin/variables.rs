
const MAX_POINTS: u32 = 255;    // 常量

fn testVariable() {
    println!("Hello, world!");

    let mut x = 5;  // 可变
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    println!("The value of MAX_POINTS is {}", MAX_POINTS);

    // 默认为常量
    let x = 1;
    let y = 2.0;

    let x = 'A';
    let y: char = '1';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    tup.0;

    let arr: [u32; 3] = [1, 2, 3];

    // 常量
    const AA: i32 = 1;

    println!("{}", AA);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("{}", spaces);
}

fn testType() {
    let guess: u32 = "42".parse().expect("Not  a number");
    println!("{}", guess);
    // 标量：整数、浮点、布尔、字符
    let a1: u8 = 1;
    let b1: f32 = 2.0;
    let c1: bool = false;
    let d1: char = 'x';
    // 复合类型：元组和数组
    // 1. tuple 其中类型可以不同
    let t1: (i32, f64, u8) = (500, 6.4, 1);
    // 解构
    let (x, y, z) = t1;
    println!("{}, {}, {}, ", t1.0, t1.1, t1.2);

    // 2. array 其中类型必须相同
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    let a2 = [3; 5];

    let first = a1[0];

    let second = a1[1];

}

fn main() {
    testVariable();
    testType();
}
