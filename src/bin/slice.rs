fn main() {
    let mut s = String::from("Hello world!");
    let word = first_world(&s);

    // s.clear();

    println!("{}", word);

    let hello = &s[..5];
    let world = &s[6..];

    let whold = &s[..];
}

fn first_world(s: &String) -> &str {    // 返回的字符串切片
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_world2(s: &str) -> &str {  // 字符串切片作为函数参数
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}