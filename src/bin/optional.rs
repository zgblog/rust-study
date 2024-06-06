fn main() {
    test();
}

fn test() {
    let some_number = Some(5);
    let some_string = Some("A String");

    let absend_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{}", six.unwrap());

    test_2();
    test_3();
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match必須匹配所有变体
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}

fn test_2() {
    let v = 0u8;
    match v {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("....")   // 通配符
    }
}

fn test_3() {
    let v = Some(8);

    // if let语法，只匹配一个
    if let Some(3) = v {
        println!("three")
    } else {
        println!("others");
    }
}