
fn main() {
    another_function(5);    // argument 实参

    let x = 5;
    let y = {
        let x = 1;
        x + 3
    };

    println!("The value of y is {}", y);
    let x = five();
    println!("{}", x);

    condition();

    loop_test();

    while_test();

    iter_test();

    range_test();
}

fn another_function(x: i32) {
    println!("The value is {}", x); // parameter 形参
}

fn five() -> i32 {
    return 5;
}

fn condition() {    // 条件
    let number = 6;

    // 多个if else 可以使用match重构
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    }

    let number2 = if true {5} else {6};
}

fn loop_test() {    // 测试循环loop
    let mut counter = 0;

    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is: {}", result);
}

fn while_test() {
    let mut number = 10;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("END")
}

fn iter_test() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn range_test() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("END");
}

