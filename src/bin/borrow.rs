fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("Hello");
    let len = update(&mut s2);
    println!("{}", s2);
} 

fn calculate_length(s: &String) -> usize {  // 引用作为函数参数称为借用
    s.len()
}

fn update(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}