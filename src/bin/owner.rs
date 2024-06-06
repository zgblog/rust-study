fn main() {
    let mut s = String::from("Hello");

    s.push_str(", World");

    println!("{}", s);

    move_test();

    clone_test();

    let s = String::from("Hello World");

    tale_ownership(s);

    let x = 5;

    make_copy(x);

    println!("{}", x);
    //
    let s1 = gives_ownership(); // 从函数结果中获取返回所有权

    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);
}

fn move_test() {
    let s1 = String::from("Hello");
    let s2 = s1;    // s1失去所有权

    println!("{}", s2);
}

fn clone_test() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("{}, {}", s1, s2);
}

fn tale_ownership(some_string: String) {    // 所有权转移到参数中
    println!("{}", some_string);
}

fn make_copy(some_number: i32) {            // 基本类型做了copy
    println!("{}", some_number);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {   // 变量使用后又返回
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}