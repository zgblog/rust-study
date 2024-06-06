use std::fmt::Display;


fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);

    struct_test();
}

/**
 * 'a为生命周期标注，被'a所替代的具体生命周期是x和y作用域的相重叠的那一部分，即x和y生命周期较小的那一个；返回的引用值保证在x和y中较短的那个生命周期结束之前有效
 */
fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 生命周期语法是将函数的多个参数与其返回值的生命周期进行关联

/**
 * 结构体中的生命周期标注
 */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn struct_test() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt {part: first_sentence};
}

fn longest_with_an_announcement<'a, T> (x: &'a str, y: &'a str, ann: T) where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len {
        x
    } else {
        y
    }
}