use std::{thread, time::Duration};

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
    closure();
    closure_test2();
}

/**
 * 模拟执行，执行2s
 */
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    /**
     * 闭包，语法 |xxx| {}
     */
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher<T> 
    where T: Fn(u32) -> u32
{
     calculation: T,
     value: Option<u32>
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(value) => value,
            None => {
                // 缓存值
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn closure() {
    let x = 4;

    // 闭包比较于函数可以捕获外部的变量【借用】
    let equal_to_x = |z| z == x;
    // fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));
}

fn closure_test2() {
    let x = vec![1, 2, 3];
    // 强制获取所有权
    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));

}