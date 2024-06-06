use std::fmt::Display;
use std::fmt::Debug;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());
    
}

/**
 * 定义一个公有trait
 */
pub trait Summary {
    fn summarize(&self) -> String;
}

trait Summary_2 {
    fn summarize(&self) -> String { // 默认实现
        String::from("to be continue")
    }
}

trait Summary_3 {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String { // 允许调用未默认实现的方法
        format!("to be {}", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/**
 * trait作为函数参数
 */
fn notify(item: impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

/**
 * Trait Bound语法
 */
fn notify_2<T: Summary>(item: T) {
    println!("Breaking news: {}", item.summarize());
}

/**
 * 两个trait使用 + 指定需要实现两个trait
 */
fn notify_3(item: impl Summary + Display) {
    println!("Breaking news: {}", item.summarize());
}

fn notify_4<T: Summary + Display>(item: T) {
    println!("Breaking news: {}", item.summarize());
}

/**
 * where简化trait bound
 */
fn notify_5<T, U>(t: T, u: U) where T: Display + Clone, U: Clone + Debug {

}

/**
 * 返回trait类型
 */
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

