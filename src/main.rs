use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;    // as 关键字，起别名
use std::{cmp::Ordering, io::Empty};   // {,}
use std::io::{self, Write}; // {自身 , }
use std::collections::*;    // 通配符

// fn f1() -> Result<>

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("Hello, world!");
}
