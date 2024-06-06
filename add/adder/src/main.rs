use add_one;
use add_two;

/**
 * 工作空间依赖
 */
fn main() {
    let num = 10;
    println!("Hello world! {} plush one is {}", num, add_one::add_one(num));
    let (a, b) = (5, 6);
    println!("add-two {} plus {} is {}", a, b, add_two::add_two(a, b));
}
