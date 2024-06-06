fn main() {
    create_test();
    update_test();
    index_test();
}

fn create_test() {
    // 1. 创建一个空的，使用String::new
    let mut s = String::new();

    // 2. 从str创建
    let data = "initial contents";
    let s = data.to_string();
    // 字面量形式
    let s = "initial contents".to_string();

    // 3. 使用String::from创建
    let s = String::from("initial contents");
    // 各种语言
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

}

/**
 * 更新字符串
 */
fn update_test() {
    let mut s = String::from("foo");
    // 1. 使用push_str更新
    s.push_str("bar");
    let s2 = "...";
    s.push_str(s2);
    println!("{}", s);

    // 2. 使用+
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;  // 注意s1被move了，不可用了，+运算符实际使用了add函数 fn add(self, s: &str) -> String {
    println!("{}", s3);

    // 3. 使用宏 format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

}

/**
 * 索引测试
 */
fn index_test() {
    // 因为字符的字节占用不确定，所有没有直接索引的功能
    // 遍历
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}