use std::{cmp::Ordering, io};    // prelude
use rand::Rng;  // trait

fn main() {
    println!("猜数");

    let secret_number = rand::thread_rng().gen_range(0..100);   // i32 u32 i64

    // println!("神秘数字是：{}", secret_number);

    loop {
        println!("猜测一个数");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("无法读取行");
    
        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的数是：{}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
            Ordering::Greater => println!("Too big")
        }
    }
}
